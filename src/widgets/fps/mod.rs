pub mod helper;
pub mod tests;

use crate::utils::*;
use crate::widgets::*;
use crate::widgets::text::base_text::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use helper::*;
use macros::set_widget_attributes;

use super::color::{GREEN_COLOR, WHITE_COLOR, WARNING_COLOR, DANGER_COLOR};

const DEFAULT_FPS_TEXT_SIZE: f32 = 20.0;

/// Marker component for identifying the label part of the FPS text (e.g., "FPS:").
#[derive(Component)]
pub struct IsFPSTextLabel;

/// Marker component for identifying the FPS count text (e.g., "60.0").
#[derive(Component)]
pub struct IsFPSTextCount;

/// Component to indicate whether the FPS text color can change dynamically.
/// - `true`: The FPS text will change color based on the FPS value.
/// - `false`: The FPS text color remains constant.
#[derive(Component)]
pub struct CanChangeColor(pub bool);

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct FpsBuilder {
    pub change_color: RVal,
    pub right_side: RVal,
    pub all_reactive_keys: Vec<String>,
    pub root_node: Entity
}

impl FpsBuilder {
    pub fn new(root_node: Entity, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            attributes,
            root_node,
            all_reactive_keys: Vec::new(),
            cloned_attrs: WidgetAttributes::default(),
            change_color: RVal::Bool(true),
            right_side: RVal::Bool(false)
        }
    }

    pub fn build_fps_count_text(&self, commands: &mut Commands) -> Entity {
        let text_font = TextFont {
            font: self.cloned_attrs.font_handle.clone().unwrap(),
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let entity = commands
            .spawn((
                TextSpan::default(),
                text_font.clone(),
                TextColor(WHITE_COLOR),
                IsFPSTextCount,
                DefaultTextSpanConfig::new(
                    TextSpan::default(),
                    text_font,
                    TextColor(WHITE_COLOR)
                )
            ))
            .id();
        insert_class_id(commands, entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        entity
    }

    pub fn build_fps_count_text_world(&self, world: &mut World) -> Entity {
        let text_font = TextFont {
            font: self.cloned_attrs.font_handle.clone().unwrap(),
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let entity = world
            .spawn((
                TextSpan::default(),
                text_font.clone(),
                TextColor(WHITE_COLOR),
                IsFPSTextCount,
                DefaultTextSpanConfig::new(
                    TextSpan::default(),
                    text_font,
                    TextColor(WHITE_COLOR)
                )
            ))
            .id();
        insert_class_id_world(world, entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        entity
    }

    /// Internal system to update the FPS count and optionally change its color based on the value.
    pub(crate) fn update_fps_count_system(
        diagnostics: Res<DiagnosticsStore>,
        mut text_q: Query<(&mut TextSpan, &mut TextColor, &CanChangeColor, &IsFPSTextCount)>
    ) {
        for (mut text, mut color, change_color, _) in text_q.iter_mut() {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    let value = value as usize;
                    text.0 = format!("{value}");

                    if change_color.0 {
                        if value >= 100 {
                            color.0 = GREEN_COLOR;
                        }
                        else if value >= 60 && value < 100 {
                            color.0 = WARNING_COLOR;
                        }
                        else {
                            color.0 = DANGER_COLOR;
                        }
                    }
                }
            }
        }
    }

    pub fn set_node_right_side(&mut self, state: bool) {
        if state {
            self.cloned_attrs.node.left = Val::Auto;
            self.cloned_attrs.node.right = Val::Px(6.0);
        }
    }

    pub fn set_side(&mut self, r_data: &HashMap<String, RVal>) {
        match self.right_side.to_owned() {
            RVal::Bool(v) => self.set_node_right_side(v),
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);
                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::Bool(state) => self.set_node_right_side(*state),
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }
    }
}

impl SetupWidget for FpsBuilder {
    fn components(&mut self) -> impl Bundle {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = self.cloned_attrs.node.clone();
        process_spacing_built_in_class(&mut style_components.node, &self.cloned_attrs.class);
        (
            MainWidget,
            IsFPSTextLabel,
            GlobalZIndex(6),
            style_components.clone(),
            DefaultWidgetConfig::from(style_components),
            ReactiveWidget
        )
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.all_reactive_keys = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_fps_container_node();
        self.cloned_attrs.override_text_size = Some(DEFAULT_FPS_TEXT_SIZE);
        self.cloned_attrs.default_visibility = Visibility::Visible;
        self.set_side(r_data);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);

        let mut label = FaBaseText::new_with_attributes("FPS:", &self.cloned_attrs);
        label.use_get_color = true;

        let label_entity = label.build(r_data, commands);
        let count_entity = self.build_fps_count_text(commands);

        match self.change_color.to_owned() {
            RVal::Bool(v) => {
                commands.entity(count_entity).insert(CanChangeColor(v));
            }
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);
                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::Bool(state) => {
                                commands.entity(count_entity).insert(CanChangeColor(*state));
                            }
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }

        commands
            .entity(label_entity)
            .add_child(count_entity)
            .insert(self.components());

        commands.entity(self.root_node).add_child(label_entity);

        insert_class_id(commands, label_entity, &self.attributes.id, &self.attributes.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                label_entity,
                WidgetBuilder {
                    builder: BuilderType::Fps(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        label_entity
    }

    fn build_with_world(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Option<Entity> {
        self.all_reactive_keys = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_fps_container_node();
        self.cloned_attrs.override_text_size = Some(DEFAULT_FPS_TEXT_SIZE);
        self.cloned_attrs.default_visibility = Visibility::Visible;
        self.set_side(r_data);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);

        let mut label = FaBaseText::new_with_attributes("FPS:", &self.cloned_attrs);
        label.use_get_color = true;

        let label_entity = label.build_with_world(r_data, world);
        let count_entity = self.build_fps_count_text_world(world);

        match self.change_color.to_owned() {
            RVal::Bool(v) => {
                world.entity_mut(count_entity).insert(CanChangeColor(v));
            }
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);
                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::Bool(state) => {
                                world.entity_mut(count_entity).insert(CanChangeColor(*state));
                            }
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }

        world
            .entity_mut(label_entity.unwrap())
            .add_child(count_entity)
            .insert(self.components());

        world.entity_mut(self.root_node).add_child(label_entity.unwrap());

        insert_class_id_world(world, label_entity.unwrap(), &self.attributes.id, &self.attributes.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            label_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::Fps(cloned_builder)
            }
        ));
        label_entity
    }
}

#[macro_export]
macro_rules! fps {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let root_entity = famiq_builder.resource.root_node_entity.unwrap();
        let f_builder = &mut FpsBuilder::new(root_entity, &famiq_builder.get_font_handle());
        $(
            $crate::fps_text_attributes!(f_builder, $key : $value);
        )*
        f_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! fps_text_attributes {
    ($f_builder:ident, change_color: $change_color:expr) => {{
        match to_rval($change_color) {
            Ok(v) => $f_builder.change_color = v,
            Err(_) => panic!("\nchange_color attribute accepts only bool and reactive string\n")
        }
    }};
    ($f_builder:ident, right_side: $right_side:expr) => {{
        match to_rval($right_side) {
            Ok(v) => $f_builder.right_side = v,
            Err(_) => panic!("\right_side attribute accepts only bool and reactive string\n")
        }
    }};
    // common attributes
    ($f_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($f_builder, $key : $value);
    }};
}

// /// Component to indicate whether the FPS text color can change dynamically.
// /// - `true`: The FPS text will change color based on the FPS value.
// /// - `false`: The FPS text color remains constant.
// #[derive(Component)]
// pub struct CanChangeColor(pub bool);


// pub struct FaFpsText;

// impl<'a> FaFpsText {
//     fn _build_fps(
//         attributes: &WidgetAttributes,
//         change_color: bool,
//         root_node: &'a mut EntityCommands
//     ) -> Entity {
//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = attributes.node.clone();
//         style_components.visibility = Visibility::Visible;

//         let label_txt_font = TextFont {
//             font: attributes.font_handle.clone().unwrap(),
//             font_size: DEFAULT_FPS_TEXT_SIZE,
//             ..default()
//         };
//         let count_txt_font = label_txt_font.clone();

//         let label_txt_entity = root_node
//             .commands()
//             .spawn((
//                 Text::new("FPS:"),
//                 label_txt_font.clone(),
//                 TextColor(WHITE_COLOR),
//                 TextLayout::new_with_justify(JustifyText::Center),
//                 DefaultTextConfig::new(
//                     Text::new("FPS:"),
//                     label_txt_font,
//                     TextColor(WHITE_COLOR),
//                     TextLayout::new_with_justify(JustifyText::Center),
//                 ),
//                 MainWidget,
//                 IsFamiqFPSTextLabel,
//                 style_components.clone(),
//                 GlobalZIndex(6),
//                 DefaultWidgetConfig::from(style_components)
//             ))
//             .observe(FaFpsText::handle_on_mouse_over)
//             .observe(FaFpsText::handle_on_mouse_out)
//             .id();

//         let count_txt_entity = root_node
//             .commands()
//             .spawn((
//                 TextSpan::default(),
//                 count_txt_font.clone(),
//                 TextColor(WHITE_COLOR),
//                 IsFamiqFPSTextCount,
//                 CanChangeColor(change_color),
//                 DefaultTextSpanEntity::new(
//                     TextSpan::default(),
//                     count_txt_font,
//                     TextColor(WHITE_COLOR),
//                 )
//             ))
//             .id();

//         insert_id_and_class(root_node, label_txt_entity, &attributes.id, &attributes.class);
//         insert_id_and_class(root_node, count_txt_entity, &attributes.id, &attributes.class);
//         entity_add_child(root_node, count_txt_entity, label_txt_entity);
//         label_txt_entity
//     }

//     pub fn new(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands,
//         change_color: bool,
//     ) -> Entity {
//         Self::_build_fps(attributes, change_color, root_node)
//     }

//     fn handle_on_mouse_over(
//         mut trigger: Trigger<Pointer<Over>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         fps_q: Query<Option<&WidgetId>, With<IsFamiqFPSTextLabel>>
//     ) {
//         if let Ok(id) = fps_q.get(trigger.entity()) {
//             FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::FpsText, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_out(
//         mut trigger: Trigger<Pointer<Out>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         fps_q: Query<Option<&WidgetId>, With<IsFamiqFPSTextLabel>>
//     ) {
//         if let Ok(id) = fps_q.get(trigger.entity()) {
//             FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::FpsText, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     /// Internal system to update the FPS count and optionally change its color based on the value.
//     pub fn update_fps_count_system(
//         diagnostics: Res<DiagnosticsStore>,
//         mut text_q: Query<(&mut TextSpan, &mut TextColor, &CanChangeColor, &IsFamiqFPSTextCount)>
//     ) {
//         for (mut text, mut color, change_color, _) in text_q.iter_mut() {
//             if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
//                 if let Some(value) = fps.smoothed() {
//                     text.0 = format!("{value:.0}");

//                     if change_color.0 {
//                         if value > 100.0 {
//                             color.0 = GREEN_COLOR;
//                         }
//                         else if value > 60.0 && value < 100.0 {
//                             color.0 = WARNING_COLOR;
//                         }
//                         else {
//                             color.0 = DANGER_COLOR;
//                         }
//                     }
//                     // else {
//                     //     color.0 = WHITE_COLOR;
//                     // }
//                 }
//             }
//         }
//     }
// }

// /// Builder for creating an FPS text widget.
// pub struct FaFpsTextBuilder<'a> {
//     pub attributes: WidgetAttributes,
//     pub change_color: bool,
//     pub right_side: bool,
//     pub root_node: EntityCommands<'a>
// }

// impl<'a> FaFpsTextBuilder<'a> {
//     pub fn new(font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
//         let mut attributes = WidgetAttributes::default();
//         attributes.font_handle = Some(font_handle);
//         Self {
//             attributes,
//             root_node,
//             change_color: false,
//             right_side: false
//         }
//     }

//     /// Enables dynamic color changes based on FPS value.
//     pub fn change_color(mut self, can_change: bool) -> Self {
//         self.change_color = can_change;
//         self
//     }

//     /// Aligns the FPS widget to the right or left top corner of the screen.
//     pub fn side(mut self, right: bool) -> Self {
//         self.right_side = right;
//         self
//     }

//     /// Spawn fps into UI World.
//     pub fn build(&mut self) -> Entity {
//         self._node();

//         if self.right_side {
//             self.attributes.node.left = Val::Auto;
//             self.attributes.node.right = Val::Px(6.0);
//         }

//         FaFpsText::new(
//             &self.attributes,
//             &mut self.root_node,
//             self.change_color
//         )
//     }
// }

// impl<'a> SetWidgetAttributes for FaFpsTextBuilder<'a> {
//     fn attributes(&mut self) -> &mut WidgetAttributes {
//         &mut self.attributes
//     }

//     fn _node(&mut self) {
//         self.attributes.node = default_fps_text_container_node();
//         if self.attributes.default_display_changed {
//             self.attributes.node.display = self.attributes.default_display;
//         }
//         process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
//     }
// }

// /// API to create an `FaFpsTextBuilder`.
// pub fn fa_fps_builder<'a>(builder: &'a mut FamiqBuilder) -> FaFpsTextBuilder<'a> {
//     let font_handle = builder.asset_server.load(&builder.resource.font_path);
//     FaFpsTextBuilder::new(
//         font_handle,
//         builder.ui_root_node.reborrow()
//     )
// }

// #[macro_export]
// macro_rules! fa_fps {
//     ( $( $key:ident : $value:tt ),* $(,)? ) => {{
//         let builder = builder_mut();
//         let mut fps = fa_fps_builder(builder);
//         $(
//             $crate::fa_fps_attributes!(fps, $key : $value);
//         )*
//         fps.build()
//     }};
// }

// #[macro_export]
// macro_rules! fa_fps_attributes {
//     ($fps:ident, color: $color:expr) => {{
//         $fps = $fps.color($color);
//     }};

//     ($fps:ident, right_side: $right_side:expr) => {{
//         $fps.right_side = $right_side;
//     }};

//     ($fps:ident, change_color: $change_color:expr) => {{
//         $fps.change_color = $change_color;
//     }};

//     // common attributes
//     ($fps:ident, $key:ident : $value:expr) => {{
//         $crate::common_attributes!($fps, $key : $value);
//     }};
// }

// /// a system to check if FPS internal system(s) can run.
///
/// True only if fps widget is created.
pub fn can_run_fps_systems(fps_q: Query<&IsFPSTextLabel>) -> bool {
    !fps_q.is_empty()
}
