pub mod components;
pub mod helper;
pub mod tests;

use bevy::prelude::*;
use crate::widgets::{
    DefaultWidgetEntity, FamiqBuilder, FamiqToolTipText, BaseStyleComponents
};
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::event_writer::FaInteractionEvent;
use crate::utils::{
    process_spacing_built_in_class,
    insert_id_and_class,
    get_embedded_asset_path
};
use crate::widgets::color::built_in_color_parser;
use super::tooltip::{FaToolTip, FaToolTipResource, IsFamiqToolTipText};

pub use components::*;
use helper::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CircularMaterial {
    #[uniform(0)]
    u_color: Vec3,
    #[uniform(1)]
    u_time: f32
}

impl UiMaterial for CircularMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/circular.wgsl").into()
    }
}

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
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark
}

/// Represents a Famiq circular UI element, such as a spinner or loading indicator.
pub struct FaCircular;

// Needs container
impl<'a> FaCircular {
    fn _build_circular(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        color: Color,
        size: &CircularSize,
    ) -> Entity {
        let mut node = default_circular_node(size);
        process_spacing_built_in_class(&mut node, &class);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = node;

        let outer_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                DefaultWidgetEntity::from(style_components),
                IsFamiqCircular,
                SpinnerColor(color)
            ))
            .id();

        insert_id_and_class(root_node, outer_entity, &id, &class);
        outer_entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        color: Color,
        size: CircularSize,
        has_tooltip: bool,
        tooltip_text: Option<String>
    ) -> Entity {
        let outer = Self::_build_circular(
            id,
            class,
            root_node,
            color,
            &size,
        );

        if has_tooltip {
            root_node.commands().entity(outer).insert(FamiqToolTipText(tooltip_text.unwrap()));
        }
        outer
    }

    /// Internal system to handle circular interaction events.
    pub fn handle_circular_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut circular_q: Query<
            (&ComputedNode, &GlobalTransform, Option<&FamiqToolTipText>),
            With<IsFamiqCircular>
        >,
        mut tooltip_res: ResMut<FaToolTipResource>,
        mut tooltip_text_q: Query<&mut Text, With<IsFamiqToolTipText>>
    ) {
        for e in events.read() {
            if let Ok((computed, transform, tooltip_text)) = circular_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        if let Some(text) = tooltip_text {
                            FaToolTip::_update_toolitp_text(&text.0, &mut tooltip_text_q);
                            tooltip_res.show(text.0.clone(), computed.size(), transform.translation());
                        }
                    },
                    Interaction::None => {
                        if tooltip_text.is_some() {
                            tooltip_res.hide();
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    /// Internal system to detect new circular bing created.
    pub fn detect_new_circular_widget_system(
        mut commands: Commands,
        mut circular_material: ResMut<Assets<CircularMaterial>>,
        circular_q: Query<(Entity, &SpinnerColor), Added<IsFamiqCircular>>,
    ) {
        for (entity, color) in circular_q.iter() {
            if let Color::Srgba(value) = color.0 {
                commands
                    .entity(entity)
                    .insert(
                        MaterialNode(circular_material.add(CircularMaterial {
                            u_time: 0.0,
                            u_color: Vec3::new(value.red, value.green, value.blue)
                        }))
                    );
            }
        }
    }

    pub fn _update_circular_material_u_time(
        time: Res<Time>,
        mut materials: ResMut<Assets<CircularMaterial>>,
        query: Query<&MaterialNode<CircularMaterial>>
    ) {
        for handle in &query {
            if let Some(material) = materials.get_mut(handle) {
                material.u_time = -time.elapsed_secs();
            }
        }
    }
}

/// Builder for creating Famiq circular widget.
pub struct FaCircularBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub size: Option<f32>,
    pub root_node: EntityCommands<'a>,
    pub has_tooltip: bool,
    pub tooltip_text: String,
    pub color: Option<Color>
}

impl<'a> FaCircularBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            size: None,
            root_node,
            has_tooltip: false,
            tooltip_text: String::new(),
            color: None
        }
    }

    fn _process_built_in_classes(&mut self) -> Option<CircularSize> {
        let mut use_color = CircularColor::Default;
        let mut use_size = None;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-primary" => use_color = CircularColor::Primary,
                    "is-primary-dark" => use_color = CircularColor::PrimaryDark,
                    "is-secondary" => use_color = CircularColor::Secondary,
                    "is-danger" => use_color = CircularColor::Danger,
                    "is-danger-dark" => use_color = CircularColor::DangerDark,
                    "is-success" => use_color = CircularColor::Success,
                    "is-success-dark" => use_color = CircularColor::SuccessDark,
                    "is-warning" => use_color = CircularColor::Warning,
                    "is-warning-dark" => use_color = CircularColor::WarningDark,
                    "is-info" => use_color = CircularColor::Info,
                    "is-info-dark" => use_color = CircularColor::InfoDark,

                    "is-small" => use_size = Some(CircularSize::Small),
                    "is-large" => use_size = Some(CircularSize::Large),

                    _ => ()
                }
            }
        }

        if self.color.is_none() {
            self.color = Some(get_circular_color(&use_color));
        }
        use_size
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

    /// Method to add id to circular.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Method to add tooltip to circular.
    pub fn tooltip(mut self, text: &str) -> Self {
        self.has_tooltip = true;
        self.tooltip_text = text.to_string();
        self
    }

    /// Method to set color.
    pub fn set_color(mut self, color: &str) -> Self {
        self.color = built_in_color_parser(color);
        self
    }

    /// Spawn circular to UI world
    pub fn build(&mut self) -> Entity {
        let size = self._process_built_in_classes();
        let use_size = size.unwrap_or_else(|| self._process_custom_size() );
        FaCircular::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_node,
            self.color.unwrap(),
            use_size,
            self.has_tooltip,
            Some(self.tooltip_text.clone())
        )
    }
}

/// API to create `FaCircularBuilder`
pub fn fa_circular<'a>(builder: &'a mut FamiqBuilder) -> FaCircularBuilder<'a> {
    FaCircularBuilder::new(builder.ui_root_node.reborrow())
}

/// Determines if circular internal system(s) can run.
///
/// True only if circular widget is created.
pub fn can_run_circular_systems(circular_q: Query<&IsFamiqCircular>) -> bool {
    !circular_q.is_empty()
}
