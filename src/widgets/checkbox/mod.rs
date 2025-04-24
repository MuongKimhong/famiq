pub mod styling;
pub mod components;
pub mod systems;
pub use components::*;
pub use styling::*;
use systems::*;

use bevy::prelude::*;

use crate::plugin::{CursorType, CursorIcons};
use crate::widgets::text::base_text::*;
use crate::widgets::container::base_container::*;
use crate::utils::*;
use crate::widgets::*;
use macros::*;

use super::color::PRIMARY_DARK_COLOR;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct CheckboxBuilder {
    pub choices: Vec<String>,
    pub all_reactive_keys: Vec<String>,
    pub vertical: RVal // align item vertically
}

impl CheckboxBuilder {
    pub fn new(font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            attributes,
            cloned_attrs: WidgetAttributes::default(),
            all_reactive_keys: Vec::new(),
            choices: Vec::new(),
            vertical: RVal::Bool(false)
        }
    }

    pub(crate) fn choice_text(
        &self,
        text: &str,
        r_data: &HashMap<String, RVal>,
        commands: &mut Commands
    ) -> Entity {
        let mut text = FaBaseText::new_with_attributes(text, &self.cloned_attrs);
        text.use_get_color = true;
        text.build(r_data, commands)
    }

    pub(crate) fn choice_text_world(
        &self,
        text: &str,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Entity {
        let mut text = FaBaseText::new_with_attributes(text, &self.cloned_attrs);
        text.use_get_color = true;
        text.build_with_world(r_data, world).unwrap()
    }

    fn tickbox_components(&self) -> impl Bundle {
        (
            default_choice_box_node(),
            IsFamiqCheckboxItemBox,
            CheckBoxChoiceTicked(false),
            BackgroundColor::default(),
            BorderRadius::all(Val::Px(4.0)),
            BorderColor(get_color(&self.cloned_attrs.color))
        )
    }

    pub(crate) fn choice_tick_box(&self, commands: &mut Commands) -> Entity {
        commands.spawn(self.tickbox_components()).id()
    }

    pub(crate) fn choice_tick_box_world(&self, world: &mut World) -> Entity {
        world.spawn(self.tickbox_components()).id()
    }

    fn choice_wrapper_components(&self, text: String, checkbox_entity: Entity, box_entity: Entity) -> impl Bundle {
        (
            default_choice_container_node(),
            CheckBoxItemText(text),
            CheckBoxMainContainerEntity(checkbox_entity),
            CheckBoxItemBoxEntity(box_entity),
            IsFamiqCheckboxItem
        )
    }

    pub(crate) fn choice_wrapper(
        &self,
        text: String,
        box_entity: Entity,
        checkbox_entity: Entity,
        commands: &mut Commands,
    ) -> Entity {
        commands
            .spawn(self.choice_wrapper_components(text, checkbox_entity, box_entity))
            .observe(on_mouse_out)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .id()
    }

    pub(crate) fn choice_wrapper_world(
        &self,
        text: String,
        box_entity: Entity,
        checkbox_entity: Entity,
        world: &mut World,
    ) -> Entity {
        world
            .spawn(self.choice_wrapper_components(text, checkbox_entity, box_entity))
            .observe(on_mouse_out)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .id()
    }

    pub(crate) fn build_choices(
        &mut self,
        checkbox_entity: Entity,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>,
    ) {
        let mut all_choices: Vec<Entity> = Vec::new();
        for choice in self.choices.iter() {
            let reactive_keys = get_reactive_key(choice);
            let parsed_choice = replace_reactive_keys(choice, &reactive_keys, r_data);
            self.all_reactive_keys.extend_from_slice(&reactive_keys);

            let choice_box = self.choice_tick_box(commands);
            let choice_text = self.choice_text(&parsed_choice, r_data,  commands);
            let choice_wrapper = self.choice_wrapper(
                parsed_choice,
                choice_box,
                checkbox_entity,
                commands
            );
            commands.entity(choice_wrapper).add_children(&[choice_box, choice_text]);
            all_choices.push(choice_wrapper);
        }
        commands.entity(checkbox_entity).add_children(&all_choices);
    }

    pub(crate) fn build_choices_world(
        &mut self,
        checkbox_entity: Entity,
        world: &mut World,
        r_data: &HashMap<String, RVal>
    ) {
        let mut all_choices: Vec<Entity> = Vec::new();

        for choice in self.choices.iter() {
            let reactive_keys = get_reactive_key(choice);
            let parsed_choice = replace_reactive_keys(choice, &reactive_keys, r_data);
            self.all_reactive_keys.extend_from_slice(&reactive_keys);

            let choice_box = self.choice_tick_box_world(world);
            let choice_text = self.choice_text_world(&parsed_choice, r_data, world);
            let choice_wrapper = self.choice_wrapper_world(
                parsed_choice,
                choice_box,
                checkbox_entity,
                world
            );
            world.entity_mut(choice_wrapper).add_children(&[choice_box, choice_text]);
            all_choices.push(choice_wrapper);
        }
        world.entity_mut(checkbox_entity).add_children(&all_choices);
    }

    pub(crate) fn set_flex_direction(&mut self, vertical: bool) {
        if vertical {
            self.cloned_attrs.node.flex_direction = FlexDirection::Column;
        } else {
            self.cloned_attrs.node.flex_direction = FlexDirection::Row;
        }
    }

    pub(crate) fn handle_vertical_val(&mut self, r_data: &HashMap<String, RVal>) {
        match self.vertical.to_owned() {
            RVal::Bool(v) => self.set_flex_direction(v),
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);

                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::Bool(state) => self.set_flex_direction(*state),
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_main_container_node();
        self.handle_vertical_val(r_data);
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
    }
}

impl SetupWidget for CheckboxBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqCheckbox, MainWidget, ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.prepare_attrs(r_data);

        let mut checkbox = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let checkbox_entity = checkbox.build(r_data, commands);
        self.build_choices(checkbox_entity, commands, r_data);
        commands
            .entity(checkbox_entity)
            .insert(self.components());

        insert_model(commands, checkbox_entity, &self.cloned_attrs.model_key);
        insert_class_id(commands, checkbox_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                checkbox_entity,
                WidgetBuilder {
                    builder: BuilderType::Checkbox(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        checkbox_entity
    }

    fn build_with_world(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Option<Entity> {
        self.prepare_attrs(r_data);

        let mut checkbox = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let checkbox_entity = checkbox.build_with_world(r_data, world);
        self.build_choices_world(checkbox_entity.unwrap(), world, r_data);
        world
            .entity_mut(checkbox_entity.unwrap())
            .insert(self.components());

        insert_model_world(world, checkbox_entity.unwrap(), &self.cloned_attrs.model_key);
        insert_class_id_world(world, checkbox_entity.unwrap(), &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            checkbox_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::Checkbox(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
        checkbox_entity
    }
}

#[macro_export]
macro_rules! checkbox {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let c_builder = &mut CheckboxBuilder::new(&famiq_builder.get_font_handle());
        $(
            $crate::choices_attributes!(c_builder, $key : $value);
        )*
        c_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! choices_attributes {
    ($c_builder:ident, choices: $choices:expr) => {{
        $c_builder.choices = $choices.iter().map(|s| s.to_string()).collect();
    }};

    ($c_builder:ident, vertical: $vertical:expr) => {{
        match to_rval($vertical) {
            Ok(v) => $c_builder.vertical = v,
            Err(_) => panic!("\nvertical attribute accepts only boolean and reactive string\n")
        }
    }};
    ($c_builder:ident, model: $model:expr) => {{
        $c_builder.set_model($model);
    }};
    ($c_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($c_builder, $key : $value);
    }};
}

pub fn can_run_checkbox_systems(checkbox_q: Query<&IsFamiqCheckbox>) -> bool {
    !checkbox_q.is_empty()
}
