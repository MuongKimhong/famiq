pub mod components;
pub mod styling;
pub mod tests;
pub mod systems;

pub(crate) use components::*;
use systems::*;
use styling::*;

use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::widgets::*;
use crate::widgets::text::base_text::*;
use crate::widgets::container::base_container::*;
use crate::event_writer::*;
use bevy::prelude::*;
use macros::*;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ButtonBuilder {
    pub value: String,
    pub all_reactive_keys: Vec<String>,
    pub old_text_entity: Option<Entity>
}

impl ButtonBuilder {
    pub fn new(value: String, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            value,
            attributes,
            all_reactive_keys: Vec::new(),
            cloned_attrs: WidgetAttributes::default(),
            old_text_entity: None
        }
    }

    pub fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) -> String {
        let reactive_keys = get_reactive_key(&self.value);
        let parsed_text = replace_reactive_keys(&self.value, &reactive_keys, r_data);
        self.all_reactive_keys.extend_from_slice(&reactive_keys);

        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_button_node();
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
        parsed_text
    }
}

impl SetupWidget for ButtonBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqButton, ButtonColorBeforePressed(None), MainWidget, ReactiveWidget)
    }

    fn build(
        &mut self,
        r_data: &HashMap<String, RVal>,
        commands: &mut Commands
    ) -> Entity {
        let parsed_text = self.prepare_attrs(r_data);
        let mut text = FaBaseText::new_with_attributes(&parsed_text,  &self.cloned_attrs);
        let text_entity = text.build(r_data, commands);
        self.old_text_entity = Some(text_entity);

        let mut button = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let button_entity = button.build(r_data, commands);

        commands.entity(text_entity).insert(IsFamiqButtonText);
        commands
            .entity(button_entity)
            .insert(self.components())
            .add_child(text_entity)
            .insert(ButtonTextEntity(text_entity))
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, button_entity);
        }
        insert_class_id(commands, text_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_class_id(commands, button_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                button_entity,
                WidgetBuilder {
                    builder: BuilderType::Button(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        button_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        let parsed_text = self.prepare_attrs(r_data);
        let mut text = FaBaseText::new_with_attributes(&parsed_text,  &self.cloned_attrs);
        text.rebuild(r_data, self.old_text_entity.unwrap(), world);

        let mut button = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        button.rebuild(r_data, old_entity, world);

        let mut query = world.query::<(&BackgroundColor, &mut ButtonColorBeforePressed)>();
        if let Ok((bg, mut bf_pressed)) = query.get_mut(world, old_entity) {
            bf_pressed.0 = Some(bg.0.clone());
        }

        insert_class_id_world(world, self.old_text_entity.unwrap(), &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Button(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

/// Macro for creating a button.
#[macro_export]
macro_rules! button {
    ( text: $text:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let btn_builder = &mut ButtonBuilder::new($text.to_string(), &famiq_builder.get_font_handle());
        $(
            $crate::button_attributes!(btn_builder, $key : $value);
        )*
        btn_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! button_attributes {
    ($btn_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($btn_builder, $key : $value);
    }};
}

/// Checks if the button internal system(s) can run.
///
/// `True` only if there is a button widget created.
pub fn can_run_button_systems(button_q: Query<&IsFamiqButton>) -> bool {
    !button_q.is_empty()
}
