pub mod components;
pub mod helper;
pub mod styling;
pub mod systems;
pub mod tests;

use crate::utils::*;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity,
    FamiqWidgetId, FamiqBuilder, WidgetStyle,
    ExternalStyleHasChanged, ResourceMap, BaseStyleComponents
};
use bevy::ecs::system::EntityCommands;
use bevy::ui::FocusPolicy;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use components::*;
pub use helper::*;

use super::color::{BLACK_COLOR, WHITE_COLOR};
pub use styling::*;
pub use systems::*;

#[derive(Resource, Default, Debug)]
pub struct FaSelectionResource {
    pub choices_id: HashMap<String, String>, // id - choice
    pub choices_entity: HashMap<Entity, String> // entity - choice
}

/// Implement the trait for FaSelectionResource
impl ResourceMap for FaSelectionResource {
    fn _insert_by_id(&mut self, id: String, value: String) {
        self.choices_id.insert(id, value);
    }

    fn _insert_by_entity(&mut self, entity: Entity, value: String) {
        self.choices_entity.insert(entity, value);
    }

    fn get_value_by_id(&self, id: &str) -> String {
        self.choices_id.get(id).map_or_else(
            || String::from(""),
            |v| if v == "-/-" { String::from("") } else { v.clone() },
        )
    }

    fn get_value_by_entity(&self, entity: Entity) -> String {
        self.choices_entity.get(&entity).map_or_else(
            || String::from(""),
            |v| if v == "-/-" { String::from("") } else { v.clone() },
        )
    }

    fn exists_by_id(&self, id: &str) -> bool {
        self.choices_id.contains_key(id)
    }

    fn exists_by_entity(&self, entity: Entity) -> bool {
        self.choices_entity.contains_key(&entity)
    }
}

pub enum SelectorVariant {
    Outlined,
    Default,
    Underlined,
}

#[derive(PartialEq)]
pub enum SelectorColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

pub enum SelectorShape {
    Default,
    Round,
    Rectangle
}

pub enum SelectionSize {
    Small,
    Normal,
    Large,
}

pub struct FaSelection;

// Needs container
impl<'a> FaSelection {
    fn _build_container(root_node: &'a mut EntityCommands) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_selection_container_node();
        style_components.visibility = Visibility::Visible;

        root_node
            .commands()
            .spawn((style_components, IsFamiqSelectionContainer))
            .id()
    }

    fn _build_selector_placeholder(
        id: &Option<String>,
        class: &Option<String>,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &SelectionSize,
        color: &SelectorColor
    ) -> Entity {
        let mut use_color = WHITE_COLOR;

        if *color == SelectorColor::Default {
            use_color = BLACK_COLOR;
        }
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(&size),
            ..default()
        };

        let entity = root_node
            .commands()
            .spawn((
                Text::new(placeholder),
                txt_font.clone(),
                TextColor(use_color),
                TextLayout::new_with_justify(JustifyText::Left),
                SelectorPlaceHolder,
                DefaultTextEntity::new(
                    Text::new(placeholder),
                    txt_font,
                    TextColor(use_color),
                    TextLayout::new_with_justify(JustifyText::Left)
                ),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        insert_id_and_class(root_node, entity, id, class);
        entity
    }

    fn _build_selector_arrow_icon(
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &SelectionSize,
        color: &SelectorColor
    ) -> Entity {
        let mut use_color = WHITE_COLOR;

        if *color == SelectorColor::Default {
            use_color = BLACK_COLOR;
        }
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(&size),
            ..default()
        };

        root_node
            .commands()
            .spawn((
                Text::new("▼"),
                txt_font,
                TextColor(use_color),
                TextLayout::new_with_justify(JustifyText::Right),
                ArrowIcon
            ))
            .id()
    }

    fn _build_selector(
        id: &Option<String>,
        class: &Option<String>,
        root_node: &'a mut EntityCommands,
        variant: &SelectorVariant,
        shape: &SelectorShape,
        color: &SelectorColor,
        placeholder: &str,
        placeholder_entity: Entity,
        arrow_icon_entity: Entity
    ) -> Entity {
        let mut border_width = UiRect::all(Val::Px(2.0));
        let mut border_radius = outlined_border_radius();

        match shape {
            SelectorShape::Round => border_radius = round_border_radius(),
            SelectorShape::Rectangle => border_radius = rectangle_border_radius(),
            _ => ()
        }
        match variant {
            SelectorVariant::Underlined => {
                border_radius = underlined_border_radius();
                border_width = UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(2.0),
                }
            }
            _ => ()
        }

        let mut node = default_selector_node(border_width);
        process_spacing_built_in_class(&mut node, class);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = node;
        style_components.border_color = get_selector_border_color(color);
        style_components.background_color = get_selector_background_color(color);
        style_components.border_radius = border_radius;
        style_components.visibility = Visibility::Visible;

        let selector_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqSelectionSelector,
                DefaultWidgetEntity::from(style_components),
                Selection::new(placeholder.to_string()),
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity)
            ))
            .id();

        insert_id_and_class(root_node, selector_entity, id, class);
        selector_entity
    }

    fn _build_choices_panel(
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
        font_handle: Handle<Font>,
        container_entity: Entity,
        color: &SelectorColor,
        selector_entity: Entity
    ) -> Entity {
        let mut choice_entities: Vec<Entity> = Vec::new();
        let mut all_choices = Vec::with_capacity(choices.len() + 1);
        all_choices.push("-/-".to_string());
        all_choices.extend_from_slice(choices);

        for choice in all_choices.iter() {
            let txt = Self::_build_choice_text(choice, root_node, &font_handle, color);
            let container = Self::_build_choice_container(root_node, txt, selector_entity);
            entity_add_child(root_node, txt, container);
            choice_entities.push(container);
        }

        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_selection_choices_panel_node();
        style_components.border_color = get_choice_panel_border_color(color);
        style_components.background_color = get_choice_panel_background_color(color);
        style_components.visibility = Visibility::Hidden;
        style_components.border_radius = BorderRadius::all(Val::Px(5.0));

        let panel = root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqSelectionChoicesPanel,
                SelectionContainerEntity(container_entity),
                FocusPolicy::Block,
                GlobalZIndex(2)
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

    fn _build_choice_text(
        choice: &str,
        root_node: &'a mut EntityCommands,
        font_handle: &Handle<Font>,
        color: &SelectorColor
    ) -> Entity {
        let txt_font = TextFont {
            font: font_handle.clone(),
            ..default()
        };
        let mut txt_color = TextColor(WHITE_COLOR);
        if *color == SelectorColor::Default {
            txt_color = TextColor(BLACK_COLOR);
        }
        root_node
            .commands()
            .spawn((
                Text::new(choice),
                txt_font,
                txt_color,
                TextLayout::new_with_justify(JustifyText::Center),
                Visibility::Inherited,
            ))
            .id()
    }

    // return Entity of selector
    pub fn new(
        id: Option<String>,
        class: Option<String>,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        variant: SelectorVariant,
        color: SelectorColor,
        size: SelectionSize,
        shape: SelectorShape,
        choices: &Vec<String>,
    ) -> Entity {
        let container = Self::_build_container(root_node);
        let placeholder_entity = Self::_build_selector_placeholder(
            &id,
            &class,
            placeholder,
            root_node,
            font_handle.clone(),
            &size,
            &color
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(root_node, font_handle.clone(), &size, &color);
        let selector = Self::_build_selector(
            &id,
            &class,
            root_node,
            &variant,
            &shape,
            &color,
            placeholder,
            placeholder_entity,
            arrow_icon_entity
        );
        let choices_panel = Self::_build_choices_panel(
            root_node,
            choices,
            font_handle.clone(),
            container,
            &color,
            selector
        );

        root_node.commands().entity(selector).insert(SelectionChoicesPanelEntity(choices_panel));

        entity_add_children(root_node, &vec![placeholder_entity, arrow_icon_entity], selector);
        entity_add_children(root_node, &vec![selector, choices_panel], container);

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

    pub fn show_choice_panel(
        panel_q: &mut Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,
        panel_entity: Entity
    ) {
        if let Ok(mut visibility) = panel_q.get_mut(panel_entity) {
            *visibility = Visibility::Visible;
        }
    }

    pub fn hide_choice_panel(
        panel_q: &mut Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,
        panel_entity: Entity
    ) {
        if let Ok(mut visibility) = panel_q.get_mut(panel_entity) {
            *visibility = Visibility::Hidden;
        }
    }

    pub fn set_placeholder_color(
        is_focused: bool,
        text_q: &mut Query<(&mut TextColor, &WidgetStyle), With<SelectorPlaceHolder>>,
        placeholder_entity: Entity,
        selector_bg_color: &Color
    ) {
        if let Ok((mut text_color, widget_style)) = text_q.get_mut(placeholder_entity) {
            // can update text color only if no external style set to placeholder.
            if widget_style.color.is_some() {
                return;
            }

            if is_focused {
                if *selector_bg_color == WHITE_COLOR {
                    text_color.0 = BLACK_COLOR
                } else {
                    text_color.0 = PLACEHOLDER_COLOR_FOCUSED;
                }
            } else {
                text_color.0 = PLACEHOLDER_COLOR_UNFOCUSED;
            }
        }
    }
}

pub struct FaSelectionBuilder<'a> {
    pub id: Option<String>,
    pub placeholder: String,
    pub class: Option<String>,
    pub choices: Option<Vec<String>>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaSelectionBuilder<'a> {
    pub fn new(
        placeholder: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>
    ) -> Self {
        Self {
            id: None,
            placeholder,
            class: None,
            choices: Some(Vec::new()),
            font_handle,
            root_node
        }
    }

    fn _process_built_in_classes(&self) -> (SelectorColor, SelectorVariant, SelectorShape, SelectionSize) {
        let mut use_color = SelectorColor::Default;
        let mut use_size = SelectionSize::Normal;
        let mut use_shape = SelectorShape::Default;
        let mut use_variant = SelectorVariant::Default;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-underlined" => use_variant = SelectorVariant::Underlined,
                    "is-outlined" => use_variant = SelectorVariant::Outlined,

                    "is-small" => use_size = SelectionSize::Small,
                    "is-large" => use_size = SelectionSize::Large,

                    "is-round" => use_shape = SelectorShape::Round,
                    "is-rectangle" => use_shape = SelectorShape::Rectangle,

                    "is-primary" => use_color = SelectorColor::Primary,
                    "is-secondary" => use_color = SelectorColor::Secondary,
                    "is-danger" => use_color = SelectorColor::Danger,
                    "is-success" => use_color = SelectorColor::Success,
                    "is-warning" => use_color = SelectorColor::Warning,
                    "is-info" => use_color = SelectorColor::Info,

                    _ => ()
                }
            }
        }
        (use_color, use_variant, use_shape, use_size)
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn choices<I>(mut self, choices: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.choices = Some(choices.into_iter().map(Into::into).collect());
        self
    }

    pub fn build(&mut self) -> Entity {
        let (color, variant, shape, size) = self._process_built_in_classes();
        FaSelection::new(
            self.id.clone(),
            self.class.clone(),
            self.placeholder.as_str(),
            &mut self.root_node,
            self.font_handle.clone(),
            variant,
            color,
            size,
            shape,
            self.choices.as_ref().unwrap()
        )
    }
}

pub fn fa_selection<'a>(
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

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionContainer>) -> bool {
    !selection_q.is_empty()
}
