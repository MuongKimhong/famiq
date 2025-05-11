use bevy::prelude::*;
use cosmic_text::{Attrs, Buffer, Editor, FontSystem, Edit, SwashCache, Color as CosmicColor};
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

fn blend_alpha(color: CosmicColor, pixels: &mut Vec<u8>, pixel_index: usize) {
    // Convert to [0,1]
    let src_a = color.a() as f32 / 255.0;
    if src_a <= 0.01 {
        return;
    }
    let src_r = color.r() as f32 / 255.0;
    let src_g = color.g() as f32 / 255.0;
    let src_b = color.b() as f32 / 255.0;

    let dst_r = pixels[pixel_index] as f32 / 255.0;
    let dst_g = pixels[pixel_index + 1] as f32 / 255.0;
    let dst_b = pixels[pixel_index + 2] as f32 / 255.0;
    let dst_a = pixels[pixel_index + 3] as f32 / 255.0;

    // pre-multiplied alpha
    let premul_src_r = src_r * src_a;
    let premul_src_g = src_g * src_a;
    let premul_src_b = src_b * src_a;

    let premul_dst_r = dst_r * dst_a;
    let premul_dst_g = dst_g * dst_a;
    let premul_dst_b = dst_b * dst_a;

    let out_a = src_a + dst_a * (1.0 - src_a);
    let out_r = (premul_src_r + premul_dst_r * (1.0 - src_a)) / out_a.max(1e-5);
    let out_g = (premul_src_g + premul_dst_g * (1.0 - src_a)) / out_a.max(1e-5);
    let out_b = (premul_src_b + premul_dst_b * (1.0 - src_a)) / out_a.max(1e-5);

    // reduce faint
    let gamma_boost = 1.1;
    let boosted_a = (out_a * gamma_boost).clamp(0.0, 1.0);

    pixels[pixel_index]     = (out_r * 255.0).clamp(0.0, 255.0) as u8;
    pixels[pixel_index + 1] = (out_g * 255.0).clamp(0.0, 255.0) as u8;
    pixels[pixel_index + 2] = (out_b * 255.0).clamp(0.0, 255.0) as u8;
    pixels[pixel_index + 3] = (boosted_a * 255.0).clamp(0.0, 255.0) as u8;
}

pub fn draw_editor_buffer(
    buffer_dim: &Vec2,
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    editor: &mut Editor,
    cosmic_data_color: &CosmicDataColor
) -> Vec<u8>{
    let y_offset = 2.5;
    let width = buffer_dim.x as usize;
    let height = buffer_dim.y as usize;
    let expected_len = width * height * 4;
    let mut pixels: Vec<u8> = vec![0; expected_len];

    let draw_closure = |x: i32, y: i32, w: u32, h: u32, color: CosmicColor| {
        for row in 0..h as i32 {
            for col in 0..w as i32 {
                // let y_row = ((y + row)).max(0);
                let y_row = ((y + row) as f32 + y_offset).max(0.0) as i32;
                let x_col = (x + col).max(0);

                if y_row >= height as i32 || x_col >= width as i32 {
                    continue;
                }

                let pixel_index = (y_row as usize * width + x_col as usize) * 4;

                if pixel_index + 3 >= pixels.len() {
                    continue;
                }
                blend_alpha(color, &mut pixels, pixel_index);
            }
        }
    };
    editor.draw(
        font_system,
        swash_cache,
        cosmic_data_color.text_color,
        cosmic_data_color.cursor_color,
        cosmic_data_color.selection_color,
        cosmic_data_color.selected_text_color,
        draw_closure,
    );
    pixels
}

pub(crate) fn create_empty_buffer_texture(buffer_dim: &Vec2, image_assets: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let empty_pixels: Vec<u8> = vec![0; (buffer_dim.x as usize) * (buffer_dim.y as usize) * 4];
    let empty_texture = Image::new_fill(
        Extent3d {
            width: buffer_dim.x as u32,
            height: buffer_dim.y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &empty_pixels,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default()
    );
    image_assets.add(empty_texture)
}

pub(crate) fn create_buffer_texture(
    buffer_dim: &Vec2,
    buffer_pixels: &Vec<u8>,
    image_assets: &mut ResMut<Assets<Image>>
) -> Handle<Image> {
    let mut texture = Image::new_fill(
        Extent3d {
            width: buffer_dim.x as u32,
            height: buffer_dim.y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        buffer_pixels,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default()
    );
    texture.sampler = ImageSampler::linear();
    image_assets.add(texture)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn handle_copy_paste_wasm(
    keys: &Res<ButtonInput<KeyCode>>,
    keycode: KeyCode,
    text_edit: &FaTextEdit,
    wasm_channel: &Option<Res<WasmPasteAsyncChannel>>,
    input_entity: Entity
) -> Result<WasmCopyPasteResult, ()>
{
    if text_edit.is_ctrl_c_pressed(keys, keycode) {
        if !text_edit.selected_text.trim().is_empty() {
            write_clipboard_wasm(&text_edit.selected_text);
            return Ok(WasmCopyPasteResult::CopyOk);
        }
    }
    else if text_edit.is_ctrl_v_pressed(keys, keycode) {
        let tx = wasm_channel.as_ref().unwrap().tx.clone();
        let _task = AsyncComputeTaskPool::get().spawn(async move {
            let promise = read_clipboard_wasm();

            let result = JsFuture::from(promise).await;

            if let Ok(js_text) = result {
                if let Some(text) = js_text.as_string() {
                    let _ = tx.try_send(WasmPaste { text, entity: input_entity });
                }
            }
        });
        return Ok(WasmCopyPasteResult::PasteOk);
    }

    Ok(WasmCopyPasteResult::NoCopyAndPaste)
}
