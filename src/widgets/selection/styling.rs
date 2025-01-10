use crate::utils::extract_val;
use crate::widgets::color::*;
use super::SelectorColor;
use bevy::prelude::*;

pub const PLACEHOLDER_COLOR_UNFOCUSED: Color = Color::srgba(0.651, 0.651, 0.651, 0.6);
pub const PLACEHOLDER_COLOR_FOCUSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.922);
pub const LABEL_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.922);
pub const PANEL_BG_COLOR: Color = Color::srgba(0.29, 0.29, 0.282, 1.0);
pub const ITEM_ON_HOVER_BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
pub const ITEM_NORMAL_BG_COLOR: Color = Color::NONE; // transparent

pub fn default_selector_node(border_width: UiRect) -> Node {
    Node {
        flex_direction: FlexDirection::Row, // Horizontal layout
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        border: border_width,
        padding: UiRect {
            left: Val::Px(15.0),
            right: Val::Px(15.0),
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
        },
        margin: UiRect {
            top: Val::Px(5.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(5.0),
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

pub fn outlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(6.0))
}

pub fn underlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn rectangle_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn round_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Percent(100.0))
}

pub fn set_selection_panel_pos(selector_style: &Node, panel_style: &mut Node, label_height: f32) {
    let mut pos = 0.0;
    let top_offset = 6.0;

    if let Some(height) = extract_val(selector_style.height) {
        pos += height;
    }
    if let Some(m_top) = extract_val(selector_style.margin.top) {
        pos += m_top;
    }
    if let Some(m_bottom) = extract_val(selector_style.margin.bottom) {
        pos += m_bottom;
    }
    pos += label_height;
    panel_style.top = Val::Px(pos + top_offset);
    panel_style.left = selector_style.left;
}

pub fn set_selection_panel_width(
    parent_q: &Query<&ComputedNode>,
    parent: &Parent,
    panel_style: &mut Node,
) {
    if let Ok(parent_node) = parent_q.get(parent.get()) {
        let parent_width = parent_node.size().x;
        let padding_left = 15.0;
        let padding_right = 15.0;
        panel_style.width = Val::Px(parent_width - padding_left - padding_right);
    }
}

pub fn get_selector_background_color(color: &SelectorColor) -> BackgroundColor {
    match color {
        SelectorColor::Primary => BackgroundColor(PRIMARY_DARK_COLOR),
        SelectorColor::Secondary => BackgroundColor(SECONDARY_DARK_COLOR),
        SelectorColor::Success => BackgroundColor(SUCCESS_DARK_COLOR),
        SelectorColor::Danger => BackgroundColor(DANGER_DARK_COLOR),
        SelectorColor::Warning => BackgroundColor(WARNING_DARK_COLOR),
        SelectorColor::Info => BackgroundColor(INFO_DARK_COLOR),
        _ => BackgroundColor(PANEL_BG_COLOR)
    }
}

pub fn get_selector_border_color(color: &SelectorColor) -> BorderColor {
    match color {
        SelectorColor::Primary => BorderColor(PRIMARY_COLOR),
        SelectorColor::Secondary => BorderColor(SECONDARY_COLOR),
        SelectorColor::Success => BorderColor(SUCCESS_COLOR),
        SelectorColor::Danger => BorderColor(DANGER_COLOR),
        SelectorColor::Warning => BorderColor(WARNING_COLOR),
        SelectorColor::Info => BorderColor(INFO_COLOR),
        _ => BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922))
    }
}

pub fn get_choice_panel_background_color(color: &SelectorColor) -> BackgroundColor {
    get_selector_background_color(color)
}

pub fn get_choice_panel_border_color(color: &SelectorColor) -> BorderColor {
    match color {
        SelectorColor::Primary => BorderColor(PRIMARY_DARK_COLOR),
        SelectorColor::Secondary => BorderColor(SECONDARY_DARK_COLOR),
        SelectorColor::Success => BorderColor(SUCCESS_DARK_COLOR),
        SelectorColor::Danger => BorderColor(DANGER_DARK_COLOR),
        SelectorColor::Warning => BorderColor(WARNING_DARK_COLOR),
        SelectorColor::Info => BorderColor(INFO_DARK_COLOR),
        _ => BorderColor(PANEL_BG_COLOR)
    }
}
