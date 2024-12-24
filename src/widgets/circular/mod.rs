pub mod helper;

use bevy::prelude::*;
use crate::widgets::{FamiqWidgetId, DefaultWidgetEntity};
use crate::utils;
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

pub enum CircularVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

pub struct FaCircular;

impl<'a> FaCircular {
    fn _build_inner_circle(root_node: &'a mut EntityCommands, size: &CircularSize) -> Entity {
        let node = default_inner_circle_node(size);
        let border_color = BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.55));
        let border_radius = BorderRadius::all(Val::Percent(45.0));
        let bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55));
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        root_node
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
                    speed_sequence: vec![160.0, 320.0, 160.0], // Sequence of speeds
                    current_index: 0,
                },
            ))
            .id()
    }

    fn _build_outer_circle(
        id: &str,
        root_node: &'a mut EntityCommands,
        variant: &CircularVariant,
        size: &CircularSize,
        spinner_entity: Entity
    ) -> Entity {
        let node = default_outer_circle_node(size);
        let border_color = get_outer_circle_border_color(variant);
        let border_radius = BorderRadius::all(Val::Percent(50.0));
        let bg_color = BackgroundColor(Color::NONE);
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

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
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility
                ),
                IsFamiqCircular,
                CircularSpinnerEntity(spinner_entity)
            ))
            .id()
    }

    pub fn new(
        id: &str,
        root_node: &'a mut EntityCommands,
        variant: CircularVariant,
        size: CircularSize,
    ) -> Entity {
        let inner = Self::_build_inner_circle(root_node, &size);
        let outer = Self::_build_outer_circle(id, root_node, &variant, &size, inner);

        utils::entity_add_child(root_node, inner, outer);
        outer
    }

    pub fn rotate_node(
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

    pub fn update_speed(
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
