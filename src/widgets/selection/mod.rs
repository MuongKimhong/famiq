pub mod components;
pub mod styling;
pub mod systems;
pub mod tests;

use crate::fa_text;
use crate::resources::*;
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::*;
use crate::plugin::{CursorType, CursorIcons};
use bevy::ecs::system::EntityCommands;
use bevy::ui::FocusPolicy;
use bevy::prelude::*;

pub use components::*;
pub use styling::*;
pub use systems::*;

pub struct FaSelection;

impl<'a> FaSelection {
    fn _build_selector_placeholder(
        attributes: &WidgetAttributes,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let class = &attributes.class.clone().unwrap_or("".into());
        let id = &attributes.id.clone().unwrap_or("".into());
        let layout = TextLayout::new(JustifyText::Left, LineBreak::NoWrap);
        let entity = fa_text!(
            text: placeholder,
            id: id,
            class: class,
            has_node: false,
            has_observer: false,
            use_get_text_color: true,
            text_layout: layout
        );
        root_node.commands().entity(entity).insert(SelectorPlaceHolder);
        entity
    }

    fn _build_selector_arrow_icon(root_node: &'a mut EntityCommands) -> Entity {
        let layout = TextLayout::new_with_justify(JustifyText::Right);
        let entity = fa_text!(
            text: "▼",
            has_node: false,
            has_observer: false,
            use_get_text_color: true,
            text_layout: layout
        );
        root_node.commands().entity(entity).insert(ArrowIcon);
        entity
    }

    fn _build_selector(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
        placeholder_entity: Entity,
        arrow_icon_entity: Entity
    ) -> Entity {
        let selection_color = get_color(&attributes.color);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.border_color = BorderColor(selection_color);
        style_components.background_color = BackgroundColor(selection_color);
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));
        style_components.visibility = Visibility::Visible;

        let selector_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqSelectionSelector,
                MainWidget,
                DefaultWidgetConfig::from(style_components),
                SelectionValue::default(),
                Selection::new(placeholder.to_string()),
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity)
            ))
            .observe(FaSelection::handle_on_mouse_over)
            .observe(FaSelection::handle_on_mouse_out)
            .observe(FaSelection::handle_on_mouse_down)
            .observe(FaSelection::handle_on_mouse_up)
            .id();

        if attributes.has_tooltip {
            build_tooltip_node(attributes, root_node, selector_entity);
        }
        if attributes.model_key.is_some() {
            root_node
                .commands()
                .entity(selector_entity)
                .insert(ReactiveModelKey(attributes.model_key.clone().unwrap()));
        }

        insert_id_and_class(root_node, selector_entity, &attributes.id, &attributes.class);
        selector_entity
    }

    fn _build_choices_panel(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
        selector_entity: Entity
    ) -> Entity {
        let mut choice_entities: Vec<Entity> = Vec::new();
        let mut all_choices = Vec::with_capacity(choices.len() + 1);
        all_choices.push("-/-".to_string());
        all_choices.extend_from_slice(choices);

        for choice in all_choices.iter() {
            let txt = Self::_build_choice_text(choice, root_node);
            let container = Self::_build_choice_container(root_node, txt, selector_entity);
            entity_add_child(root_node, txt, container);
            choice_entities.push(container);
        }
        let selection_color = get_color(&attributes.color);
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_selection_choices_panel_node();
        style_components.border_color = BorderColor(selection_color);
        style_components.background_color = BackgroundColor(selection_color);
        style_components.border_radius = BorderRadius::all(Val::Px(5.0));

        let panel = root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqSelectionChoicesPanel,
                FocusPolicy::Block,
                GlobalZIndex(2),
                Transform::default()
            ))
            .id();

        entity_add_children(root_node, &choice_entities, panel);
        panel
    }

    fn _build_choice_container(
        root_node: &'a mut EntityCommands,
        text_entity: Entity,
        selector_entity: Entity
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_choice_container_node();
        style_components.border_radius = BorderRadius::all(Val::Px(5.0));

        root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                SelectorEntity(selector_entity)
            ))
            .id()
    }

    fn _build_choice_text(choice: &str, root_node: &'a mut EntityCommands) -> Entity {
        let entity = fa_text!(
            text: choice,
            has_node: false,
            has_observer: false,
            use_get_text_color: true
        );
        root_node
            .commands()
            .entity(entity)
            .insert(Visibility::Inherited)
            .remove::<DefaultTextConfig>();
        entity
    }

    // return Entity of selector
    pub fn new(
        attributes: &WidgetAttributes,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
    ) -> Entity {
        let placeholder_entity = Self::_build_selector_placeholder(
            attributes,
            placeholder,
            root_node,
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(root_node);
        let selector = Self::_build_selector(
            attributes,
            root_node,
            placeholder,
            placeholder_entity,
            arrow_icon_entity
        );
        let choices_panel = Self::_build_choices_panel(
            attributes,
            root_node,
            choices,
            selector
        );
        root_node.commands().entity(selector).insert(SelectionChoicesPanelEntity(choices_panel));
        entity_add_children(root_node, &vec![placeholder_entity, arrow_icon_entity, choices_panel], selector);
        selector
    }

    pub fn arrow_up(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
        if let Ok(mut text) = text_q.get_mut(arrow_entity) {
            text.0 = "▲".to_string()
        }
    }

    pub fn arrow_down(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
        if let Ok(mut text) = text_q.get_mut(arrow_entity) {
            text.0 = "▼".to_string()
        }
    }

    fn handle_on_mouse_over(
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

    fn handle_on_mouse_out(
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

    fn handle_on_mouse_down(
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

    fn handle_on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
        mut writer: EventWriter<FaMouseEvent>,
    ) {
        if let Ok(id) = selector_q.get_mut(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Selection, trigger.entity(), id);
        }
        trigger.propagate(false);
    }
}

pub struct FaSelectionBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub placeholder: String,
    pub choices: Vec<String>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaSelectionBuilder<'a> {
    pub fn new(
        placeholder: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>
    ) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            placeholder,
            choices: Vec::new(),
            root_node
        }
    }

    pub fn choices<I>(mut self, choices: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.choices = choices.into_iter().map(Into::into).collect();
        self
    }

    pub fn build(&mut self) -> Entity {
        self._process_built_in_size_class();
        self._process_built_in_color_class();
        self._node();
        FaSelection::new(
            &self.attributes,
            self.placeholder.as_str(),
            &mut self.root_node,
            &self.choices
        )
    }
}

impl<'a> SetWidgetAttributes for FaSelectionBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_selector_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

pub fn fa_selection_builder<'a>(
    builder: &'a mut FamiqBuilder,
    placeholder: &str
) -> FaSelectionBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaSelectionBuilder::new(
        placeholder.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

#[macro_export]
macro_rules! fa_selection {
    (
        model: $model:expr,
        placeholder: $placeholder:expr
        $(, $key:ident : $value:tt )* $(,)?
    ) => {{
        let builder = builder_mut();
        let mut selection = fa_selection_builder(builder, $placeholder);
        selection = selection.model($model);
        $(
            $crate::fa_selection_attributes!(selection, $key : $value);
        )*
        selection.build()
    }};

    ( $( $tt:tt )* ) => {
        panic!("\n[FamiqError]: fa_selection! requires model field.\n");
    };
}

#[macro_export]
macro_rules! fa_selection_attributes {
    ($selection:ident, choices: $choices:expr) => {{
        $selection = $selection.choices($choices);
    }};

    ($selection:ident, tooltip: $tooltip:expr) => {{
        $selection = $selection.tooltip($tooltip);
    }};

    // common attributes
    ($selection:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($selection, $key : $value);
    }};
}

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionSelector>) -> bool {
    !selection_q.is_empty()
}
