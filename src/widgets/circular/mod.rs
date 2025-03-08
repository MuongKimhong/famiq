pub mod components;
pub mod helper;
pub mod tests;

use bevy::prelude::*;
use crate::widgets::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::event_writer::FaMouseEvent;
use crate::utils::*;

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

/// Represents a Famiq circular UI element, such as a spinner or loading indicator.
pub struct FaCircular;

// Needs container
impl<'a> FaCircular {
    fn _build_circular(attributes: &WidgetAttributes, root_node: &'a mut EntityCommands) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();

        let color = get_color(&attributes.color);
        let outer_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                DefaultWidgetEntity::from(style_components),
                IsFamiqCircular,
                SpinnerColor(color)
            ))
            .observe(FaCircular::handle_on_mouse_up)
            .observe(FaCircular::handle_on_mouse_down)
            .observe(FaCircular::handle_on_mouse_over)
            .observe(FaCircular::handle_on_mouse_out)
            .id();

        insert_id_and_class(root_node, outer_entity, &attributes.id, &attributes.class);
        outer_entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        has_tooltip: bool,
        tooltip_text: Option<String>
    ) -> Entity {
        let circular = Self::_build_circular(attributes, root_node);

        if has_tooltip {
            let tooltip = build_tooltip_node(
                &tooltip_text.unwrap(),
                attributes.font_handle.clone().unwrap(),
                root_node
            );
            root_node.commands().entity(circular).insert(FamiqTooltipEntity(tooltip));
            root_node.commands().entity(circular).add_child(tooltip);
        }
        circular
    }

    fn handle_on_mouse_over(
        mut over: Trigger<Pointer<Over>>,
        mut circular_q: Query<(&GlobalTransform, Option<&FamiqTooltipEntity>, Option<&FamiqWidgetId>), With<IsFamiqCircular>>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok((transform, tooltip_entity, id)) = circular_q.get_mut(over.entity()) {
            show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
            FaMouseEvent::send_over_event(&mut writer, WidgetType::Circular, over.entity(), id);
        }
        over.propagate(false);
    }

    fn handle_on_mouse_out(
        mut out: Trigger<Pointer<Out>>,
        mut circular_q: Query<(Option<&FamiqTooltipEntity>, Option<&FamiqWidgetId>), With<IsFamiqCircular>>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok((tooltip_entity, id)) = circular_q.get_mut(out.entity()) {
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            FaMouseEvent::send_out_event(&mut writer, WidgetType::Circular, out.entity(), id);
        }
        out.propagate(false);
    }

    fn handle_on_mouse_down(
        mut down: Trigger<Pointer<Down>>,
        mut circular_q: Query<Option<&FamiqWidgetId>, With<IsFamiqCircular>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok(id) = circular_q.get_mut(down.entity()) {
            if down.event().button == PointerButton::Secondary {
                FaMouseEvent::send_down_event(&mut writer, WidgetType::Circular, down.entity(), id, true);
            } else {
                FaMouseEvent::send_down_event(&mut writer, WidgetType::Circular, down.entity(), id, false);
            }
        }
        down.propagate(false);
    }

    fn handle_on_mouse_up(
        mut up: Trigger<Pointer<Up>>,
        mut circular_q: Query<Option<&FamiqWidgetId>, With<IsFamiqCircular>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok(id) = circular_q.get_mut(up.entity()) {
            FaMouseEvent::send_up_event(&mut writer, WidgetType::Circular, up.entity(), id);
        }
        up.propagate(false);
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
    pub attributes: WidgetAttributes,
    pub root_node: EntityCommands<'a>,
    pub has_tooltip: bool,
    pub tooltip_text: String,
}

impl<'a> FaCircularBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>, font_handle: Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            root_node,
            has_tooltip: false,
            tooltip_text: String::new()
        }
    }

    /// Method to add tooltip to circular.
    pub fn tooltip(mut self, text: &str) -> Self {
        self.has_tooltip = true;
        self.tooltip_text = text.to_string();
        self
    }

    /// Spawn circular to UI world
    pub fn build(&mut self) -> Entity {
        self._process_built_in_color_class();
        self._process_built_in_size_class();
        self._node();
        FaCircular::new(
            &self.attributes,
            &mut self.root_node,
            self.has_tooltip,
            Some(self.tooltip_text.clone())
        )
    }
}

impl<'a> SetWidgetAttributes for FaCircularBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_circular_node(&self.attributes.size);

        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }

        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaCircularBuilder`
pub fn fa_circular<'a>(builder: &'a mut FamiqBuilder) -> FaCircularBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaCircularBuilder::new(builder.ui_root_node.reborrow(), font_handle)
}

/// Determines if circular internal system(s) can run.
///
/// True only if circular widget is created.
pub fn can_run_circular_systems(circular_q: Query<&IsFamiqCircular>) -> bool {
    !circular_q.is_empty()
}
