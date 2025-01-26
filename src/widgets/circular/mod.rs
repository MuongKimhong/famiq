pub mod components;
pub mod helper;
pub mod tests;

use bevy::prelude::*;
use crate::widgets::{
    FamiqWidgetId, FamiqWidgetClasses, FamiqWidgetResource,
    DefaultWidgetEntity, FamiqWidgetBuilder, WidgetStyle,
    ExternalStyleHasChanged
};
use crate::utils::{entity_add_child, lighten_color, darken_color};

pub use components::*;
use helper::*;

/// Represents built-in size of a circular UI element.
pub enum CircularSize {
    Small,
    Normal,
    Large,
    CustomSize(f32)
}

/// Represents built-in color options for a circular UI element.
pub enum CircularColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

/// Represents a Famiq circular UI element, such as a spinner or loading indicator.
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
                    speed: 250.0,
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating), // every 1 secs
                    speed_sequence: vec![250.0, 450.0, 250.0], // Sequence of speeds
                    current_index: 0,
                }
            ))
            .id();

        entity
    }

    fn _build_outer_circle(
        id: Option<String>,
        class: Option<String>,
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

        let outer_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
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
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)

            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(outer_entity).insert(FamiqWidgetId(id.to_string()));
        }
        if let Some(class) = class {
            root_node.commands().entity(outer_entity).insert(FamiqWidgetClasses(class));
        }
        outer_entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        color: CircularColor,
        size: CircularSize,
    ) -> Entity {
        let spinner = Self::_build_spinner(root_node, &color, &size);
        let outer = Self::_build_outer_circle(
            id,
            class,
            root_node,
            &color,
            &size,
            spinner
        );

        entity_add_child(root_node, spinner, outer);
        outer
    }

    /// System to rotate spinner entities based on their rotation speed.
    ///
    /// # Parameters
    /// - `time`: Resource containing the time delta.
    /// - `query`: Query for spinner entities and their `RotatingSequence` components.
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

    /// System to update spinner rotation speeds based on a predefined sequence.
    ///
    /// # Parameters
    /// - `time`: Resource containing the time delta.
    /// - `query`: Query for spinner entities and their `RotatingSequence` components.
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

/// Builder for creating Famiq circular elements.
pub struct FaCircularBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub size: Option<f32>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaCircularBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            size: None,
            root_node
        }
    }

    fn _process_built_in_classes(&self) -> (CircularColor, Option<CircularSize>) {
        let mut use_color = CircularColor::Default;
        let mut use_size = None;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-primary" => use_color = CircularColor::Primary,
                    "is-secondary" => use_color = CircularColor::Secondary,
                    "is-danger" => use_color = CircularColor::Danger,
                    "is-success" => use_color = CircularColor::Success,
                    "is-warning" => use_color = CircularColor::Warning,
                    "is-info" => use_color = CircularColor::Info,

                    "is-small" => use_size = Some(CircularSize::Small),
                    "is-large" => use_size = Some(CircularSize::Large),

                    _ => ()
                }
            }
        }
        (use_color, use_size)
    }

    fn _process_custom_size(&self) -> CircularSize {
        let mut use_size = CircularSize::Normal;

        if let Some(size) = self.size.as_ref() {
            if *size > 0.0 {
                use_size = CircularSize::CustomSize(*size);
            }
        }
        use_size
    }

    /// Method to set circular's custom size.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Method to add class to circular.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to circular
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Spawn circular to UI world
    pub fn build(&mut self) -> Entity {
        let (color, size) = self._process_built_in_classes();
        let use_size = size.unwrap_or_else(|| self._process_custom_size() );
        FaCircular::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_node,
            color,
            use_size
        )
    }
}

/// API to create `FaCircularBuilder`
pub fn fa_circular<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaCircularBuilder<'a> {
    builder.resource.can_run_systems.circular = true;
    FaCircularBuilder::new(builder.ui_root_node.reborrow())
}

/// Determines if circular internal system(s) can run.
///
/// True only if circular widget is created.
pub fn can_run_circular_systems(builder_res: Res<FamiqWidgetResource>) -> bool {
    builder_res.can_run_systems.circular
}
