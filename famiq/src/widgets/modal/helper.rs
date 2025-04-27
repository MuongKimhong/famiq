use bevy::prelude::*;

pub fn default_modal_background_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        position_type: PositionType::Absolute,
        display: Display::None,
        left: Val::Px(0.0),
        right: Val::Px(0.0),
        ..default()
    }
}

pub fn default_modal_container_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
