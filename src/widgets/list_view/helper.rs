use bevy::prelude::*;

use crate::widgets::FaWidgetBundle;

fn default_list_view_container_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        height: Val::Percent(50.0),
        overflow: Overflow::clip(),
        ..default()
    }
}

pub fn default_list_view_container() -> FaWidgetBundle {
    FaWidgetBundle {
        style: default_list_view_container_style(),
        ..default()
    }
}

fn default_list_view_move_panel_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        ..default()
    }
}

pub fn default_list_view_move_panel() -> FaWidgetBundle {
    FaWidgetBundle {
        style: default_list_view_move_panel_style(),
        ..default()
    }
}
