use bevy::prelude::*;

pub const ITEM_ON_HOVER_BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.3);
pub const ITEM_NORMAL_BG_COLOR: Color = Color::NONE; // transparent

pub fn default_selector_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row, // Horizontal layout
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            top: Val::Px(4.0),
            bottom: Val::Px(4.0),
        },
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        height: Val::Auto,
        width: Val::Percent(100.0),
        overflow: Overflow::clip_x(),
        ..default()
    }
}

pub fn default_selection_choices_panel_node() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Auto,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        padding: UiRect::all(Val::Px(0.0)),
        margin: UiRect {
            top: Val::Px(2.0),
            ..default()
        },
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        display: Display::None,
        ..default()
    }
}

pub fn default_choice_container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        padding: UiRect {
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
            right: Val::Px(0.0),
            left: Val::Px(10.0),
        },
        ..default()
    }
}
