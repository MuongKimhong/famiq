use bevy::prelude::*;

pub fn default_move_panel_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        padding: UiRect {
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(2.0),
            bottom: Val::Px(2.0)
        },
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
        overflow: Overflow::scroll_y(),
        padding: UiRect::all(Val::Px(0.0)),
        ..default()
    }
}
