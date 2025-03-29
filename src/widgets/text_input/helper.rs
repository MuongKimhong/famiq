use bevy::prelude::*;
use cosmic_text::{Attrs, Buffer, Editor, FontSystem};
use std::sync::Arc;
use crate::utils::*;
use super::*;

pub(crate) fn scroll_right(buf_texture_node: &mut Node, text_edit: &FaTextEdit) {
    let left_val = extract_val(buf_texture_node.left).unwrap();
    buf_texture_node.left = Val::Px(left_val - text_edit.glyph_width);
}

pub(crate) fn scroll_left(buf_texture_node: &mut Node, text_edit: &FaTextEdit) {
    let left_val = extract_val(buf_texture_node.left).unwrap();

    if left_val < text_edit.max_scroll_left() - text_edit.glyph_width {
        buf_texture_node.left = Val::Px(left_val + text_edit.glyph_width);
    } else {
        buf_texture_node.left = Val::Px(0.0);
    }
}

pub(crate) fn scroll_left_end(buf_texture_node: &mut Node) {
    buf_texture_node.left = Val::Px(0.0);
}

/// find closest cursor index at pointer location
pub(crate) fn find_glyph_index_on_mouse_down(
    buffer: &mut Buffer,
    font_system: &mut FontSystem,
    texture_node: &Node,
    text_edit: &mut FaTextEdit,
    pointer_x: f32
) -> Option<usize> {
    let line_layout = buffer.line_layout(font_system, 0);

    if line_layout.is_none() {
        return None;
    }
    let mut closest_glyph_index = None;
    let mut closest_distance = f32::MAX;
    let glyphs = &line_layout.unwrap()[0].glyphs;

    for i in 0..=text_edit.value.len() {
        let cursor_pos = text_edit.calculate_cursor_pos(glyphs, texture_node, i);
        let distance = (pointer_x - cursor_pos).abs();

        if distance < closest_distance {
            closest_distance = distance;
            closest_glyph_index = Some(i);
        }
    }

    return closest_glyph_index;
}

pub(crate) fn update_selection_state_on_arrow_keys(
    text_edit: &mut FaTextEdit,
    editor: &mut Editor
) {
    let selection_start = text_edit.selection_start_index;
    let selection_end = text_edit.selection_end_index;

    if selection_start.is_some() && selection_end.is_some() {
        text_edit.selection_end_index = Some(text_edit.cursor_index);
        let start_index = selection_start.unwrap();
        let end_index = text_edit.selection_end_index.unwrap();

        editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
        editor.set_selection(Selection::Normal(Cursor::new(0, start_index)));

        if start_index > end_index {
            text_edit.selected_text = text_edit.value[end_index..start_index].to_owned();
        }
        else if start_index < end_index {
            text_edit.selected_text = text_edit.value[start_index..end_index].to_owned();
        }
    }
}

pub(crate) fn clear_buffer_before_insert(
    editor: &mut Editor,
    text_edit: &mut FaTextEdit,
    font_system: &mut FontSystem,
    attrs: Attrs
) {
    editor.with_buffer_mut(|buffer| {
        if text_edit.value.is_empty() && !text_edit.buffer_empty {
            buffer.set_text(font_system, "", attrs, Shaping::Advanced);
            text_edit.buffer_empty = true;
        }
    });
}

/// return True, if need to redraw buffer else False
pub(crate) fn handle_cursor_blink_on_focused(
    blink_timer: &mut CursorBlinkTimer,
    cosmic_color: &mut CosmicDataColor
) -> bool {
    if !blink_timer.can_blink {
        blink_timer.is_transparent = false; // Force cursor visible, no blinking
        blink_timer.timer.reset();
        cosmic_color.cursor_color = cosmic_color.text_color;
    }
    else if blink_timer.timer.finished() {
        blink_timer.is_transparent = !blink_timer.is_transparent;
        cosmic_color.cursor_color = if blink_timer.is_transparent {
            CURSOR_INVISIBLE
        } else {
            cosmic_color.text_color
        };
        return true;
    }
    return false;
}

/// return True, if need to redraw buffer else False
pub(crate) fn handle_cursor_blink_on_unfocused(
    blink_timer: &mut CursorBlinkTimer,
    cosmic_color: &mut CosmicDataColor
) -> bool {
    if blink_timer.timer.finished() && !blink_timer.is_transparent {
        blink_timer.is_transparent = true;
        cosmic_color.cursor_color = CURSOR_INVISIBLE;
        return true;
    }
    else if blink_timer.is_transparent {
        return false; // cursor already invisible
    }
    return false;
}

pub(crate) fn update_buffer_text_layout(
    font_system: &mut FontSystem,
    text_edit: &mut FaTextEdit,
    buffer_dim: &mut Vec2,
    buffer: &mut Buffer,
    texture_node: &Node,
) {
    if let Some(layout) = buffer.line_layout(font_system, 0) {
        text_edit.text_width = layout[0].w;
        let glyphs = &layout[0].glyphs;
        text_edit.check_need_scroll(glyphs, texture_node);

        buffer_dim.x = text_edit.text_width + text_edit.glyph_width;
        buffer_dim.x *= 2.0;
    }
    buffer.set_size(font_system, Some(buffer_dim.x), Some(buffer_dim.y));
}
