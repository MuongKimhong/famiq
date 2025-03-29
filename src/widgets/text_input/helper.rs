use bevy::prelude::*;
use cosmic_text::{
    Attrs, Buffer, Editor, FontSystem, Edit, SwashCache, Color as CosmicColor,
    LayoutRun, LayoutGlyph, Placement, SwashContent
};
use unicode_segmentation::UnicodeSegmentation;
use std::{cmp, iter::once};
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
    }
    buffer.set_size(font_system, Some(buffer_dim.x), Some(buffer_dim.y));
}

// pub fn draw_editor_buffer(
//     buffer_dim: &Vec2,
//     font_system: &mut FontSystem,
//     swash_cache: &mut SwashCache,
//     editor: &mut Editor,
//     text_color: CosmicColor,
//     cursor_color: CosmicColor,
//     selection_color: CosmicColor,
//     selected_text_color: CosmicColor
// ) -> Vec<u8> {
//     let y_offset = 2.5;
//     let width = buffer_dim.x as usize;
//     let height = buffer_dim.y as usize;
//     let expected_len = width * height * 4;
//     let mut pixels: Vec<u8> = vec![0; expected_len];

//     let draw_closure = |x: i32, y: i32, w: u32, h: u32, color: CosmicColor| {
//         for row in 0..h as i32 {
//             for col in 0..w as i32 {
//                 // let y_row = ((y + row)).max(0);
//                 let y_row = ((y + row) as f32 + y_offset).max(0.0) as i32;
//                 let x_col = (x + col).max(0);

//                 if y_row >= height as i32 || x_col >= width as i32 {
//                     continue;
//                 }

//                 let idx = (y_row as usize * width + x_col as usize) * 4;

//                 if idx + 3 >= pixels.len() {
//                     continue;
//                 }
//                 // convert to [0, 1]
//                 let src_r = color.r() as f32 / 255.0;
//                 let src_g = color.g() as f32 / 255.0;
//                 let src_b = color.b() as f32 / 255.0;
//                 let src_a = color.a() as f32 / 255.0;

//                 // Destination color (rendered color) in float [0, 1]
//                 let dst_r = pixels[idx] as f32 / 255.0;
//                 let dst_g = pixels[idx + 1] as f32 / 255.0;
//                 let dst_b = pixels[idx + 2] as f32 / 255.0;
//                 let dst_a = pixels[idx + 3] as f32 / 255.0;

//                 // blend alpha
//                 let out_a = src_a + dst_a * (1.0 - src_a);
//                 let out_r = src_r * src_a + dst_r * (1.0 - src_a);
//                 let out_g = src_g * src_a + dst_g * (1.0 - src_a);
//                 let out_b = src_b * src_a + dst_b * (1.0 - src_a);

//                 // Write blended color back to pixel buffer
//                 pixels[idx]     = (out_r * 255.0).clamp(0.0, 255.0) as u8;
//                 pixels[idx + 1] = (out_g * 255.0).clamp(0.0, 255.0) as u8;
//                 pixels[idx + 2] = (out_b * 255.0).clamp(0.0, 255.0) as u8;
//                 pixels[idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;
//             }
//         }
//     };
//     editor.draw(
//         font_system,
//         swash_cache,
//         text_color,
//         cursor_color,
//         selection_color,
//         selected_text_color,
//         draw_closure,
//     );
//     pixels
// }

fn draw_cursor<F>(
    cursor_position: Option<(i32, i32)>,
    cursor_color: CosmicColor,
    line_height: f32,
    f: &mut F
)
where F: FnMut(i32, i32, u32, u32, CosmicColor)
{
    if let Some((x, y)) = cursor_position {
        f(x, y, 1, line_height as u32, cursor_color);
    }
}

fn draw_selection_highlight<F>(
    selection_bounds: Option<(Cursor, Cursor)>,
    line_i: usize,
    line_top: f32,
    line_height: f32,
    run: &LayoutRun,
    selection_color: CosmicColor,
    buffer_width: Option<f32>,
    f: &mut F
)
where F: FnMut(i32, i32, u32, u32, CosmicColor)
{
    if let Some((start, end)) = selection_bounds {
        if line_i >= start.line && line_i <= end.line {
            let mut range_opt = None;
            for glyph in run.glyphs.iter() {
                // Guess x offset based on characters
                let cluster = &run.text[glyph.start..glyph.end];
                let total = cluster.grapheme_indices(true).count();
                let mut c_x = glyph.x;
                let c_w = glyph.w / total as f32;
                for (i, c) in cluster.grapheme_indices(true) {
                    let c_start = glyph.start + i;
                    let c_end = glyph.start + i + c.len();
                    if (start.line != line_i || c_end > start.index)
                        && (end.line != line_i || c_start < end.index)
                    {
                        range_opt = match range_opt.take() {
                            Some((min, max)) => Some((
                                cmp::min(min, c_x as i32),
                                cmp::max(max, (c_x + c_w) as i32),
                            )),
                            None => Some((c_x as i32, (c_x + c_w) as i32)),
                        };
                    } else if let Some((min, max)) = range_opt.take() {
                        f(
                            min,
                            line_top as i32,
                            cmp::max(0, max - min) as u32,
                            line_height as u32,
                            selection_color,
                        );
                    }
                    c_x += c_w;
                }
            }

            if run.glyphs.is_empty() && end.line > line_i {
                // Highlight all of internal empty lines
                range_opt = Some((0, buffer_width.unwrap_or(0.0) as i32));
            }

            if let Some((mut min, mut max)) = range_opt.take() {
                if end.line > line_i {
                    // Draw to end of line
                    if run.rtl {
                        min = 0;
                    } else {
                        max = buffer_width.unwrap_or(0.0) as i32;
                    }
                }
                f(
                    min,
                    line_top as i32,
                    cmp::max(0, max - min) as u32,
                    line_height as u32,
                    selection_color,
                );
            }
        }
    }
}

fn linear_to_srgb(x: f32) -> f32 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}


fn draw_glyphs<F>(
    run: &LayoutRun,
    selection_bounds: Option<(Cursor, Cursor)>,
    text_color: CosmicColor,
    selected_text_color: CosmicColor,
    line_i: usize,
    line_y: f32,
    swash_cache: &mut SwashCache,
    font_system: &mut FontSystem,
    f: &mut F
)
where
    F: FnMut(i32, i32, u32, u32, CosmicColor),
{
    for glyph in run.glyphs.iter() {
        let physical_glyph = glyph.physical((0., 0.), 1.0);

        let mut glyph_color = glyph.color_opt.unwrap_or(text_color);
        if text_color != selected_text_color {
            if let Some((start, end)) = selection_bounds {
                if line_i >= start.line
                    && line_i <= end.line
                    && (start.line != line_i || glyph.end > start.index)
                    && (end.line != line_i || glyph.start < end.index)
                {
                    glyph_color = selected_text_color;
                }
            }
        }

        let image = swash_cache
            .get_image_uncached(font_system, physical_glyph.cache_key)
            .unwrap();

        let Placement {
            left,
            top,
            width,
            height,
        } = image.placement;

        match image.content {
            SwashContent::Mask => {
                let mut i = 0;
                for y in 0..height as i32 {
                    for x in 0..width as i32 {
                        let alpha = image.data[i];
                        i += 1;

                        let color = CosmicColor::rgba(
                            glyph_color.r(),
                            glyph_color.g(),
                            glyph_color.b(),
                            alpha,
                        );

                        // f(left + x, -top + y, 1, 1, color);
                        f(
                            physical_glyph.x + left + x,
                            line_y as i32 + physical_glyph.y + (-top + y),
                            1,
                            1,
                            color,
                        );
                    }
                }
            }
            SwashContent::Color => {
                let mut i = 0;
                for y in 0..height as i32 {
                    for x in 0..width as i32 {
                        let color = CosmicColor::rgba(
                            image.data[i],
                            image.data[i + 1],
                            image.data[i + 2],
                            image.data[i + 3],
                        );
                        i += 4;

                        // f(left + x, -top + y, 1, 1, color);
                        f(
                            physical_glyph.x + left + x,
                            line_y as i32 + physical_glyph.y + (-top + y),
                            1,
                            1,
                            color,
                        );
                    }
                }
            }
            SwashContent::SubpixelMask => todo!(),
        }
    }
}


pub fn draw_editor_buffer(
    buffer_dim: &Vec2,
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    editor: &mut Editor,
    text_color: CosmicColor,
    cursor_color: CosmicColor,
    selection_color: CosmicColor,
    selected_text_color: CosmicColor
) -> Vec<u8> {
    let y_offset = 2.5;
    let width = buffer_dim.x as usize;
    let height = buffer_dim.y as usize;
    let expected_len = width * height * 4;
    let mut pixels: Vec<u8> = vec![0; expected_len];

    let mut draw_closure = |x: i32, y: i32, w: u32, h: u32, color: CosmicColor| {
        for row in 0..h as i32 {
            for col in 0..w as i32 {
                // let y_row = ((y + row)).max(0);
                let y_row = ((y + row) as f32 + y_offset).max(0.0) as i32;
                let x_col = (x + col).max(0);

                if y_row >= height as i32 || x_col >= width as i32 {
                    continue;
                }

                let idx = (y_row as usize * width + x_col as usize) * 4;

                if idx + 3 >= pixels.len() {
                    continue;
                }
                // convert to [0, 1]
                let src_r = color.r() as f32 / 255.0;
                let src_g = color.g() as f32 / 255.0;
                let src_b = color.b() as f32 / 255.0;
                let src_a = color.a() as f32 / 255.0;

                // Destination color (rendered color) in float [0, 1]
                let dst_r = pixels[idx] as f32 / 255.0;
                let dst_g = pixels[idx + 1] as f32 / 255.0;
                let dst_b = pixels[idx + 2] as f32 / 255.0;
                let dst_a = pixels[idx + 3] as f32 / 255.0;

                // blend alpha
                let out_a = src_a + dst_a * (1.0 - src_a);
                let out_r = src_r * src_a + dst_r * (1.0 - src_a);
                let out_g = src_g * src_a + dst_g * (1.0 - src_a);
                let out_b = src_b * src_a + dst_b * (1.0 - src_a);

                // Write blended color back to pixel buffer
                // pixels[idx]     = (out_r * 255.0).clamp(0.0, 255.0) as u8;
                // pixels[idx + 1] = (out_g * 255.0).clamp(0.0, 255.0) as u8;
                // pixels[idx + 2] = (out_b * 255.0).clamp(0.0, 255.0) as u8;
                // pixels[idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;
                pixels[idx]     = (linear_to_srgb(out_r) * 255.0).clamp(0.0, 255.0) as u8;
                pixels[idx + 1] = (linear_to_srgb(out_g) * 255.0).clamp(0.0, 255.0) as u8;
                pixels[idx + 2] = (linear_to_srgb(out_b) * 255.0).clamp(0.0, 255.0) as u8;
                pixels[idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;

            }
        }
    };
    let selection_bounds = editor.selection_bounds();
    let cursor_position = editor.cursor_position();
    editor.with_buffer(|buffer| {
        for run in buffer.layout_runs() {
            let line_i = run.line_i;
            let line_y = run.line_y;
            let line_top = run.line_top;
            let line_height = run.line_height;

            draw_selection_highlight(
                selection_bounds,
                line_i,
                line_top,
                line_height,
                &run,
                selection_color,
                buffer.size().0,
                &mut draw_closure
            );

            draw_cursor(cursor_position, cursor_color, line_height, &mut draw_closure);

            draw_glyphs(
                &run,
                selection_bounds,
                text_color,
                selected_text_color,
                line_i,
                line_y,
                swash_cache,
                font_system,
                &mut draw_closure
            );
        }
    });
    pixels
}
