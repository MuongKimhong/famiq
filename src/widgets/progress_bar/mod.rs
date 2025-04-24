pub mod helper;
pub mod components;
pub mod systems;
pub mod tests;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use macros::set_widget_attributes;
use crate::widgets::container::base_container::*;
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::*;

pub use components::*;
pub use systems::*;
use helper::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    u_time: f32,
    #[uniform(1)]
    u_color: Vec3,
    #[uniform(2)]
    u_blend: f32,
    #[uniform(3)]
    u_size: Vec2
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/progress_bar.wgsl").into()
    }
}

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ProgressBarBuilder {
    pub size: RVal
}

impl ProgressBarBuilder {
    pub fn new() -> Self {
        Self {
            size: RVal::FNum(0.0),
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default()
        }
    }

    fn on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        bar_q: Query<(&GlobalTransform, Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqProgressBar>>
    ) {
        if let Ok((transform, id, tooltip_entity)) = bar_q.get(trigger.entity()) {
            show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::ProgressBar, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        bar_q: Query<(Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqProgressBar>>
    ) {
        if let Ok((id, tooltip_entity)) = bar_q.get(trigger.entity()) {
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::ProgressBar, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut writer: EventWriter<FaMouseEvent>,
        bar_q: Query<Option<&WidgetId>, With<IsFamiqProgressBar>>
    ) {
        if let Ok(id) = bar_q.get(trigger.entity()) {
            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::ProgressBar, trigger.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::ProgressBar, trigger.entity(), id);
            }
        }
        trigger.propagate(false);
    }

    fn on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut writer: EventWriter<FaMouseEvent>,
        bar_q: Query<Option<&WidgetId>, With<IsFamiqProgressBar>>
    ) {
        if let Ok(id) = bar_q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::ProgressBar, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    pub fn build_progress_value(
        &self,
        bar_entity: Entity,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut value = FaBaseContainer::new();
        value.cloned_attrs.node = default_progress_value_node(None);
        value.cloned_attrs.overrided_background_color = Some(Color::NONE);
        value.cloned_attrs.overrided_border_color = Some(Color::NONE);
        let value_entity = value.build(r_data, commands);

        commands
            .entity(value_entity)
            .insert((
                IsFamiqProgressValue,
                ProgressValueColor(get_color(&self.cloned_attrs.color)),
                ProgressBarEntity(bar_entity),
                ProgressValuePercentage(None)
            ));
        value_entity
    }

    pub fn build_progress_value_world(
        &self,
        bar_entity: Entity,
        world: &mut World,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut value = FaBaseContainer::new();
        value.cloned_attrs.node = default_progress_value_node(None);
        value.cloned_attrs.default_z_index = ZIndex(2);
        value.cloned_attrs.overrided_background_color = Some(Color::NONE);
        value.cloned_attrs.overrided_border_color = Some(Color::NONE);

        let value_entity = value.build_with_world(r_data, world);
        world
            .entity_mut(value_entity.unwrap())
            .insert((
                IsFamiqProgressValue,
                ProgressValueColor(get_color(&self.cloned_attrs.color)),
                ProgressBarEntity(bar_entity),
                ProgressValuePercentage(None)
            ));
        value_entity.unwrap()
    }
}

impl SetupWidget for ProgressBarBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqProgressBar, MainWidget, ReactiveWidget)
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
        self.cloned_attrs.node = default_progress_bar_node(&self.cloned_attrs.size);
        self.cloned_attrs.default_visibility = Visibility::Visible;
        self.cloned_attrs.overrided_border_color = Some(Color::srgba(0.6, 0.6, 0.6, 0.2));
        self.cloned_attrs.overrided_background_color = Some(Color::srgba(0.6, 0.6, 0.6, 0.2));
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut bar = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let bar_entity = bar.build(r_data, commands);
        let value_entity = self.build_progress_value(bar_entity, commands, r_data);

        commands
            .entity(bar_entity)
            .insert((self.components(), ProgressValueEntity(value_entity)))
            .add_child(value_entity)
            .observe(ProgressBarBuilder::on_mouse_up)
            .observe(ProgressBarBuilder::on_mouse_down)
            .observe(ProgressBarBuilder::on_mouse_over)
            .observe(ProgressBarBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, bar_entity);
        }
        insert_class_id(commands, bar_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model(commands, bar_entity, &self.cloned_attrs.model_key);
        let cloned_builder = self.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                all_reactive_keys,
                bar_entity,
                WidgetBuilder {
                    builder: BuilderType::ProgressBar(cloned_builder)
                }
            ));
        });
        bar_entity
    }

    fn build_with_world(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Option<Entity> {
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
        self.cloned_attrs.node = default_progress_bar_node(&self.cloned_attrs.size);
        self.cloned_attrs.default_visibility = Visibility::Visible;
        self.cloned_attrs.overrided_border_color = Some(Color::srgba(0.6, 0.6, 0.6, 0.2));
        self.cloned_attrs.overrided_background_color = Some(Color::srgba(0.6, 0.6, 0.6, 0.2));
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut bar = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let bar_entity = bar.build_with_world(r_data, world);
        let value_entity = self.build_progress_value_world(bar_entity.unwrap(), world, r_data);

        world
            .entity_mut(bar_entity.unwrap())
            .insert((self.components(), ProgressValueEntity(value_entity)))
            .add_child(value_entity)
            .observe(ProgressBarBuilder::on_mouse_up)
            .observe(ProgressBarBuilder::on_mouse_down)
            .observe(ProgressBarBuilder::on_mouse_over)
            .observe(ProgressBarBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, &mut world.commands(), bar_entity.unwrap());
        }
        insert_class_id_world(world, bar_entity.unwrap(), &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model_world(world, bar_entity.unwrap(), &self.cloned_attrs.model_key);
        let cloned_builder = self.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            all_reactive_keys,
            bar_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::ProgressBar(cloned_builder)
            }
        ));
        bar_entity
    }
}

#[macro_export]
macro_rules! progress_bar {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let p_builder = &mut ProgressBarBuilder::new();
        $(
            $crate::progress_bar_attributes!(p_builder, $key : $value);
        )*
        p_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! progress_bar_attributes {
    ($p_builder:ident, size: $size:expr) => {{
        match to_rval($size) {
            Ok(v) => $p_builder.size = v,
            Err(_) => panic!("\nsize attribute accepts only f32 and reactive string\n")
        }
    }};
    ($p_builder:ident, model: $model:expr) => {{
        $p_builder.set_model($model);
    }};
    ($p_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($p_builder, $key : $value);
    }};
}

// /// Represents the Famiq progress bar widget, which includes the bar and its value.
// pub struct FaProgressBar;

// impl<'a> FaProgressBar {
//     fn _build_progress_bar(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands,
//     ) -> Entity {
//         let color = Color::srgba(0.6, 0.6, 0.6, 0.2);

//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = attributes.node.clone();
//         style_components.visibility = Visibility::Visible;
//         style_components.background_color = BackgroundColor(color);
//         style_components.border_color = BorderColor(color);
//         style_components.border_radius = BorderRadius::all(Val::Px(5.0));

//         let entity = root_node
//             .commands()
//             .spawn((
//                 style_components.clone(),
//                 DefaultWidgetConfig::from(style_components),
//                 IsFamiqProgressBar,
//                 MainWidget
//             ))
//             .observe(FaProgressBar::handle_on_mouse_over)
//             .observe(FaProgressBar::handle_on_mouse_out)
//             .observe(FaProgressBar::handle_on_mouse_down)
//             .observe(FaProgressBar::handle_on_mouse_up)
//             .id();

//         insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
//         entity
//     }

//     fn _build_progress_value(
//         root_node: &'a mut EntityCommands,
//         attributes: &WidgetAttributes,
//         color: &WidgetColor,
//         bar_entity: Entity
//     ) -> Entity {
//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = default_progress_value_node(None);

//         let entity = root_node
//             .commands()
//             .spawn((
//                 style_components,
//                 IsFamiqProgressValue,
//                 FamiqProgressBarEntity(bar_entity),
//                 ProgressValueColor(get_color(color))
//             ))
//             .id();

//         if attributes.model_key.is_some() {
//             root_node.commands().entity(entity).insert(ReactiveModelKey(attributes.model_key.clone().unwrap()));
//         }
//         root_node
//             .commands()
//             .entity(entity)
//             .insert(FaProgressValuePercentage(None));
//         entity
//     }

//     pub fn new(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands,
//     ) -> Entity {
//         let bar = Self::_build_progress_bar(attributes, root_node);
//         let value = Self::_build_progress_value(root_node, attributes, &attributes.color, bar);

//         if attributes.has_tooltip {
//             build_tooltip_node(attributes, root_node, bar);
//         }
//         root_node.commands().entity(bar).insert(FamiqProgressValueEntity(value));
//         entity_add_child(root_node, value, bar);
//         bar
//     }

//     fn handle_on_mouse_over(
//         mut trigger: Trigger<Pointer<Over>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         bar_q: Query<(&GlobalTransform, Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqProgressBar>>
//     ) {
//         if let Ok((transform, id, tooltip_entity)) = bar_q.get(trigger.entity()) {
//             show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
//             FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::ProgressBar, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_out(
//         mut trigger: Trigger<Pointer<Out>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         bar_q: Query<(Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqProgressBar>>
//     ) {
//         if let Ok((id, tooltip_entity)) = bar_q.get(trigger.entity()) {
//             hide_tooltip(tooltip_entity, &mut tooltip_q);
//             FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::ProgressBar, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_down(
//         mut trigger: Trigger<Pointer<Down>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         bar_q: Query<Option<&WidgetId>, With<IsFamiqProgressBar>>
//     ) {
//         if let Ok(id) = bar_q.get(trigger.entity()) {
//             if trigger.event().button == PointerButton::Secondary {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::ProgressBar, trigger.entity(), id);
//             } else {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::ProgressBar, trigger.entity(), id);
//             }
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_up(
//         mut trigger: Trigger<Pointer<Up>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         bar_q: Query<Option<&WidgetId>, With<IsFamiqProgressBar>>
//     ) {
//         if let Ok(id) = bar_q.get(trigger.entity()) {
//             FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::ProgressBar, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     /// Internal system to detect new progress bars bing created.
//     pub fn detect_new_progress_bar_widget_system(
//         mut commands: Commands,
//         mut progress_materials: ResMut<Assets<ProgressBarMaterial>>,
//         bar_q: Query<
//             (&ComputedNode, &FamiqProgressValueEntity),
//             Or<(Added<IsFamiqProgressBar>, Changed<ComputedNode>)>
//         >,
//         value_q: Query<(&ProgressValueColor, &FaProgressValuePercentage)>,
//     ) {
//         for (computed_node, value_entity) in bar_q.iter() {
//             if let Ok((value_color, percentage)) = value_q.get(value_entity.0) {

//                 if let Color::Srgba(value) = value_color.0 {
//                     let u_blend = if percentage.0.is_some() {
//                         0.0
//                     } else {
//                         1.0
//                     };
//                     commands
//                         .entity(value_entity.0)
//                         .insert(
//                             MaterialNode(progress_materials.add(ProgressBarMaterial {
//                                 u_time: 0.0,
//                                 u_color: Vec3::new(value.red, value.green, value.blue),
//                                 u_blend,
//                                 u_size: computed_node.size()
//                             }))
//                         );
//                 }
//             }
//         }
//     }

//     pub fn _update_progress_bar_material_u_time(
//         time: Res<Time>,
//         mut materials: ResMut<Assets<ProgressBarMaterial>>,
//         query: Query<(&MaterialNode<ProgressBarMaterial>, &FaProgressValuePercentage)>
//     ) {
//         for (material_handle, percentage) in query.iter() {
//             if let Some(material) = materials.get_mut(material_handle) {
//                 if percentage.0.is_none() {
//                     material.u_time = -time.elapsed_secs();
//                     material.u_blend = 1.0;
//                 } else {
//                     material.u_time = 0.0;
//                     material.u_blend = 0.0;
//                 }
//             }
//         }
//     }
// }

// /// Builder for creating `FaProgressBar` widget.
// pub struct FaProgressBarBuilder<'a> {
//     pub attributes: WidgetAttributes,
//     pub root_node: EntityCommands<'a>
// }

// impl<'a> FaProgressBarBuilder<'a> {
//     pub fn new(root_node: EntityCommands<'a>, font_handle: Handle<Font>) -> Self {
//         let mut attributes = WidgetAttributes::default();
//         attributes.font_handle = Some(font_handle);
//         Self {
//             attributes,
//             root_node
//         }
//     }

//     /// Spawn progress bar into UI World.
//     pub fn build(&mut self) -> Entity {
//         self._process_built_in_color_class();
//         self._process_built_in_size_class();
//         self._node();
//         FaProgressBar::new(
//             &self.attributes,
//             &mut self.root_node
//         )
//     }
// }

// impl<'a> SetWidgetAttributes for FaProgressBarBuilder<'a> {
//     fn attributes(&mut self) -> &mut WidgetAttributes {
//         &mut self.attributes
//     }

//     fn _node(&mut self) {
//         self.attributes.node = default_progress_bar_node(&self.attributes.size);
//         if self.attributes.default_display_changed {
//             self.attributes.node.display = self.attributes.default_display;
//         }
//         process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
//     }
// }

// /// API to create `FaProgressBar`.
// pub fn fa_progress_bar_builder<'a>(builder: &'a mut FamiqBuilder) -> FaProgressBarBuilder<'a> {
//     let font_handle = builder.asset_server.load(&builder.resource.font_path);
//     FaProgressBarBuilder::new(
//         builder.ui_root_node.reborrow(),
//         font_handle
//     )
// }

// #[macro_export]
// macro_rules! fa_progress_bar {
//     ( $( $key:ident : $value:tt ),* $(,)? ) => {{
//         let builder = builder_mut();
//         let mut progress_bar = fa_progress_bar_builder(builder);
//         $(
//             $crate::fa_progress_bar_attributes!(progress_bar, $key : $value);
//         )*
//         progress_bar.build()
//     }};
// }

// #[macro_export]
// macro_rules! fa_progress_bar_attributes {
//     ($progress_bar:ident, color: $color:expr) => {{
//         $progress_bar = $progress_bar.color($color);
//     }};

//     ($progress_bar:ident, tooltip: $tooltip:expr) => {{
//         $progress_bar = $progress_bar.tooltip($tooltip);
//     }};

//     ($progress_bar:ident, model: $model:expr) => {{
//         $progress_bar = $progress_bar.model($model);
//     }};

//     // common attributes
//     ($progress_bar:ident, $key:ident : $value:expr) => {{
//         $crate::common_attributes!($progress_bar, $key : $value);
//     }};
// }

pub fn can_run_fa_progress_bar_systems(bar_q: Query<&IsFamiqProgressBar>) -> bool {
    !bar_q.is_empty()
}
