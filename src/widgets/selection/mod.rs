pub mod components;
pub mod helper;
pub mod styling;
pub mod systems;
pub mod tests;

use crate::utils::*;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity,
    FamiqWidgetId, FamiqWidgetBuilder, WidgetStyle,
    ExternalStyleHasChanged, ResourceMap
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
    pub choices: HashMap<String, String>, // id - choice
}

/// Implement the trait for FaSelectionResource
impl ResourceMap for FaSelectionResource {
    fn _update_or_insert(&mut self, id: String, value: String) {
        self.choices.insert(id, value);
    }

    fn get_value(&self, id: &str) -> String {
        self.choices.get(id).map_or_else(
            || String::from(""),
            |v| if v == "-/-" { String::from("") } else { v.clone() },
        )
    }

    fn exists(&self, id: &str) -> bool {
        self.choices.contains_key(id)
    }
}

// impl FaSelectionResource {
//     pub fn _update_or_insert(&mut self, id: String, selected_choice: String) {
//         self.choices.insert(id, selected_choice);
//     }

//     /// Get selection value by id
//     pub fn get_value(&self, id: &str) -> String {
//         if let Some(v) = self.choices.get(id) {
//             if v == "-/-" {
//                 return String::from("")
//             } else {
//                 v.to_owned()
//             }
//         } else {
//             String::from("")
//         }
//     }

//     /// Check if selection id exists in resource
//     pub fn exists(&self, id: &str) -> bool {
//         self.choices.get(id).is_some()
//     }
// }

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
        root_node
            .commands()
            .spawn((
                default_selection_container_node(),
                BorderColor::default(),
                BorderRadius::default(),
                BackgroundColor::default(),
                ZIndex::default(),
                Visibility::Visible,
                IsFamiqSelectionContainer,
                DefaultWidgetEntity::new(
                    default_selection_container_node(),
                    BorderColor::default(),
                    BorderRadius::default(),
                    BackgroundColor::default(),
                    ZIndex::default(),
                    Visibility::Visible,
                ),
                Interaction::default(),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id()
    }

    fn _build_selector_placeholder(
        id: &Option<String>,
        class: &Option<String>,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &SelectionSize,
    ) -> Entity {
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
                TextColor(PLACEHOLDER_COLOR_UNFOCUSED),
                TextLayout::new_with_justify(JustifyText::Left),
                SelectorPlaceHolder,
                DefaultTextEntity::new(
                    Text::new(placeholder),
                    txt_font,
                    TextColor(PLACEHOLDER_COLOR_UNFOCUSED),
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
    ) -> Entity {
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
                TextColor(PLACEHOLDER_COLOR_UNFOCUSED),
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

        let selector_entity = root_node
            .commands()
            .spawn((
                default_selector_node(border_width),
                get_selector_border_color(color),
                border_radius.clone(),
                get_selector_background_color(color),
                ZIndex::default(),
                Visibility::Visible,
                BoxShadow {
                    color: Color::NONE,
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(0.5),
                    blur_radius: Val::Px(1.0)
                },
                IsFamiqSelectionSelector,
                DefaultWidgetEntity::new(
                    default_selector_node(border_width),
                    get_selector_border_color(color),
                    border_radius,
                    get_selector_background_color(color),
                    ZIndex::default(),
                    Visibility::Visible,
                ),
                Interaction::default(),
                Selection::new(placeholder.to_string()),
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity)
            ))
            .insert((WidgetStyle::default(), ExternalStyleHasChanged(false)))
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

        let panel = root_node
            .commands()
            .spawn((
                default_selection_choices_panel_node(),
                get_choice_panel_border_color(color),
                BorderRadius::all(Val::Px(5.0)),
                get_choice_panel_background_color(color),
                ZIndex::default(),
                Visibility::Hidden,
                IsFamiqSelectionChoicesPanel,
                SelectionContainerEntity(container_entity),
                GlobalZIndex(2),
                FocusPolicy::Block
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
        root_node
            .commands()
            .spawn((
                default_choice_container_node(),
                BorderColor::default(),
                BorderRadius::all(Val::Px(5.0)),
                BackgroundColor::default(),
                ZIndex::default(),
                Visibility::Inherited,
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                Interaction::default(),
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

    // return Entity of Container (Selection refers to container itself)
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
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(root_node, font_handle.clone(), &size);
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

        container
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
    builder: &'a mut FamiqWidgetBuilder,
    placeholder: &str
) -> FaSelectionBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaSelectionBuilder::new(
        placeholder.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionContainer>) -> bool {
    selection_q.iter().count() > 0
}
