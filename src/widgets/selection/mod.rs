pub mod components;
pub mod styling;
pub mod systems;
pub mod tests;

use macros::set_widget_attributes;
use crate::widgets::text::base_text::*;
use crate::widgets::container::base_container::*;
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::*;
use crate::plugin::{CursorType, CursorIcons};
use bevy::ui::FocusPolicy;
use bevy::prelude::*;

pub(crate) use components::*;
pub(crate) use styling::*;
pub(crate) use systems::*;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct SelectionBuilder {
    pub all_reactive_keys: Vec<String>,
    pub placeholder: String,
    pub choices: Vec<String>,
    pub panel_entity: Option<Entity>
}

impl SelectionBuilder {
    pub fn new(placeholder: String, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            attributes,
            cloned_attrs: WidgetAttributes::default(),
            all_reactive_keys: Vec::new(),
            placeholder,
            choices: Vec::new(),
            panel_entity: None
        }
    }

    fn handle_placeholder_val(&mut self, r_data: &HashMap<String, RVal>) -> String {
        let reactive_keys = get_reactive_key(&self.placeholder);
        let parsed_text = replace_reactive_keys(&self.placeholder, &reactive_keys, r_data);
        self.all_reactive_keys.extend_from_slice(&reactive_keys);
        parsed_text
    }

    pub fn build_placeholder(&mut self, commands: &mut Commands, r_data: &HashMap<String, RVal>) -> (Entity, String) {
        let parsed_text = self.handle_placeholder_val(r_data);
        let mut ph = FaBaseText::new_with_attributes(&parsed_text, &self.cloned_attrs);
        ph.layout = TextLayout::new(JustifyText::Left, LineBreak::NoWrap);

        let ph_entity = ph.build(r_data, commands);
        commands.entity(ph_entity).insert(SelectorPlaceHolder);
        insert_class_id(commands, ph_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        (ph_entity, parsed_text)
    }

    pub fn build_arrow(&self, commands: &mut Commands, r_data: &HashMap<String, RVal>) -> Entity {
        let mut arrow = FaBaseText::new_with_attributes("▼", &self.cloned_attrs);
        arrow.layout = TextLayout::new_with_justify(JustifyText::Right);

        let arrow_entity = arrow.build(r_data, commands);
        commands.entity(arrow_entity).insert(ArrowIcon);
        insert_class_id(commands, arrow_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        arrow_entity
    }

    pub fn build_choice_text(&self, text: &str, commands: &mut Commands, r_data: &HashMap<String, RVal>) -> Entity {
        let mut choice_text = FaBaseText::new_with_attributes(text, &self.cloned_attrs);
        choice_text.build(r_data, commands)
    }

    fn choice_wrapper_components(&self, text: Entity, selector: Entity) -> impl Bundle {
        (IsFamiqSelectionChoice, SelectionChoiceTextEntity(text), SelectorEntity(selector))
    }

    pub fn build_choice_wrapper(
        &self,
        text_entity: Entity,
        selector_entity: Entity,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut wrapper = FaBaseContainer::new();
        wrapper.cloned_attrs.node = default_choice_container_node();

        let wrapper_entity = wrapper.build(r_data, commands);
        commands
            .entity(wrapper_entity)
            .add_child(text_entity)
            .insert(self.choice_wrapper_components(text_entity, selector_entity));
        wrapper_entity
    }

    fn choice_panel_components(&self) -> impl Bundle {
        (
            IsFamiqSelectionChoicesPanel,
            FocusPolicy::Block,
            GlobalZIndex(2),
            Transform::default()
        )
    }

    pub fn build_choices_panel(
        &mut self,
        selector_entity: Entity,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>,
    ) -> Entity {
        let mut choice_entities: Vec<Entity> = Vec::new();
        self.choices.insert(0, "-/-".into());

        self.choices.iter().for_each(|choice| {
            let choice_text = self.build_choice_text(choice, commands, r_data);
            let choice_wrapper = self.build_choice_wrapper(choice_text, selector_entity, commands, r_data);
            choice_entities.push(choice_wrapper);
        });

        let mut panel = FaBaseContainer::new();
        panel.cloned_attrs.node = default_selection_choices_panel_node();
        panel.cloned_attrs.color = self.cloned_attrs.color.clone();
        panel.cloned_attrs.id = self.cloned_attrs.id.clone();
        panel.cloned_attrs.class = self.cloned_attrs.class.clone();

        let panel_entity = panel.build(r_data, commands);
        commands
            .entity(panel_entity)
            .add_children(&choice_entities)
            .remove::<DefaultWidgetConfig>()
            .insert(self.choice_panel_components());

        self.panel_entity = Some(panel_entity);
        panel_entity
    }

    pub fn rebuild_choices_panel(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) {
        let mut panel = FaBaseContainer::new();
        panel.cloned_attrs.node = default_selection_choices_panel_node();
        panel.cloned_attrs.color = self.cloned_attrs.color.clone();
        panel.cloned_attrs.id = self.cloned_attrs.id.clone();
        panel.cloned_attrs.class = self.cloned_attrs.class.clone();
        panel.rebuild(r_data, self.panel_entity.unwrap(), world);
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_selector_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
    }

    pub(crate) fn arrow_up(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
        if let Ok(mut text) = text_q.get_mut(arrow_entity) {
            text.0 = "▲".to_string()
        }
    }

    pub(crate) fn arrow_down(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
        if let Ok(mut text) = text_q.get_mut(arrow_entity) {
            text.0 = "▼".to_string()
        }
    }
}

impl SetupWidget for SelectionBuilder {
    fn components(&mut self) -> impl Bundle {
        (
            MainWidget,
            IsFamiqSelectionSelector,
            ReactiveWidget,
            SelectionValue::default(),
        )
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.prepare_attrs(r_data);
        let mut selector = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let selector_entity = selector.build(r_data, commands);
        let (ph_entity, ph_text) = self.build_placeholder(commands, r_data);
        let arrow_entity = self.build_arrow(commands, r_data);
        let panel_entity = self.build_choices_panel(selector_entity, commands, r_data);

        commands.entity(ph_entity).insert(SelectorEntity(selector_entity));

        commands
            .entity(selector_entity)
            .add_children(&[ph_entity, arrow_entity, panel_entity])
            .insert((
                self.components(),
                SelectionChoicesPanelEntity(panel_entity),
                SelectorPlaceHolderEntity(ph_entity),
                SelectorArrowIconEntity(arrow_entity),
                SelectorPlaceholderText(ph_text)
            ))
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, selector_entity);
        }
        insert_class_id(commands, selector_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model(commands, selector_entity, &self.cloned_attrs.model_key);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                selector_entity,
                WidgetBuilder {
                    builder: BuilderType::Selection(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        selector_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        self.prepare_attrs(r_data);
        let mut selector = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        selector.rebuild(r_data, old_entity, world);
        self.rebuild_choices_panel(r_data, world);

        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Selection(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

#[macro_export]
macro_rules! selection {
    ( placeholder: $placeholder:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let s_builder = &mut SelectionBuilder::new($placeholder.to_string(), &famiq_builder.get_font_handle());
        $(
            $crate::selection_attributes!(s_builder, $key : $value);
        )*
        s_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! selection_attributes {
    ($s_builder:ident, choices: $choices:expr) => {{
        $s_builder.choices = $choices.iter().map(|s| s.to_string()).collect();
    }};
    ($s_builder:ident, model: $model:expr) => {{
        $s_builder.set_model($model);
    }};
    ($s_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($s_builder, $key : $value);
    }};
}

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionSelector>) -> bool {
    !selection_q.is_empty()
}
