use crate::widgets::color::*;
use super::*;
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
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
        },
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        height: Val::Auto,
        width: Val::Percent(100.0),
        ..default()
    }
}

pub fn default_selection_container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart, // Align children at the top
        height: Val::Auto,
        ..default()
    }
}

pub fn default_selection_choices_panel_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        height: Val::Auto,
        padding: UiRect {
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
        },
        margin: UiRect::all(Val::Px(2.0)),
        position_type: PositionType::Absolute,
        ..default()
    }
}

pub fn default_choice_container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        padding: UiRect {
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
            right: Val::Px(0.0),
            left: Val::Px(10.0),
        },
        ..default()
    }
}

// color for both selector and choice panel
pub fn get_selection_color(color: &WidgetColor) -> Color {
    match color {
        WidgetColor::Primary => PRIMARY_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_DARK_COLOR,
        WidgetColor::Secondary => SECONDARY_COLOR,
        WidgetColor::Success => SUCCESS_COLOR,
        WidgetColor::SuccessDark => SUCCESS_DARK_COLOR,
        WidgetColor::Danger => DANGER_COLOR,
        WidgetColor::DangerDark => DANGER_DARK_COLOR,
        WidgetColor::Warning => WARNING_COLOR,
        WidgetColor::WarningDark => WARNING_DARK_COLOR,
        WidgetColor::Info => INFO_COLOR,
        WidgetColor::InfoDark => INFO_DARK_COLOR,
        _ => WHITE_COLOR
    }
}

pub fn get_text_color(color: &WidgetColor) -> Color {
    match color {
        WidgetColor::Secondary => WHITE_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_COLOR,
        WidgetColor::SuccessDark => SUCCESS_COLOR,
        WidgetColor::DangerDark => DANGER_COLOR,
        WidgetColor::WarningDark => WARNING_COLOR,
        WidgetColor::InfoDark => INFO_COLOR,
        _ => BLACK_COLOR,
    }
}
