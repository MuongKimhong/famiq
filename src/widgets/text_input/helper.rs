use bevy::prelude::*;
use smol_str::SmolStr;
use crate::{utils, widgets::*};
use super::*;
use bevy::text::TextLayoutInfo;
use crate::widgets::color::*;

pub fn default_input_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
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

pub fn get_text_size(size: &WidgetSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        WidgetSize::Small => size_small,
        WidgetSize::Large => size_large,
        _ => size_normal

    };
    text_size
}

pub fn get_input_color(color: &WidgetColor) -> Color {
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

/// Internal helper function to updates the cursor position based on character width and action.
/// `add` indicates whether a character is added (true) or removed (false).
pub fn _update_cursor_position(
    cursor_q: &mut Query<
        (&mut Node, &mut Visibility, &IsFamiqTextInputCursor),
        Without<CharacterSize>
    >,
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

/// Internal helper function to calculate cursor size,
/// updating visibility and set initial position.
pub fn _handle_cursor_on_focused(
    cursor_node: &mut Node,
    text_input_node: &Node,
    text_info: &TextLayoutInfo,
    text_content: &str,
    char_size: &mut CharacterSize,
    text_input: &TextInput
) {
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
    else {

        let mut position = text_input.cursor_index as f32 * char_size.width;

        match utils::extract_val(text_input_node.padding.left) {
            Some(v) => position += v,
            _ => {}
        }

        cursor_node.left = Val::Px(position);
    }
}

/// Internal helper function to update text_input value & text_input resource.
pub fn _update_text_input_value(
    input_entity: Entity,
    input_id: Option<&FamiqWidgetId>,
    input_resource: &mut ResMut<FaTextInputResource>,
    text_input: &mut TextInput,
    appending: bool,
    new_char: Option<&SmolStr>
) {
    if appending {
        text_input.text.insert_str(text_input.cursor_index, new_char.unwrap());
        text_input.cursor_index += 1;

    } else {
        if text_input.cursor_index > 0 {
            let byte_index = text_input.text.char_indices().nth(text_input.cursor_index - 1).map(|(i, _)| i).unwrap();
            text_input.text.remove(byte_index);
            text_input.cursor_index -= 1;
        }
    }

    // call _insert_by_id & _insert_by_entity because insert will update the value if it exists.
    if let Some(id) = input_id {
        input_resource._insert_by_id(id.0.clone(), text_input.text.clone());
    }
    input_resource._insert_by_entity(input_entity, text_input.text.clone());
}

/// Internal helper function to mask placeholder as password.
pub fn _handle_mask_placeholder(
    toggle_icon_entity: Option<&FamiqTextInputToggleIconEntity>,
    toggle_icon_q: &Query<&TogglePasswordIcon>,
    text_input: &TextInput,
    placeholder_text: &mut Text
) {
    if let Some(toggle_icon_entity) = toggle_icon_entity {
        if let Ok(toggle_icon) = toggle_icon_q.get(toggle_icon_entity.0) {
            if !toggle_icon.can_see_text {
                placeholder_text.0 = mask_string(text_input.text.as_str());
            }
        }
    }
}
