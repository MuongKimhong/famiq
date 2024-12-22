pub mod components;
pub mod helper;
pub mod styling;
pub mod systems;

use crate::utils;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId};
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

pub enum SelectionSize {
    Small,
    Normal,
    Large,
}

pub struct FaSelection;

impl<'a> FaSelection {
    fn _build_container(id: &str, root_node: &'a mut EntityCommands) -> Entity {
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
        id: &str,
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
        id: &str,
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
        placeholder: &str,
        placeholder_entity: Entity,
        arrow_icon_entity: Entity,
        choices_panel_entity: Entity,
        label_entity: Option<Entity>
    ) -> Entity {
        let border_width;
        let border_radius;

        match variant {
            SelectorVariant::Underlined => {
                border_width = underlined_border_width();
                border_radius = underlined_border_radius();
            }
            _ => {
                border_width = outlined_border_width();
                border_radius = outlined_border_radius();
            }
        }
        let node = default_selector_node(border_width);
        let border_color = BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922));
        let bg_color = BackgroundColor::default();
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
        container_entity: Entity
    ) -> Entity {
        let node = default_selection_choices_panel_node();
        let border_color = BorderColor(PANEL_BG_COLOR);
        let border_radius = BorderRadius::all(Val::Px(5.0));
        let bg_color = BackgroundColor(PANEL_BG_COLOR);
        let z_index = ZIndex(100);
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
                SelectionContainerEntity(container_entity)
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
                Visibility::Inherited
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
        placeholder: &str,
        label: Option<&str>,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        variant: SelectorVariant,
        size: SelectionSize,
        choices: &Vec<String>,
    ) -> Entity {
        let mut label_entity = None;
        let container = Self::_build_container(id, root_node);

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
            container
        );

        let placeholder_entity = Self::_build_selector_placeholder(
            id,
            placeholder,
            root_node,
            asset_server,
            font_path,
            &size,
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(id, root_node, asset_server, font_path, &size);
        let selector = Self::_build_selector(
            id,
            root_node,
            &variant,
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

    // pub fn set_to_unfocus(
    //     selection_q: &mut Query<&mut Selection>,
    //     selection_entity: Entity,
    //     for_all: bool,
    // ) {
    //     if for_all {
    //         handle_unfocus_selection_all(selection_q);
    //     } else {
    //         handle_unfocus_selection_one(selection_q, selection_entity);
    //     }
    // }

    // pub fn set_to_focus(selection_q: &mut Query<&mut Selection>, selection_entity: Entity) {
    //     if let Ok(mut selection) = selection_q.get_mut(selection_entity) {
    //         selection.focused = true;
    //     }
    // }
}

// // dont' need container
// impl<'a> FaSelection {
//     pub fn _build_selection_container(
//         selection_id: &str,
//         root_node: &'a mut EntityCommands,
//         selector: Entity,
//         panel: Entity,
//         label: Option<Entity>,
//     ) -> Entity {
//         let mut children = vec![selector, panel];

//         if let Some(label_entity) = label {
//             children.insert(0, label_entity);
//         }

//         let container_bundle = default_selection_container_bundle();
//         let container_entity = root_node
//             .commands()
//             .spawn((
//                 container_bundle.clone(),
//                 FamiqWidgetId(format!("{selection_id}_selection_container")),
//                 IsFamiqSelectionContainer,
//                 DefaultWidgetBundle(container_bundle),
//             ))
//             .id();

//         root_node.add_child(container_entity);
//         utils::entity_push_children(root_node, &children, container_entity);
//         container_entity
//     }

//     pub fn _build_selection_label(
//         selection_id: &str,
//         root_node: &'a mut EntityCommands,
//         asset_server: &'a ResMut<'a, AssetServer>,
//         font_path: &String,
//         size: &Option<SelectionSize>,
//         label: &str,
//     ) -> Entity {
//         let label_bundle = default_selection_label_bundle();
//         let text_bunle = create_selection_label_text(label, size, asset_server, font_path);

//         let text_entity = root_node
//             .commands()
//             .spawn((
//                 text_bunle,
//                 DefaultTextBundle(create_selection_label_text(
//                     label,
//                     size,
//                     asset_server,
//                     font_path,
//                 )),
//             ))
//             .id();

//         let label_entity = root_node
//             .commands()
//             .spawn((
//                 label_bundle.clone(),
//                 FamiqWidgetId(format!("{selection_id}_selection_label")),
//                 IsFamiqSelectionLabel,
//                 DefaultWidgetBundle(label_bundle),
//             ))
//             .id();

//         utils::entity_add_child(root_node, text_entity, label_entity);
//         label_entity
//     }

//     pub fn _build_selection_panel(
//         selection_id: &str,
//         items: &Vec<String>,
//         root_node: &'a mut EntityCommands,
//         asset_server: &'a ResMut<'a, AssetServer>,
//         font_path: &String,
//     ) -> Entity {
//         let items = create_items_text(selection_id, items, root_node, asset_server, font_path);

//         let panel_bundle = default_selection_items_panel_bundle();
//         let panel_entity = root_node
//             .commands()
//             .spawn((
//                 panel_bundle.clone(),
//                 FamiqWidgetId(format!("{selection_id}_items_panel")),
//                 IsFamiqSelectionItemsPanel,
//                 DefaultWidgetBundle(panel_bundle),
//             ))
//             .id();

//         utils::entity_push_children(root_node, &items, panel_entity);
//         panel_entity
//     }

//     pub fn _build_selector(
//         id: &str,
//         placeholder: &str,
//         root_node: &'a mut EntityCommands,
//         asset_server: &'a ResMut<'a, AssetServer>,
//         font_path: &String,
//         size: &Option<SelectionSize>,
//         border_width: UiRect,
//         border_radius: BorderRadius,
//         panel_entity: Entity,
//         label_entity: Option<Entity>,
//     ) -> Entity {
//         let selection_bundle = default_selector_bundle(border_width, border_radius, &size);

//         let placeholder_entity = root_node
//             .commands()
//             .spawn((
//                 create_selector_placeholder(placeholder, &size, asset_server, font_path),
//                 SelectorPlaceHolder,
//                 FamiqWidgetId(format!("{id}_placeholder_text")),
//                 DefaultTextBundle(create_selector_placeholder(
//                     placeholder,
//                     &size,
//                     asset_server,
//                     font_path,
//                 )),
//             ))
//             .id();

//         let arrow_icon_entity = root_node
//             .commands()
//             .spawn((
//                 create_selector_arrow_down(size, asset_server, font_path),
//                 ArrowIcon,
//                 DefaultTextBundle(create_selector_arrow_down(size, asset_server, font_path)),
//             ))
//             .id();

//         let selector_entity = root_node
//             .commands()
//             .spawn((
//                 selection_bundle.clone(),
//                 FamiqWidgetId(id.to_string()),
//                 Selection::new(placeholder.to_string()),
//                 IsFamiqSelection,
//                 SelectorPlaceHolderEntity(placeholder_entity),
//                 SelectorArrowIconEntity(arrow_icon_entity),
//                 SelectionItemsPanelEntity(panel_entity),
//                 DefaultWidgetBundle(selection_bundle),
//             ))
//             .id();

//         if let Some(label) = label_entity {
//             root_node
//                 .commands()
//                 .entity(selector_entity)
//                 .insert(SelectionLabelEntity(label));
//         }

//         utils::entity_push_children(
//             root_node,
//             &vec![placeholder_entity, arrow_icon_entity],
//             selector_entity,
//         );

//         selector_entity
//     }

//     pub fn build_selection(
//         id: &str,
//         placeholder: &str,
//         label: Option<&str>,
//         root_node: &'a mut EntityCommands,
//         asset_server: &'a ResMut<'a, AssetServer>,
//         font_path: &String,
//         size: Option<SelectionSize>,
//         items: &Vec<String>,
//         variant: SelectorVariant,
//     ) -> Entity {
//         let mut label_entity_to_use: Option<Entity> = None;

//         let mut border_width = outlined_border_width();
//         let mut border_radius = outlined_border_radius();

//         match variant {
//             SelectorVariant::Underlined => {
//                 border_width = underlined_border_width();
//                 border_radius = underlined_border_radius();
//             }
//             _ => (),
//         }
//         let panel = Self::_build_selection_panel(id, items, root_node, asset_server, font_path);

//         if let Some(label_value) = label {
//             label_entity_to_use = Some(Self::_build_selection_label(
//                 id,
//                 root_node,
//                 asset_server,
//                 font_path,
//                 &size,
//                 label_value,
//             ));
//         }

//         let selector = Self::_build_selector(
//             id,
//             placeholder,
//             root_node,
//             asset_server,
//             font_path,
//             &size,
//             border_width,
//             border_radius,
//             panel,
//             label_entity_to_use,
//         );
//         Self::_build_selection_container(id, root_node, selector, panel, label_entity_to_use)
//     }

//     pub fn set_to_unfocus(
//         selection_q: &mut Query<&mut Selection>,
//         selection_entity: Entity,
//         for_all: bool,
//     ) {
//         if for_all {
//             handle_unfocus_selection_all(selection_q);
//         } else {
//             handle_unfocus_selection_one(selection_q, selection_entity);
//         }
//     }

//     pub fn set_to_focus(selection_q: &mut Query<&mut Selection>, selection_entity: Entity) {
//         if let Ok(mut selection) = selection_q.get_mut(selection_entity) {
//             selection.focused = true;
//         }
//     }
// }
