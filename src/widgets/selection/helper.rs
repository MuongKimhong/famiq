use crate::utils::strip_assets_prefix;
use crate::widgets::selection::*;
use crate::widgets::{DefaultTextBundle, DefaultWidgetBundle, FaWidgetBundle};
use bevy::prelude::*;

fn set_selection_default_size(
    size: &Option<SelectionSize>,
    text_input_bundle: &mut FaWidgetBundle,
) {
    let width = Val::Percent(100.0);
    let size_small_height = Val::Px(28.0);
    let size_normal_height = Val::Px(34.0);
    let size_large_height = Val::Px(40.0);

    let height = match size {
        Some(SelectionSize::Small) => size_small_height,
        Some(SelectionSize::Normal) => size_normal_height,
        Some(SelectionSize::Large) => size_large_height,
        None => size_normal_height,
    };
    text_input_bundle.style.height = height;
    text_input_bundle.style.width = width;
}

pub fn default_selector_bundle(
    border_width: UiRect,
    border_radius: BorderRadius,
    size: &Option<SelectionSize>,
) -> FaWidgetBundle {
    let mut selection_bundle = FaWidgetBundle {
        style: get_default_selector_style(border_width),
        border_radius,
        border_color: BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922)),
        ..default()
    };
    set_selection_default_size(size, &mut selection_bundle);
    selection_bundle
}

pub fn default_selection_container_bundle() -> FaWidgetBundle {
    FaWidgetBundle {
        style: default_selection_container_style(),
        ..default()
    }
}

pub fn default_selection_items_panel_bundle() -> FaWidgetBundle {
    FaWidgetBundle {
        style: default_selection_items_panel_style(),
        border_radius: BorderRadius::all(Val::Px(5.0)),
        z_index: ZIndex::Global(10),
        visibility: Visibility::Hidden,
        background_color: BackgroundColor(PANEL_BG_COLOR),
        ..default()
    }
}

pub fn default_selection_label_bundle() -> FaWidgetBundle {
    FaWidgetBundle {
        style: default_selection_label_style(),
        ..default()
    }
}

pub fn create_selection_label_text(
    label: &str,
    size: &Option<SelectionSize>,
    asset_server: &ResMut<AssetServer>,
    font_path: &String,
) -> TextBundle {
    let path = strip_assets_prefix(font_path).unwrap();
    TextBundle::from_section(
        label,
        TextStyle {
            font: asset_server.load(path),
            font_size: get_text_size(size),
            color: LABEL_COLOR,
        },
    )
}

pub fn get_text_size(size: &Option<SelectionSize>) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        Some(SelectionSize::Small) => size_small,
        Some(SelectionSize::Normal) => size_normal,
        Some(SelectionSize::Large) => size_large,
        None => size_normal,
    };
    text_size
}

pub fn create_selector_placeholder(
    placeholder: &str,
    size: &Option<SelectionSize>,
    asset_server: &ResMut<AssetServer>,
    font_path: &String,
) -> TextBundle {
    let path = strip_assets_prefix(font_path).unwrap();

    TextBundle::from_section(
        placeholder,
        TextStyle {
            font: asset_server.load(path),
            font_size: get_text_size(size),
            color: PLACEHOLDER_COLOR_UNFOCUSED,
        },
    )
}

pub fn create_selector_arrow_down(
    input_size: &Option<SelectionSize>,
    asset_server: &ResMut<AssetServer>,
    font_path: &String,
) -> TextBundle {
    TextBundle::from_section(
        "v",
        TextStyle {
            font: asset_server.load(strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(input_size),
            color: PLACEHOLDER_COLOR_UNFOCUSED,
        },
    )
}

pub fn handle_unfocus_selection_all(selection_q: &mut Query<&mut Selection>) {
    for mut selection in selection_q.iter_mut() {
        selection.focused = false;
    }
}

pub fn handle_unfocus_selection_one(
    selection_q: &mut Query<&mut Selection>,
    selection_entity: Entity,
) {
    if let Ok(mut selection) = selection_q.get_mut(selection_entity) {
        selection.focused = false;
    }
}

pub fn create_items_text<'a>(
    selection_id: &str,
    items: &Vec<String>,
    root_node: &'a mut EntityCommands,
    asset_server: &'a ResMut<'a, AssetServer>,
    font_path: &String,
) -> Vec<Entity> {
    let mut selection_items: Vec<Entity> = Vec::new();

    for item in items.iter() {
        let item_bundle = FaWidgetBundle {
            style: default_item_style(),
            background_color: ITEM_NORMAL_BG_COLOR.into(), // transparent
            ..default()
        };

        let item_text_entity = root_node
            .commands()
            .spawn((
                TextBundle::from_section(item, default_item_text_style(asset_server, font_path)),
                DefaultTextBundle(TextBundle::from_section(
                    item,
                    default_item_text_style(asset_server, font_path),
                )),
                FamiqWidgetId(format!("{selection_id}_selection_item_text")),
            ))
            .id();

        let item_entity = root_node
            .commands()
            .spawn((
                item_bundle.clone(),
                FamiqWidgetId(format!("{selection_id}_selection_item")),
                IsFamiqSelectionItem,
                SelectionItemTextEntity(item_text_entity),
                DefaultWidgetBundle(item_bundle),
            ))
            .id();

        root_node
            .commands()
            .entity(item_entity)
            .add_child(item_text_entity);

        selection_items.push(item_entity);
    }
    selection_items
}
