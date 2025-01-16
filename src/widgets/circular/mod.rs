pub mod helper;

use bevy::prelude::*;
use crate::widgets::{FamiqWidgetId, FamiqWidgetClasses, DefaultWidgetEntity};
use crate::utils::{entity_add_child, lighten_color, darken_color};
use helper::*;

#[derive(Component)]
pub struct IsFamiqCircular;

#[derive(Component)]
pub struct IsFamiqCircularSpinner;

#[derive(Component)]
pub struct RotatingSequence {
    speed: f32,        // Current rotation speed in degrees per second
    timer: Timer,
    speed_sequence: Vec<f32>, // Sequence of speeds
    current_index: usize,     // Current index in the sequence
}

#[derive(Component)]
pub struct CircularSpinnerEntity(pub Entity);

pub enum CircularSize {
    Small,
    Normal,
    Large
}

pub enum CircularColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

pub struct FaCircular;

// Needs container
impl<'a> FaCircular {
    fn _build_spinner(
        root_node: &'a mut EntityCommands,
        color: &CircularColor,
        size: &CircularSize
    ) -> Entity {
        let node = default_spinner_node(size);
        let border_radius = BorderRadius::all(Val::Percent(50.0));
        let bg_color = BackgroundColor(Color::NONE);
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;


        // let border_color = BorderColor(Color::srgba(0.878, 0.878, 0.878, 0.941));
        let border_color = get_spinner_color(color);

        let entity = root_node
            .commands()
            .spawn((
                node,
                border_color,
                border_radius,
                bg_color,
                z_index,
                visibility,
                IsFamiqCircularSpinner,
                RotatingSequence {
                    speed: 160.0,
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating), // every 1 secs
                    speed_sequence: vec![200.0, 400.0, 200.0], // Sequence of speeds
                    current_index: 0,
                }
            ))
            .id();

        entity
    }

    fn _build_outer_circle(
        id: &str,
        classes: &str,
        root_node: &'a mut EntityCommands,
        color: &CircularColor,
        size: &CircularSize,
        spinner_entity: Entity
    ) -> Entity {
        let node = default_outer_circle_node(size);
        let border_radius = BorderRadius::all(Val::Percent(50.0));
        let bg_color = BackgroundColor(Color::NONE);
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        let lightening_percentage = match color {
            CircularColor::Primary => 70.0,
            CircularColor::Warning => 70.0,
            CircularColor::Danger => 45.0,
            CircularColor::Success => 80.0,
            CircularColor::Secondary => 45.0,
            CircularColor::Info => 85.0,
            _ => -25.0, // Use negative value for darken
        };

        let base_color = get_spinner_color(color).0;
        let use_border_color = if lightening_percentage >= 0.0 {
            lighten_color(lightening_percentage, &base_color)
        } else {
            darken_color(-lightening_percentage, &base_color)
        }.unwrap();

        let border_color = BorderColor(use_border_color);


        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(id.to_string()),
                FamiqWidgetClasses(classes.to_string()),
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility
                ),
                IsFamiqCircular,
                CircularSpinnerEntity(spinner_entity),
                Interaction::default(),

            ))
            .id()
    }

    pub fn new(
        id: &str,
        classes: &str,
        root_node: &'a mut EntityCommands,
        color: CircularColor,
        size: CircularSize,
    ) -> Entity {
        let spinner = Self::_build_spinner(root_node, &color, &size);
        let outer = Self::_build_outer_circle(
            id,
            classes,
            root_node,
            &color,
            &size,
            spinner
        );

        entity_add_child(root_node, spinner, outer);
        outer
    }

    pub fn rotate_spinner(
        time: Res<Time>,
        mut query: Query<(&mut Transform, &RotatingSequence)>,
    ) {
        for (mut transform, rotating) in query.iter_mut() {
            let speed_radians = rotating.speed.to_radians();

            // Update rotation
            transform.rotation = transform.rotation * Quat::from_rotation_z(
                speed_radians * time.delta_secs()
            );
        }
    }

    pub fn update_spinner_speed(
        time: Res<Time>,
        mut query: Query<&mut RotatingSequence>,
    ) {
        for mut rotating in query.iter_mut() {
            // Update the timer
            rotating.timer.tick(time.delta());

            if rotating.timer.just_finished() {
                // Move to the next speed in the sequence
                rotating.current_index = (rotating.current_index + 1) % rotating.speed_sequence.len();
                rotating.speed = rotating.speed_sequence[rotating.current_index];
            }
        }
    }
}
