pub mod components;
pub mod helper;
pub mod styling;
pub mod systems;

use crate::utils;
use crate::widgets::{DefaultTextBundle, DefaultWidgetBundle, FamiqWidgetId};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use components::*;
pub use helper::*;
pub use styling::*;
pub use systems::*;

pub type SelectionWidgetId = String;
pub type SelectedItem = String;

#[derive(Resource)]
pub struct SelectedItemsResource {
    pub items: HashMap<SelectionWidgetId, SelectedItem>,
}

impl SelectedItemsResource {
    pub fn update_or_insert(&mut self, id: SelectionWidgetId, selected_item: SelectedItem) {
        if let Some(item_value) = self.items.get_mut(&id) {
            *item_value = selected_item;
        } else {
            self.items.insert(id, selected_item);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SelectorVariant {
    Outlined,
    Default,
    Underlined,
}

#[derive(PartialEq)]
pub enum SelectionSize {
    Small,
    Normal,
    Large,
}

pub struct FaSelection;

// dont' need container
impl<'a> FaSelection {
    pub fn _build_selection_container(
        selection_id: &str,
        root_node: &'a mut EntityCommands,
        selector: Entity,
        panel: Entity,
        label: Option<Entity>,
    ) -> Entity {
        let mut children = vec![selector, panel];

        if let Some(label_entity) = label {
            children.insert(0, label_entity);
        }

        let container_bundle = default_selection_container_bundle();
        let container_entity = root_node
            .commands()
            .spawn((
                container_bundle.clone(),
                FamiqWidgetId(format!("{selection_id}_selection_container")),
                IsFamiqSelectionContainer,
                DefaultWidgetBundle(container_bundle),
            ))
            .id();

        root_node.add_child(container_entity);
        utils::entity_push_children(root_node, &children, container_entity);
        container_entity
    }

    pub fn _build_selection_label(
        selection_id: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: &Option<SelectionSize>,
        label: &str,
    ) -> Entity {
        let label_bundle = default_selection_label_bundle();
        let text_bunle = create_selection_label_text(label, size, asset_server, font_path);

        let text_entity = root_node
            .commands()
            .spawn((
                text_bunle,
                DefaultTextBundle(create_selection_label_text(
                    label,
                    size,
                    asset_server,
                    font_path,
                )),
            ))
            .id();

        let label_entity = root_node
            .commands()
            .spawn((
                label_bundle.clone(),
                FamiqWidgetId(format!("{selection_id}_selection_label")),
                IsFamiqSelectionLabel,
                DefaultWidgetBundle(label_bundle),
            ))
            .id();

        utils::entity_add_child(root_node, text_entity, label_entity);
        label_entity
    }

    pub fn _build_selection_panel(
        selection_id: &str,
        items: &Vec<String>,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let items = create_items_text(selection_id, items, root_node, asset_server, font_path);

        let panel_bundle = default_selection_items_panel_bundle();
        let panel_entity = root_node
            .commands()
            .spawn((
                panel_bundle.clone(),
                FamiqWidgetId(format!("{selection_id}_items_panel")),
                IsFamiqSelectionItemsPanel,
                DefaultWidgetBundle(panel_bundle),
            ))
            .id();

        utils::entity_push_children(root_node, &items, panel_entity);
        panel_entity
    }

    pub fn _build_selector(
        id: &str,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: &Option<SelectionSize>,
        border_width: UiRect,
        border_radius: BorderRadius,
        panel_entity: Entity,
        label_entity: Option<Entity>,
    ) -> Entity {
        let selection_bundle = default_selector_bundle(border_width, border_radius, &size);

        let placeholder_entity = root_node
            .commands()
            .spawn((
                create_selector_placeholder(placeholder, &size, asset_server, font_path),
                SelectorPlaceHolder,
                FamiqWidgetId(format!("{id}_placeholder_text")),
                DefaultTextBundle(create_selector_placeholder(
                    placeholder,
                    &size,
                    asset_server,
                    font_path,
                )),
            ))
            .id();

        let arrow_icon_entity = root_node
            .commands()
            .spawn((
                create_selector_arrow_down(size, asset_server, font_path),
                ArrowIcon,
                DefaultTextBundle(create_selector_arrow_down(size, asset_server, font_path)),
            ))
            .id();

        let selector_entity = root_node
            .commands()
            .spawn((
                selection_bundle.clone(),
                FamiqWidgetId(id.to_string()),
                Selection::new(placeholder.to_string()),
                IsFamiqSelection,
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity),
                SelectionItemsPanelEntity(panel_entity),
                DefaultWidgetBundle(selection_bundle),
            ))
            .id();

        if let Some(label) = label_entity {
            root_node
                .commands()
                .entity(selector_entity)
                .insert(SelectionLabelEntity(label));
        }

        utils::entity_push_children(
            root_node,
            &vec![placeholder_entity, arrow_icon_entity],
            selector_entity,
        );

        selector_entity
    }

    pub fn build_selection(
        id: &str,
        placeholder: &str,
        label: Option<&str>,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: Option<SelectionSize>,
        items: &Vec<String>,
        variant: SelectorVariant,
    ) -> Entity {
        let mut label_entity_to_use: Option<Entity> = None;

        let mut border_width = outlined_border_width();
        let mut border_radius = outlined_border_radius();

        match variant {
            SelectorVariant::Underlined => {
                border_width = underlined_border_width();
                border_radius = underlined_border_radius();
            }
            _ => (),
        }
        let panel = Self::_build_selection_panel(id, items, root_node, asset_server, font_path);

        if let Some(label_value) = label {
            label_entity_to_use = Some(Self::_build_selection_label(
                id,
                root_node,
                asset_server,
                font_path,
                &size,
                label_value,
            ));
        }

        let selector = Self::_build_selector(
            id,
            placeholder,
            root_node,
            asset_server,
            font_path,
            &size,
            border_width,
            border_radius,
            panel,
            label_entity_to_use,
        );
        Self::_build_selection_container(id, root_node, selector, panel, label_entity_to_use)
    }

    pub fn set_to_unfocus(
        selection_q: &mut Query<&mut Selection>,
        selection_entity: Entity,
        for_all: bool,
    ) {
        if for_all {
            handle_unfocus_selection_all(selection_q);
        } else {
            handle_unfocus_selection_one(selection_q, selection_entity);
        }
    }

    pub fn set_to_focus(selection_q: &mut Query<&mut Selection>, selection_entity: Entity) {
        if let Ok(mut selection) = selection_q.get_mut(selection_entity) {
            selection.focused = true;
        }
    }
}
