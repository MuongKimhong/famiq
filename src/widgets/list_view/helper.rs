use bevy::prelude::*;

pub fn default_move_panel_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        ..default()
    }
}

pub fn default_listview_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        height: Val::Percent(50.0),
        overflow: Overflow::clip(),
        ..default()
    }
}
