pub mod components;
pub mod helper;
pub mod tests;

use bevy::prelude::*;
use crate::widgets::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::widgets::container::base_container::*;
use macros::*;
use crate::event_writer::*;
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

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct CircularBuilder {
    pub size: RVal
}

impl CircularBuilder {
    pub fn new() -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            size: RVal::FNum(0.0)
        }
    }

    fn on_mouse_over(
        mut over: Trigger<Pointer<Over>>,
        circular_q: Query<(&GlobalTransform, Option<&TooltipEntity>, Option<&WidgetId>), With<IsFamiqCircular>>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok((transform, tooltip_entity, id)) = circular_q.get(over.entity()) {
            show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Circular, over.entity(), id);
        }
        over.propagate(false);
    }

    fn on_mouse_out(
        mut out: Trigger<Pointer<Out>>,
        mut circular_q: Query<(Option<&TooltipEntity>, Option<&WidgetId>), With<IsFamiqCircular>>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok((tooltip_entity, id)) = circular_q.get_mut(out.entity()) {
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Circular, out.entity(), id);
        }
        out.propagate(false);
    }

    fn on_mouse_down(
        mut down: Trigger<Pointer<Down>>,
        mut circular_q: Query<Option<&WidgetId>, With<IsFamiqCircular>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok(id) = circular_q.get_mut(down.entity()) {
            if down.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Circular, down.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Circular, down.entity(), id);
            }
        }
        down.propagate(false);
    }

    fn on_mouse_up(
        mut up: Trigger<Pointer<Up>>,
        mut circular_q: Query<Option<&WidgetId>, With<IsFamiqCircular>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok(id) = circular_q.get_mut(up.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Circular, up.entity(), id);
        }
        up.propagate(false);
    }

    /// Internal system to detect new circular bing created.
    pub fn detect_new_circular(
        mut commands: Commands,
        mut circular_material: ResMut<Assets<CircularMaterial>>,
        circular_q: Query<(Entity, &SpinnerColor), Added<IsFamiqCircular>>,
    ) {
        circular_q.iter().for_each(|(entity, color)| {
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
        });
    }

    pub fn update_circular_material_u_time(
        time: Res<Time>,
        mut materials: ResMut<Assets<CircularMaterial>>,
        query: Query<&MaterialNode<CircularMaterial>>
    ) {
        query.iter().for_each(|handle| {
            if let Some(material) = materials.get_mut(handle) {
                material.u_time = -time.elapsed_secs();
            }
        });
    }
}

impl SetupWidget for CircularBuilder {
    fn components(&mut self) -> impl Bundle {
        let color = get_color(&self.cloned_attrs.color);
        (IsFamiqCircular, MainWidget, SpinnerColor(color), ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();

        match self.size.to_owned() {
            RVal::FNum(v) => {
                if v > 0.0 {
                    self.cloned_attrs.size = WidgetSize::Custom(v);
                }
            }
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);

                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::FNum(size) => self.cloned_attrs.size = WidgetSize::Custom(*size),
                            _ => {}
                        }
                    }
                }
                all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }

        self._process_built_in_size_class();
        self.cloned_attrs.node = default_circular_node(&self.cloned_attrs.size);
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut circular = FaBaseContainer::new_with_attributes(&self.cloned_attrs);

        let circular_entity = circular.build(r_data, commands);

        commands
            .entity(circular_entity)
            .insert(self.components())
            .observe(CircularBuilder::on_mouse_up)
            .observe(CircularBuilder::on_mouse_down)
            .observe(CircularBuilder::on_mouse_over)
            .observe(CircularBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, circular_entity);
        }
        insert_class_id(commands, circular_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        let cloned_builder = self.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                all_reactive_keys,
                circular_entity,
                WidgetBuilder {
                    builder: BuilderType::Circular(cloned_builder)
                }
            ));
        });
        circular_entity
    }

    fn build_with_world(&mut self, r_data: &HashMap<String, RVal>, world: &mut World) -> Option<Entity> {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();

        match self.size.to_owned() {
            RVal::FNum(v) => {
                if v > 0.0 {
                    self.cloned_attrs.size = WidgetSize::Custom(v);
                }
            }
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);

                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::FNum(size) => self.cloned_attrs.size = WidgetSize::Custom(*size),
                            _ => {}
                        }
                    }
                }
                all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }

        self._process_built_in_size_class();

        self.cloned_attrs.node = default_circular_node(&self.cloned_attrs.size);
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut circular = FaBaseContainer::new_with_attributes(&self.cloned_attrs);

        let circular_entity = circular.build_with_world(r_data, world);

        world
            .entity_mut(circular_entity.unwrap())
            .insert(self.components())
            .observe(CircularBuilder::on_mouse_up)
            .observe(CircularBuilder::on_mouse_down)
            .observe(CircularBuilder::on_mouse_over)
            .observe(CircularBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, &mut world.commands(), circular_entity.unwrap());
        }
        insert_class_id_world(world, circular_entity.unwrap(), &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            all_reactive_keys,
            circular_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::Circular(cloned_builder)
            }
        ));
        circular_entity
    }
}

#[macro_export]
macro_rules! circular {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let c_builder = &mut CircularBuilder::new();
        $(
            $crate::circular_attributes!(c_builder, $key : $value);
        )*
        c_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! circular_attributes {
    ($c_builder:ident, size: $size:expr) => {{
        match to_rval($size) {
            Ok(v) => $c_builder.size = v,
            Err(_) => panic!("\nsize attribute accepts only f32 and reactive string\n")
        }
    }};
    ($c_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($c_builder, $key : $value);
    }};
}

// /// Represents a Famiq circular UI element, such as a spinner or loading indicator.
// pub struct FaCircular;

// // Needs container
// impl<'a> FaCircular {
//     fn _build_circular(attributes: &WidgetAttributes, root_node: &'a mut EntityCommands) -> Entity {
//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = attributes.node.clone();

//         let color = get_color(&attributes.color);
//         let outer_entity = root_node
//             .commands()
//             .spawn((
//                 style_components.clone(),
//                 DefaultWidgetConfig::from(style_components),
//                 IsFamiqCircular,
//                 MainWidget,
//                 SpinnerColor(color)
//             ))
//             .observe(FaCircular::handle_on_mouse_up)
//             .observe(FaCircular::handle_on_mouse_down)
//             .observe(FaCircular::handle_on_mouse_over)
//             .observe(FaCircular::handle_on_mouse_out)
//             .id();

//         insert_id_and_class(root_node, outer_entity, &attributes.id, &attributes.class);
//         outer_entity
//     }

//     pub fn new(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands
//     ) -> Entity {
//         let circular = Self::_build_circular(attributes, root_node);

//         if attributes.has_tooltip {
//             build_tooltip_node(attributes, root_node, circular);
//         }
//         circular
//     }

//     fn handle_on_mouse_over(
//         mut over: Trigger<Pointer<Over>>,
//         circular_q: Query<(&GlobalTransform, Option<&TooltipEntity>, Option<&WidgetId>), With<IsFamiqCircular>>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         mut writer: EventWriter<FaMouseEvent>
//     ) {
//         if let Ok((transform, tooltip_entity, id)) = circular_q.get(over.entity()) {
//             show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
//             FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Circular, over.entity(), id);
//         }
//         over.propagate(false);
//     }

//     fn handle_on_mouse_out(
//         mut out: Trigger<Pointer<Out>>,
//         mut circular_q: Query<(Option<&TooltipEntity>, Option<&WidgetId>), With<IsFamiqCircular>>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         mut writer: EventWriter<FaMouseEvent>
//     ) {
//         if let Ok((tooltip_entity, id)) = circular_q.get_mut(out.entity()) {
//             hide_tooltip(tooltip_entity, &mut tooltip_q);
//             FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Circular, out.entity(), id);
//         }
//         out.propagate(false);
//     }

//     fn handle_on_mouse_down(
//         mut down: Trigger<Pointer<Down>>,
//         mut circular_q: Query<Option<&WidgetId>, With<IsFamiqCircular>>,
//         mut writer: EventWriter<FaMouseEvent>
//     ) {
//         if let Ok(id) = circular_q.get_mut(down.entity()) {
//             if down.event().button == PointerButton::Secondary {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Circular, down.entity(), id);
//             } else {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Circular, down.entity(), id);
//             }
//         }
//         down.propagate(false);
//     }

//     fn handle_on_mouse_up(
//         mut up: Trigger<Pointer<Up>>,
//         mut circular_q: Query<Option<&WidgetId>, With<IsFamiqCircular>>,
//         mut writer: EventWriter<FaMouseEvent>
//     ) {
//         if let Ok(id) = circular_q.get_mut(up.entity()) {
//             FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Circular, up.entity(), id);
//         }
//         up.propagate(false);
//     }

//     /// Internal system to detect new circular bing created.
//     pub fn detect_new_circular_widget_system(
//         mut commands: Commands,
//         mut circular_material: ResMut<Assets<CircularMaterial>>,
//         circular_q: Query<(Entity, &SpinnerColor), Added<IsFamiqCircular>>,
//     ) {
//         for (entity, color) in circular_q.iter() {
//             if let Color::Srgba(value) = color.0 {
//                 commands
//                     .entity(entity)
//                     .insert(
//                         MaterialNode(circular_material.add(CircularMaterial {
//                             u_time: 0.0,
//                             u_color: Vec3::new(value.red, value.green, value.blue)
//                         }))
//                     );
//             }
//         }
//     }

//     pub fn _update_circular_material_u_time(
//         time: Res<Time>,
//         mut materials: ResMut<Assets<CircularMaterial>>,
//         query: Query<&MaterialNode<CircularMaterial>>
//     ) {
//         for handle in &query {
//             if let Some(material) = materials.get_mut(handle) {
//                 material.u_time = -time.elapsed_secs();
//             }
//         }
//     }
// }

// /// Builder for creating Famiq circular widget.
// pub struct FaCircularBuilder<'a> {
//     pub attributes: WidgetAttributes,
//     pub root_node: EntityCommands<'a>
// }

// impl<'a> FaCircularBuilder<'a> {
//     pub fn new(root_node: EntityCommands<'a>, font_handle: Handle<Font>) -> Self {
//         let mut attributes = WidgetAttributes::default();
//         attributes.font_handle = Some(font_handle);
//         Self {
//             attributes,
//             root_node
//         }
//     }

//     /// Spawn circular to UI world
//     pub fn build(&mut self) -> Entity {
//         self._process_built_in_color_class();
//         self._process_built_in_size_class();
//         self._node();
//         FaCircular::new(
//             &self.attributes,
//             &mut self.root_node
//         )
//     }
// }

// impl<'a> SetWidgetAttributes for FaCircularBuilder<'a> {
//     fn attributes(&mut self) -> &mut WidgetAttributes {
//         &mut self.attributes
//     }

//     fn _node(&mut self) {
//         self.attributes.node = default_circular_node(&self.attributes.size);

//         if self.attributes.default_display_changed {
//             self.attributes.node.display = self.attributes.default_display;
//         }

//         process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
//     }
// }

// /// API to create `FaCircularBuilder`
// pub fn fa_circular_builder<'a>(builder: &'a mut FamiqBuilder) -> FaCircularBuilder<'a> {
//     let font_handle = builder.asset_server.load(&builder.resource.font_path);
//     FaCircularBuilder::new(builder.ui_root_node.reborrow(), font_handle)
// }

// #[macro_export]
// macro_rules! fa_circular {
//     ( $( $key:ident : $value:tt ),* $(,)? ) => {{
//         let builder = builder_mut();
//         let mut circular = fa_circular_builder(builder);
//         $(
//             $crate::fa_circular_attributes!(circular, $key : $value);
//         )*
//         circular.build()
//     }};
// }

// #[macro_export]
// macro_rules! fa_circular_attributes {
//     ($circular:ident, color: $color:expr) => {{
//         $circular = $circular.color($color);
//     }};

//     ($circular:ident, tooltip: $tooltip:expr) => {{
//         $circular = $circular.tooltip($tooltip);
//     }};

//     ($circular:ident, size: $size:expr) => {{
//         $circular = $circular.size($size);
//     }};

//     // common attributes
//     ($circular:ident, $key:ident : $value:expr) => {{
//         $crate::common_attributes!($circular, $key : $value);
//     }};
// }

/// Determines if circular internal system(s) can run.
///
/// True only if circular widget is created.
pub fn can_run_circular_systems(circular_q: Query<&IsFamiqCircular>) -> bool {
    !circular_q.is_empty()
}
