pub mod components;
pub mod helper;
pub mod styling;
pub mod systems;

use crate::utils;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use components::*;
pub use helper::*;

use super::color::WHITE_COLOR;
pub use styling::*;
pub use systems::*;

#[derive(Resource, Debug)]
pub struct SelectedChoicesResource {
    pub choices: HashMap<String, String>, // id - choice
}

impl SelectedChoicesResource {
    pub fn update_or_insert(&mut self, id: String, selected_choice: String) {
        if let Some(item_value) = self.choices.get_mut(&id) {
            *item_value = selected_choice;
        } else {
            self.choices.insert(id, selected_choice);
        }
    }
}

pub enum SelectorVariant {
    Outlined,
    Default,
    Underlined,
}

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
    fn _build_container(id: &str, classes: &str, root_node: &'a mut EntityCommands) -> Entity {
        let node = default_selection_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(id.to_string()),
                FamiqWidgetClasses(classes.to_string()),
                IsFamiqSelectionContainer,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default()
            ))
            .id()
    }

    fn _build_selector_placeholder(
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: &SelectionSize,
    ) -> Entity {
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR_UNFOCUSED);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Left);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                // FamiqWidgetId(format!("{id}_selection_placeholder")),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
            ))
            .id()
    }

    fn _build_selector_arrow_icon(
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: &SelectionSize,
    ) -> Entity {
        let txt = Text::new("▼");
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR_UNFOCUSED);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Right);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
            ))
            .id()
    }

    fn _build_selector(
        id: &str,
        root_node: &'a mut EntityCommands,
        variant: &SelectorVariant,
        shape: &SelectorShape,
        color: &SelectorColor,
        placeholder: &str,
        placeholder_entity: Entity,
        arrow_icon_entity: Entity,
        choices_panel_entity: Entity,
        label_entity: Option<Entity>
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
        let node = default_selector_node(border_width);
        let border_color = get_selector_border_color(color);
        let bg_color = get_selector_background_color(color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let selector_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(format!("{id}_selection_selector")),
                IsFamiqSelectionSelector,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                Selection::new(placeholder.to_string()),
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity),
                SelectionChoicesPanelEntity(choices_panel_entity)
            ))
            .id();

        if let Some(label) = label_entity {
            root_node
                .commands()
                .entity(selector_entity)
                .insert(SelectionLabelEntity(label));
        }
        selector_entity
    }

    fn _build_choices_panel(
        id: &str,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        container_entity: Entity,
        color: &SelectorColor
    ) -> Entity {
        let node = default_selection_choices_panel_node();
        let border_radius = BorderRadius::all(Val::Px(5.0));
        let border_color = get_choice_panel_border_color(color);
        let bg_color = get_choice_panel_background_color(color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Hidden;

        let mut choice_entities: Vec<Entity> = Vec::new();
        for choice in choices.iter() {
            let txt = Self::_build_choice_text(id, choice, root_node, asset_server, font_path);
            let container = Self::_build_choice_container(id, root_node, txt);
            utils::entity_add_child(root_node, txt, container);
            choice_entities.push(container);
        }

        let panel = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(format!("{id}_selection_choice_panel")),
                IsFamiqSelectionChoicesPanel,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                SelectionContainerEntity(container_entity),
                GlobalZIndex(2)
            ))
            .id();

        utils::entity_add_children(root_node, &choice_entities, panel);
        panel
    }

    fn _build_choice_container(id: &str, root_node: &'a mut EntityCommands, text_entity: Entity) -> Entity {
        let node = default_choice_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::all(Val::Px(5.0));
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(format!("{id}_selection_choice_container")),
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                Interaction::default()
            ))
            .id()
    }

    fn _build_choice_text(
        id: &str,
        choice: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let txt = Text::new(choice);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            ..default()
        };
        let txt_color = TextColor(WHITE_COLOR);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                FamiqWidgetId(format!("{id}_selection_choice_text")),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                Visibility::Inherited,
            ))
            .id()
    }

    fn _build_label(
        id: &str,
        label: &str,
        size: &SelectionSize,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let txt = Text::new(label);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(WHITE_COLOR);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                FamiqWidgetId(format!("{id}_selection_label")),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                Node::default(),
                IsFamiqSelectionLabel
            ))
            .id()
    }

    // return Entity of Container (Selection refers to container itself)
    pub fn new(
        id: &str,
        classes: &str,
        placeholder: &str,
        label: Option<&str>,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        variant: SelectorVariant,
        color: SelectorColor,
        size: SelectionSize,
        shape: SelectorShape,
        choices: &Vec<String>,
    ) -> Entity {
        let mut label_entity = None;
        let container = Self::_build_container(id, classes, root_node);

        if let Some(label_txt) = label {
            let label_ = Self::_build_label(id, label_txt, &size, root_node, asset_server, font_path);
            label_entity = Some(label_);
            utils::entity_add_child(root_node, label_, container);
        }

        let choices_panel = Self::_build_choices_panel(
            id,
            root_node,
            choices,
            asset_server,
            font_path,
            container,
            &color
        );

        let placeholder_entity = Self::_build_selector_placeholder(
            placeholder,
            root_node,
            asset_server,
            font_path,
            &size,
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(root_node, asset_server, font_path, &size);
        let selector = Self::_build_selector(
            id,
            root_node,
            &variant,
            &shape,
            &color,
            placeholder,
            placeholder_entity,
            arrow_icon_entity,
            choices_panel,
            label_entity
        );
        utils::entity_add_children(root_node, &vec![placeholder_entity, arrow_icon_entity], selector);
        utils::entity_add_children(root_node, &vec![selector, choices_panel], container);

        container
    }
}
