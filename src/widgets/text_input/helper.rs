use bevy::prelude::*;
use crate::utils;
use super::{TextInputSize, TextInputColor, IsFamiqTextInputCursor, CharacterSize, CURSOR_WIDTH};
use bevy::text::TextLayoutInfo;
use crate::widgets::color::*;

pub const PLACEHOLDER_COLOR: Color = Color::srgba(0.749, 0.749, 0.749, 1.0);
pub const TEXT_INPUT_VALUE_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.922);

pub fn default_input_node() -> Node {
    Node {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
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
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

pub fn outlined_border_width() -> UiRect {
    UiRect::all(Val::Px(2.0))
}

pub fn underlined_border_width() -> UiRect {
    UiRect {
        left: Val::Px(0.0),
        right: Val::Px(0.0),
        top: Val::Px(0.0),
        bottom: Val::Px(2.0),
    }
}

pub fn outlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(6.0))
}

pub fn underlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn round_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Percent(50.0))
}

pub fn rectangle_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn get_text_size(size: &TextInputSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        TextInputSize::Small => size_small,
        TextInputSize::Normal => size_normal,
        TextInputSize::Large => size_large,
    };
    text_size
}

pub fn get_input_background_color(color: &TextInputColor) -> BackgroundColor {
    match color {
        TextInputColor::Primary => BackgroundColor(PRIMARY_DARK_COLOR),
        TextInputColor::Secondary => BackgroundColor(SECONDARY_DARK_COLOR),
        TextInputColor::Success => BackgroundColor(SUCCESS_DARK_COLOR),
        TextInputColor::Danger => BackgroundColor(DANGER_DARK_COLOR),
        TextInputColor::Warning => BackgroundColor(WARNING_DARK_COLOR),
        TextInputColor::Info => BackgroundColor(INFO_DARK_COLOR),
        _ => BackgroundColor(WHITE_COLOR)
    }
}

pub fn get_input_border_color(color: &TextInputColor) -> BorderColor {
    match color {
        TextInputColor::Primary => BorderColor(PRIMARY_COLOR),
        TextInputColor::Secondary => BorderColor(SECONDARY_COLOR),
        TextInputColor::Success => BorderColor(SUCCESS_COLOR),
        TextInputColor::Danger => BorderColor(DANGER_COLOR),
        TextInputColor::Warning => BorderColor(WARNING_COLOR),
        TextInputColor::Info => BorderColor(INFO_COLOR),
        _ => BorderColor(WHITE_COLOR)
    }
}

/// Updates the cursor position based on character width and action.
/// `add` indicates whether a character is added (true) or removed (false).
pub fn update_cursor_position(
    cursor_q: &mut Query<(&mut Node, &mut Visibility, &IsFamiqTextInputCursor)>,
    cursor_entity: Entity,
    char_width: f32,
    add: bool
) {
    if let Ok((mut node, _, _)) = cursor_q.get_mut(cursor_entity) {
        let left = utils::extract_val(node.left).unwrap();

        if add {
            node.left = Val::Px(left + char_width);
        }
        else {
            node.left = Val::Px(left - char_width);
        }
    }
}


pub fn handle_on_focused(
    text_color: &mut TextColor,
    bg_color: &BackgroundColor,
    visibility: &mut Visibility,
    cursor_node: &mut Node,
    text_input_node: &Node,
    text_info: &TextLayoutInfo,
    text_content: &str,
    char_size: &mut CharacterSize,
) {
    // Update text color based on background
    text_color.0 = if bg_color.0 == WHITE_COLOR {
        BLACK_COLOR
    } else {
        TEXT_INPUT_VALUE_COLOR
    };

    *visibility = Visibility::Visible;

    // Update character size
    char_size.width = text_info.size.x / text_content.len() as f32;
    char_size.height = text_info.size.y;

    // Set cursor node position and size if it's not set yet
    if utils::extract_val(cursor_node.left).unwrap_or(0.0) == 0.0 {
        cursor_node.left = text_input_node.padding.left.clone();
        cursor_node.top = text_input_node.padding.top.clone();
        cursor_node.width = Val::Px(CURSOR_WIDTH);
        cursor_node.height = Val::Px(text_info.size.y);
    }
}
