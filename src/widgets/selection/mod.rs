pub mod components;
pub mod styling;
pub mod systems;
pub mod tests;

use macros::set_widget_attributes;
use crate::widgets::text::base_text::*;
use crate::widgets::container::base_container::*;
use crate::resources::*;
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::*;
use crate::plugin::{CursorType, CursorIcons};
use bevy::ui::FocusPolicy;
use bevy::prelude::*;

pub use components::*;
pub use styling::*;
pub use systems::*;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct SelectionBuilder {
    pub placeholder: String,
    pub choices: Vec<String>,
}

impl SelectionBuilder {
    pub fn new(placeholder: String, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            attributes,
            cloned_attrs: WidgetAttributes::default(),
            placeholder,
            choices: Vec::new()
        }
    }

    pub fn build_placeholder(
        &self,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>,
        all_reactive_keys: &mut Vec<String>
    ) -> (Entity, String) {
        let reactive_keys = get_reactive_key(&self.placeholder);
        let parsed_text = replace_reactive_keys(&self.placeholder, &reactive_keys, r_data);
        all_reactive_keys.extend_from_slice(&reactive_keys);

        let mut ph = FaBaseText::new_with_attributes(&parsed_text, &self.cloned_attrs);
        ph.layout = TextLayout::new(JustifyText::Left, LineBreak::NoWrap);

        let ph_entity = ph.build(r_data, commands);
        commands.entity(ph_entity).insert(SelectorPlaceHolder);
        insert_class_id(commands, ph_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        (ph_entity, parsed_text)
    }

    pub fn build_placeholder_world(
        &self,
        world: &mut World,
        r_data: &HashMap<String, RVal>,
        all_reactive_keys: &mut Vec<String>
    ) -> (Entity, String) {
        let reactive_keys = get_reactive_key(&self.placeholder);
        let parsed_text = replace_reactive_keys(&self.placeholder, &reactive_keys, r_data);
        all_reactive_keys.extend_from_slice(&reactive_keys);

        let mut ph = FaBaseText::new_with_attributes(&parsed_text, &self.cloned_attrs);
        ph.layout = TextLayout::new(JustifyText::Left, LineBreak::NoWrap);

        let ph_entity = ph.build_with_world(r_data, world).unwrap();
        world.entity_mut(ph_entity).insert(SelectorPlaceHolder);
        insert_class_id_world(world, ph_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        (ph_entity, parsed_text)
    }

    pub fn build_arrow(
        &self,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut arrow = FaBaseText::new_with_attributes("▼", &self.cloned_attrs);
        arrow.layout = TextLayout::new_with_justify(JustifyText::Right);

        let arrow_entity = arrow.build(r_data, commands);
        commands.entity(arrow_entity).insert(ArrowIcon);
        insert_class_id(commands, arrow_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        arrow_entity
    }

    pub fn build_arrow_world(
        &self,
        world: &mut World,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut arrow = FaBaseText::new_with_attributes("▼", &self.cloned_attrs);
        arrow.layout = TextLayout::new_with_justify(JustifyText::Right);

        let arrow_entity = arrow.build_with_world(r_data, world).unwrap();
        world.entity_mut(arrow_entity).insert(ArrowIcon);
        insert_class_id_world(world, arrow_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        arrow_entity
    }

    pub fn build_choice_text(
        &self,
        text: &str,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut choice_text = FaBaseText::new_with_attributes(text, &self.cloned_attrs);
        choice_text.build(r_data, commands)
    }

    pub fn build_choice_text_world(
        &self,
        text: &str,
        world: &mut World,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut choice_text = FaBaseText::new_with_attributes(text, &self.cloned_attrs);
        choice_text.build_with_world(r_data, world).unwrap()
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
            .insert((
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                SelectorEntity(selector_entity)
            ));

        wrapper_entity
    }

    pub fn build_choice_wrapper_world(
        &self,
        text_entity: Entity,
        selector_entity: Entity,
        world: &mut World,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut wrapper = FaBaseContainer::new();
        wrapper.cloned_attrs.node = default_choice_container_node();

        let wrapper_entity = wrapper.build_with_world(r_data, world).unwrap();
        world
            .entity_mut(wrapper_entity)
            .add_child(text_entity)
            .insert((
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                SelectorEntity(selector_entity)
            ));
        wrapper_entity
    }

    pub fn build_choices_panel(
        &mut self,
        selector_entity: Entity,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>,
        all_reactive_keys: &mut Vec<String>
    ) -> Entity {
        let mut choice_entities: Vec<Entity> = Vec::new();
        self.choices.insert(0, "-/-".into());

        self.choices.iter().for_each(|choice| {
            let reactive_keys = get_reactive_key(choice);
            let parsed_choice = replace_reactive_keys(choice, &reactive_keys, r_data);
            all_reactive_keys.extend_from_slice(&reactive_keys);

            let choice_text = self.build_choice_text(&parsed_choice, commands, r_data);
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
            .insert((
                IsFamiqSelectionChoicesPanel,
                FocusPolicy::Block,
                GlobalZIndex(2),
                Transform::default()
            ));
        panel_entity
    }

    pub fn build_choices_panel_world(
        &mut self,
        selector_entity: Entity,
        world: &mut World,
        r_data: &HashMap<String, RVal>,
        all_reactive_keys: &mut Vec<String>
    ) -> Entity {
        let mut choice_entities: Vec<Entity> = Vec::new();
        if !self.choices.contains(&"-/-".to_owned()) {
            self.choices.insert(0, "-/-".into());
        }

        self.choices.iter().for_each(|choice| {
            let reactive_keys = get_reactive_key(choice);
            let parsed_choice = replace_reactive_keys(choice, &reactive_keys, r_data);
            all_reactive_keys.extend_from_slice(&reactive_keys);

            let choice_text = self.build_choice_text_world(&parsed_choice, world, r_data);
            let choice_wrapper = self.build_choice_wrapper_world(choice_text, selector_entity, world, r_data);
            choice_entities.push(choice_wrapper);
        });

        let mut panel = FaBaseContainer::new();
        panel.cloned_attrs.node = default_selection_choices_panel_node();
        panel.cloned_attrs.color = self.cloned_attrs.color.clone();
        panel.cloned_attrs.id = self.cloned_attrs.id.clone();
        panel.cloned_attrs.class = self.cloned_attrs.class.clone();

        let panel_entity = panel.build_with_world(r_data, world).unwrap();
        world
            .entity_mut(panel_entity)
            .add_children(&choice_entities)
            .remove::<DefaultWidgetConfig>()
            .insert((
                IsFamiqSelectionChoicesPanel,
                FocusPolicy::Block,
                GlobalZIndex(2),
                Transform::default()
            ));
        panel_entity
    }

    fn on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut selector_q: Query<
            (&mut BoxShadow, &BorderColor, Option<&WidgetId>, &GlobalTransform, Option<&TooltipEntity>),
            With<IsFamiqSelectionSelector>
        >,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        if let Ok((mut box_shadow, border_color, id, transform, tooltip_entity)) = selector_q.get_mut(trigger.entity()) {
            box_shadow.color = border_color.0.clone();
            show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Selection, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut selector_q: Query<
            (&mut BoxShadow, Option<&WidgetId>, Option<&TooltipEntity>),
            With<IsFamiqSelectionSelector>
        >,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        if let Ok((mut box_shadow, id, tooltip_entity)) = selector_q.get_mut(trigger.entity()) {
            box_shadow.color = Color::NONE;
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Selection, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
        mut writer: EventWriter<FaMouseEvent>,
        mut famiq_res: ResMut<FamiqResource>
    ) {
        if let Ok(id) = selector_q.get_mut(trigger.entity()) {
            // currently true, set back to false
            if let Some(state) = famiq_res.get_widget_focus_state(&trigger.entity()) {
                if state {
                    famiq_res.update_or_insert_focus_state(trigger.entity(), false);
                    return;
                }
            }
            // currently false, set back to true
            famiq_res.update_all_focus_states(false);
            famiq_res.update_or_insert_focus_state(trigger.entity(), true);

            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Selection, trigger.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Selection, trigger.entity(), id);
            }
        }
        trigger.propagate(false);
    }

    fn on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
        mut writer: EventWriter<FaMouseEvent>,
    ) {
        if let Ok(id) = selector_q.get_mut(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Selection, trigger.entity(), id);
        }
        trigger.propagate(false);
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
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_selector_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut selector = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let selector_entity = selector.build(r_data, commands);
        let (ph_entity, ph_text) = self.build_placeholder(commands, r_data, &mut all_reactive_keys);
        let arrow_entity = self.build_arrow(commands, r_data);
        let panel_entity = self.build_choices_panel(
            selector_entity,
            commands,
            r_data,
            &mut all_reactive_keys
        );

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
            .observe(SelectionBuilder::on_mouse_up)
            .observe(SelectionBuilder::on_mouse_down)
            .observe(SelectionBuilder::on_mouse_over)
            .observe(SelectionBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, selector_entity);
        }
        insert_class_id(commands, selector_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model(commands, selector_entity, &self.cloned_attrs.model_key);

        let cloned_builder = self.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                all_reactive_keys,
                selector_entity,
                WidgetBuilder {
                    builder: BuilderType::Selection(cloned_builder)
                }
            ));
        });
        selector_entity
    }

    fn build_with_world(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Option<Entity> {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_selector_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut selector = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let selector_entity = selector.build_with_world(r_data, world).unwrap();
        let (ph_entity, ph_text)= self.build_placeholder_world(world, r_data, &mut all_reactive_keys);
        let arrow_entity = self.build_arrow_world(world, r_data);
        let panel_entity = self.build_choices_panel_world(
            selector_entity,
            world,
            r_data,
            &mut all_reactive_keys
        );

        world.entity_mut(ph_entity).insert(SelectorEntity(selector_entity));

        world
            .entity_mut(selector_entity)
            .add_children(&[ph_entity, arrow_entity, panel_entity])
            .insert((
                self.components(),
                SelectionChoicesPanelEntity(panel_entity),
                SelectorPlaceHolderEntity(ph_entity),
                SelectorArrowIconEntity(arrow_entity),
                SelectorPlaceholderText(ph_text)
            ))
            .observe(SelectionBuilder::on_mouse_up)
            .observe(SelectionBuilder::on_mouse_down)
            .observe(SelectionBuilder::on_mouse_over)
            .observe(SelectionBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, &mut world.commands(), selector_entity);
        }
        insert_class_id_world(world, selector_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model_world(world, selector_entity, &self.cloned_attrs.model_key);

        let cloned_builder = self.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            all_reactive_keys,
            selector_entity,
            WidgetBuilder {
                builder: BuilderType::Selection(cloned_builder)
            }
        ));
        Some(selector_entity)
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

// pub struct FaSelection;

// impl<'a> FaSelection {
//     fn _build_selector_placeholder(
//         attributes: &WidgetAttributes,
//         placeholder: &str,
//         root_node: &'a mut EntityCommands,
//     ) -> Entity {
//         let class = &attributes.class.clone().unwrap_or("".into());
//         let id = &attributes.id.clone().unwrap_or("".into());
//         let layout = TextLayout::new(JustifyText::Left, LineBreak::NoWrap);
//         let entity = fa_text!(
//             text: placeholder,
//             id: id,
//             class: class,
//             has_node: false,
//             has_observer: false,
//             use_get_text_color: true,
//             text_layout: layout
//         );
//         root_node.commands().entity(entity).insert(SelectorPlaceHolder);
//         entity
//     }

//     fn _build_selector_arrow_icon(root_node: &'a mut EntityCommands) -> Entity {
//         let layout = TextLayout::new_with_justify(JustifyText::Right);
//         let entity = fa_text!(
//             text: "▼",
//             has_node: false,
//             has_observer: false,
//             use_get_text_color: true,
//             text_layout: layout
//         );
//         root_node.commands().entity(entity).insert(ArrowIcon);
//         entity
//     }

//     fn _build_selector(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands,
//         placeholder: &str,
//         placeholder_entity: Entity,
//         arrow_icon_entity: Entity
//     ) -> Entity {
//         let selection_color = get_color(&attributes.color);

//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = attributes.node.clone();
//         style_components.border_color = BorderColor(selection_color);
//         style_components.background_color = BackgroundColor(selection_color);
//         style_components.border_radius = BorderRadius::all(Val::Px(6.0));
//         style_components.visibility = Visibility::Visible;

//         let selector_entity = root_node
//             .commands()
//             .spawn((
//                 style_components.clone(),
//                 IsFamiqSelectionSelector,
//                 MainWidget,
//                 DefaultWidgetConfig::from(style_components),
//                 SelectionValue::default(),
//                 Selection::new(placeholder.to_string()),
//                 SelectorPlaceHolderEntity(placeholder_entity),
//                 SelectorArrowIconEntity(arrow_icon_entity)
//             ))
//             .observe(FaSelection::handle_on_mouse_over)
//             .observe(FaSelection::handle_on_mouse_out)
//             .observe(FaSelection::handle_on_mouse_down)
//             .observe(FaSelection::handle_on_mouse_up)
//             .id();

//         if attributes.has_tooltip {
//             build_tooltip_node(attributes, root_node, selector_entity);
//         }
//         if attributes.model_key.is_some() {
//             root_node
//                 .commands()
//                 .entity(selector_entity)
//                 .insert(ReactiveModelKey(attributes.model_key.clone().unwrap()));
//         }

//         insert_id_and_class(root_node, selector_entity, &attributes.id, &attributes.class);
//         selector_entity
//     }

//     fn _build_choices_panel(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands,
//         choices: &Vec<String>,
//         selector_entity: Entity
//     ) -> Entity {
//         let mut choice_entities: Vec<Entity> = Vec::new();
//         let mut all_choices = Vec::with_capacity(choices.len() + 1);
//         all_choices.push("-/-".to_string());
//         all_choices.extend_from_slice(choices);

//         for choice in all_choices.iter() {
//             let txt = Self::_build_choice_text(choice, root_node);
//             let container = Self::_build_choice_container(root_node, txt, selector_entity);
//             entity_add_child(root_node, txt, container);
//             choice_entities.push(container);
//         }
//         let selection_color = get_color(&attributes.color);
//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = default_selection_choices_panel_node();
//         style_components.border_color = BorderColor(selection_color);
//         style_components.background_color = BackgroundColor(selection_color);
//         style_components.border_radius = BorderRadius::all(Val::Px(5.0));

//         let panel = root_node
//             .commands()
//             .spawn((
//                 style_components,
//                 IsFamiqSelectionChoicesPanel,
//                 FocusPolicy::Block,
//                 GlobalZIndex(2),
//                 Transform::default()
//             ))
//             .id();

//         entity_add_children(root_node, &choice_entities, panel);
//         panel
//     }

//     fn _build_choice_container(
//         root_node: &'a mut EntityCommands,
//         text_entity: Entity,
//         selector_entity: Entity
//     ) -> Entity {
//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = default_choice_container_node();
//         style_components.border_radius = BorderRadius::all(Val::Px(5.0));

//         root_node
//             .commands()
//             .spawn((
//                 style_components,
//                 IsFamiqSelectionChoice,
//                 SelectionChoiceTextEntity(text_entity),
//                 SelectorEntity(selector_entity)
//             ))
//             .id()
//     }

//     fn _build_choice_text(choice: &str, root_node: &'a mut EntityCommands) -> Entity {
//         let entity = fa_text!(
//             text: choice,
//             has_node: false,
//             has_observer: false,
//             use_get_text_color: true
//         );
//         root_node
//             .commands()
//             .entity(entity)
//             .insert(Visibility::Inherited)
//             .remove::<DefaultTextConfig>();
//         entity
//     }

//     // return Entity of selector
//     pub fn new(
//         attributes: &WidgetAttributes,
//         placeholder: &str,
//         root_node: &'a mut EntityCommands,
//         choices: &Vec<String>,
//     ) -> Entity {
//         let placeholder_entity = Self::_build_selector_placeholder(
//             attributes,
//             placeholder,
//             root_node,
//         );
//         let arrow_icon_entity = Self::_build_selector_arrow_icon(root_node);
//         let selector = Self::_build_selector(
//             attributes,
//             root_node,
//             placeholder,
//             placeholder_entity,
//             arrow_icon_entity
//         );
//         let choices_panel = Self::_build_choices_panel(
//             attributes,
//             root_node,
//             choices,
//             selector
//         );
//         root_node.commands().entity(selector).insert(SelectionChoicesPanelEntity(choices_panel));
//         entity_add_children(root_node, &vec![placeholder_entity, arrow_icon_entity, choices_panel], selector);
//         selector
//     }

//     pub fn arrow_up(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
//         if let Ok(mut text) = text_q.get_mut(arrow_entity) {
//             text.0 = "▲".to_string()
//         }
//     }

//     pub fn arrow_down(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
//         if let Ok(mut text) = text_q.get_mut(arrow_entity) {
//             text.0 = "▼".to_string()
//         }
//     }

//     fn handle_on_mouse_over(
//         mut trigger: Trigger<Pointer<Over>>,
//         mut selector_q: Query<
//             (&mut BoxShadow, &BorderColor, Option<&WidgetId>, &GlobalTransform, Option<&TooltipEntity>),
//             With<IsFamiqSelectionSelector>
//         >,
//         mut commands: Commands,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         window: Single<Entity, With<Window>>,
//         cursor_icons: Res<CursorIcons>,
//     ) {
//         if let Ok((mut box_shadow, border_color, id, transform, tooltip_entity)) = selector_q.get_mut(trigger.entity()) {
//             box_shadow.color = border_color.0.clone();
//             show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
//             _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
//             FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Selection, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_out(
//         mut trigger: Trigger<Pointer<Out>>,
//         mut selector_q: Query<
//             (&mut BoxShadow, Option<&WidgetId>, Option<&TooltipEntity>),
//             With<IsFamiqSelectionSelector>
//         >,
//         mut commands: Commands,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         window: Single<Entity, With<Window>>,
//         cursor_icons: Res<CursorIcons>,
//     ) {
//         if let Ok((mut box_shadow, id, tooltip_entity)) = selector_q.get_mut(trigger.entity()) {
//             box_shadow.color = Color::NONE;
//             hide_tooltip(tooltip_entity, &mut tooltip_q);
//             _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
//             FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Selection, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_down(
//         mut trigger: Trigger<Pointer<Down>>,
//         mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut famiq_res: ResMut<FamiqResource>
//     ) {
//         if let Ok(id) = selector_q.get_mut(trigger.entity()) {
//             // currently true, set back to false
//             if let Some(state) = famiq_res.get_widget_focus_state(&trigger.entity()) {
//                 if state {
//                     famiq_res.update_or_insert_focus_state(trigger.entity(), false);
//                     return;
//                 }
//             }
//             // currently false, set back to true
//             famiq_res.update_all_focus_states(false);
//             famiq_res.update_or_insert_focus_state(trigger.entity(), true);

//             if trigger.event().button == PointerButton::Secondary {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Selection, trigger.entity(), id);
//             } else {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Selection, trigger.entity(), id);
//             }
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_up(
//         mut trigger: Trigger<Pointer<Up>>,
//         mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
//         mut writer: EventWriter<FaMouseEvent>,
//     ) {
//         if let Ok(id) = selector_q.get_mut(trigger.entity()) {
//             FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Selection, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }
// }

// pub struct FaSelectionBuilder<'a> {
//     pub attributes: WidgetAttributes,
//     pub placeholder: String,
//     pub choices: Vec<String>,
//     pub root_node: EntityCommands<'a>
// }

// impl<'a> FaSelectionBuilder<'a> {
//     pub fn new(
//         placeholder: String,
//         font_handle: Handle<Font>,
//         root_node: EntityCommands<'a>
//     ) -> Self {
//         let mut attributes = WidgetAttributes::default();
//         attributes.font_handle = Some(font_handle);
//         Self {
//             attributes,
//             placeholder,
//             choices: Vec::new(),
//             root_node
//         }
//     }

//     pub fn choices<I>(mut self, choices: I) -> Self
//     where
//         I: IntoIterator,
//         I::Item: Into<String>,
//     {
//         self.choices = choices.into_iter().map(Into::into).collect();
//         self
//     }

//     pub fn build(&mut self) -> Entity {
//         self._process_built_in_size_class();
//         self._process_built_in_color_class();
//         self._node();
//         FaSelection::new(
//             &self.attributes,
//             self.placeholder.as_str(),
//             &mut self.root_node,
//             &self.choices
//         )
//     }
// }

// impl<'a> SetWidgetAttributes for FaSelectionBuilder<'a> {
//     fn attributes(&mut self) -> &mut WidgetAttributes {
//         &mut self.attributes
//     }

//     fn _node(&mut self) {
//         self.attributes.node = default_selector_node();
//         if self.attributes.default_display_changed {
//             self.attributes.node.display = self.attributes.default_display;
//         }
//         process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
//     }
// }

// pub fn fa_selection_builder<'a>(
//     builder: &'a mut FamiqBuilder,
//     placeholder: &str
// ) -> FaSelectionBuilder<'a> {
//     let font_handle = builder.asset_server.load(&builder.resource.font_path);
//     FaSelectionBuilder::new(
//         placeholder.to_string(),
//         font_handle,
//         builder.ui_root_node.reborrow()
//     )
// }

// #[macro_export]
// macro_rules! fa_selection {
//     (
//         model: $model:expr,
//         placeholder: $placeholder:expr
//         $(, $key:ident : $value:tt )* $(,)?
//     ) => {{
//         let builder = builder_mut();
//         let mut selection = fa_selection_builder(builder, $placeholder);
//         selection = selection.model($model);
//         $(
//             $crate::fa_selection_attributes!(selection, $key : $value);
//         )*
//         selection.build()
//     }};

//     ( $( $tt:tt )* ) => {
//         panic!("\n[FamiqError]: fa_selection! requires model field.\n");
//     };
// }

// #[macro_export]
// macro_rules! fa_selection_attributes {
//     ($selection:ident, choices: $choices:expr) => {{
//         $selection = $selection.choices($choices);
//     }};

//     ($selection:ident, tooltip: $tooltip:expr) => {{
//         $selection = $selection.tooltip($tooltip);
//     }};

//     // common attributes
//     ($selection:ident, $key:ident : $value:expr) => {{
//         $crate::common_attributes!($selection, $key : $value);
//     }};
// }

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionSelector>) -> bool {
    !selection_q.is_empty()
}
