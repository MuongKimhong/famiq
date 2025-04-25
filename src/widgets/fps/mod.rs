pub mod components;
pub mod helper;
pub mod tests;

use crate::utils::*;
use crate::widgets::*;
use crate::widgets::text::base_text::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub(crate) use components::*;
pub(crate) use helper::*;
use macros::set_widget_attributes;

use super::color::{GREEN_COLOR, WHITE_COLOR, WARNING_COLOR, DANGER_COLOR};

const DEFAULT_FPS_TEXT_SIZE: f32 = 18.0;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct FpsBuilder {
    pub change_color: RVal,
    pub right_side: RVal,
    pub all_reactive_keys: Vec<String>,
    pub root_node: Entity,
    pub count_text_entity: Option<Entity>
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
            right_side: RVal::Bool(false),
            count_text_entity: None
        }
    }

    pub fn build_fps_count_text(&mut self, commands: &mut Commands) -> Entity {
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
        self.count_text_entity = Some(entity);
        entity
    }

    pub fn rebuild_count_text(&mut self, world: &mut World) {
        insert_class_id_world(
            world,
            self.count_text_entity.unwrap(),
            &self.cloned_attrs.id,
            &self.cloned_attrs.class
        );
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
        } else {
            self.cloned_attrs.node.right = Val::Auto;
            self.cloned_attrs.node.left = Val::Px(6.0);
        }
    }

    pub(crate) fn handle_side_val(&mut self, r_data: &HashMap<String, RVal>) {
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

    pub(crate) fn prepar_attrs(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_fps_container_node();
        self.cloned_attrs.override_text_size = Some(DEFAULT_FPS_TEXT_SIZE);
        self.cloned_attrs.default_visibility = Visibility::Visible;
        self.handle_side_val(r_data);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
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
        self.prepar_attrs(r_data);
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

        commands.entity(label_entity).add_child(count_entity).insert(self.components());
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

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        self.prepar_attrs(r_data);
        let mut label = FaBaseText::new_with_attributes("FPS:", &self.cloned_attrs);
        label.use_get_color = true;
        label.rebuild(r_data, old_entity, world);
        self.rebuild_count_text(world);

        match self.change_color.to_owned() {
            RVal::Bool(v) => {
                world.entity_mut(self.count_text_entity.unwrap()).insert(CanChangeColor(v));
            }
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);
                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::Bool(state) => {
                                world.entity_mut(self.count_text_entity.unwrap()).insert(CanChangeColor(*state));
                            }
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }
        world.entity_mut(old_entity).insert(self.components());
        insert_class_id_world(world, old_entity, &self.attributes.id, &self.attributes.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Fps(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
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
    ($f_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($f_builder, $key : $value);
    }};
}

/// a system to check if FPS internal system(s) can run.
///
/// True only if fps widget is created.
pub fn can_run_fps_systems(fps_q: Query<&IsFPSTextLabel>) -> bool {
    !fps_q.is_empty()
}
