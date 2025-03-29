/// Suport single line only. Bugs are waiting for you somewhere.

pub mod styling;
pub mod components;
pub mod tests;
pub mod text_edit;
pub mod helper;
pub mod system_params;

use styling::*;
use system_params::*;
pub use components::*;
pub use text_edit::*;
use crate::event_writer::{FaMouseEvent, FaValueChangeEvent};
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::*;

use bevy::render::render_resource::{TextureDimension, Extent3d, TextureFormat};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::image::ImageSampler;
use bevy::asset::RenderAssetUsages;
use bevy::ecs::system::EntityCommands;
use bevy::input::ButtonState;
use bevy::prelude::*;
use cosmic_text::{
    Attrs, Metrics, Buffer, Editor, Family, Edit, Shaping, Weight, Cursor, Selection, Action
};
use smol_str::SmolStr;
use arboard::Clipboard;
use std::sync::Arc;

// scaling factor for buffer, used for Window platform.
pub const BUFFER_SCALE_FACTOR: f32 = 1.0;

#[derive(Default)]
pub struct IsFamiqTextInputResource;
pub type FaTextInputResource = InputResource<IsFamiqTextInputResource>;

#[derive(Event, Debug)]
pub struct RequestRedrawBuffer {
    pub input_entity: Entity
}

impl RequestRedrawBuffer {
    pub fn new(input_entity: Entity) -> Self {
        Self {
            input_entity
        }
    }
}

/// Represents the Famiq text input widget, which includes placeholder text, a blinking cursor, and customizable styles.
/// Support UTF-8 encoded only.
pub struct FaTextInput;

// Needs container
impl<'a> FaTextInput {
    fn _build_input(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
    ) -> Entity {
        let input_color = get_color(&attributes.color);
        let placeholder_color = get_text_color(&attributes.color);
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.border_color = BorderColor(input_color);
        style_components.background_color = BackgroundColor(input_color);
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));

        let text_data = CosmicTextData {
            handle: attributes.font_handle.clone().unwrap(),
            size: get_text_size(&attributes.size),
        };

        let entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqTextInput,
                IsFamiqMainWidget,
                DefaultWidgetEntity::from(style_components),
                FaTextEdit::new(placeholder),
                CosmicDataColor::new(placeholder_color),
                CosmicData::default(),
                TextColor(get_text_color(&attributes.color)),
                CursorBlinkTimer::default(),
                text_data
            ))
            .observe(FaTextInput::handle_on_mouse_over)
            .observe(FaTextInput::handle_on_mouse_out)
            .observe(FaTextInput::handle_on_mouse_down)
            .observe(FaTextInput::handle_on_mouse_up)
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        _input_type: TextInputType
    ) -> Entity {
        let input_entity = Self::_build_input(attributes, root_node, placeholder);

        if attributes.has_tooltip {
            build_tooltip_node(attributes, root_node, input_entity);
        }
        input_entity
    }

    pub(crate) fn handle_buffer_texture_on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut param: BufTexturePickingParam
    ) {
        let (texture_node, input_entity) = param.texture_q.get(trigger.entity()).unwrap();
        let (transform, computed, mut text_edit, mut cosmic_data) = param.input_q.get_mut(input_entity.0).unwrap();

        if text_edit.value.is_empty() {
            trigger.propagate(true);
            return;
        }
        let local_pointer_pos = mouse_pos_to_local_node_pos(
            &trigger.pointer_location.position,
            computed,
            transform
        );
        if let Some(editor) = cosmic_data.editor.as_mut() {
            let mut closest_glyph_index = None;

            editor.with_buffer_mut(|buffer| {
                closest_glyph_index = helper::find_glyph_index_on_mouse_down(
                    buffer,
                    &mut param.font_system.0,
                    texture_node,
                    &mut text_edit,
                    local_pointer_pos.x
                );
            });
            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
                text_edit.clear_selection();
                editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                editor.action(&mut param.font_system.0, Action::Escape);
                param.request_redraw.send(RequestRedrawBuffer::new(input_entity.0));
                param.famiq_res.update_all_focus_states(false);
                param.famiq_res.update_or_insert_focus_state(input_entity.0, true);
                trigger.propagate(false);
            }
        }
    }

    pub(crate) fn handle_buffer_texture_on_start_selection(
        mut trigger: Trigger<Pointer<DragStart>>,
        mut param: BufTexturePickingParam
    ) {
        let (texture_node, input_entity) = param.texture_q.get(trigger.entity()).unwrap();
        let (transform, computed, mut text_edit, mut cosmic_data) = param.input_q.get_mut(input_entity.0).unwrap();

        if text_edit.value.is_empty() {
            trigger.propagate(true);
            return;
        }
        let local_pointer_pos = mouse_pos_to_local_node_pos(
            &trigger.pointer_location.position,
            computed,
            transform
        );
        if let Some(editor) = cosmic_data.editor.as_mut() {
            let mut closest_glyph_index = None;

            editor.with_buffer_mut(|buffer| {
                closest_glyph_index = helper::find_glyph_index_on_mouse_down(
                    buffer,
                    &mut param.font_system.0,
                    texture_node,
                    &mut text_edit,
                    local_pointer_pos.x
                );
            });
            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
                text_edit.selection_start_index = Some(glyph_index);
                editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                param.request_redraw.send(RequestRedrawBuffer::new(input_entity.0));
                param.famiq_res.update_all_focus_states(false);
                param.famiq_res.update_or_insert_focus_state(input_entity.0, true);
                trigger.propagate(false);
            }
        }
    }

    pub(crate) fn handle_buffer_texture_on_selecting(
        mut trigger: Trigger<Pointer<Drag>>,
        mut param: BufTexturePickingParam
    ) {
        let (texture_node, input_entity) = param.texture_q.get(trigger.entity()).unwrap();
        let (transform, computed, mut text_edit, mut cosmic_data) = param.input_q.get_mut(input_entity.0).unwrap();

        if text_edit.value.is_empty() {
            trigger.propagate(true);
            return;
        }
        let local_pointer_pos = mouse_pos_to_local_node_pos(
            &trigger.pointer_location.position,
            computed,
            transform
        );
        if let Some(editor) = cosmic_data.editor.as_mut() {
            let mut closest_glyph_index = None;

            editor.with_buffer_mut(|buffer| {
                closest_glyph_index = helper::find_glyph_index_on_mouse_down(
                    buffer,
                    &mut param.font_system.0,
                    texture_node,
                    &mut text_edit,
                    local_pointer_pos.x
                );
            });
            if let Some(glyph_index) = closest_glyph_index {
                if let Some(start_index) = text_edit.selection_start_index {
                    text_edit.cursor_index = glyph_index;
                    text_edit.selection_end_index = Some(glyph_index);
                    editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                    editor.set_selection(Selection::Normal(Cursor::new(0, start_index)));
                    param.request_redraw.send(RequestRedrawBuffer::new(input_entity.0));

                    if start_index > glyph_index {
                        text_edit.selected_text = text_edit.value[glyph_index..start_index].to_owned();
                    }
                    else if start_index < glyph_index {
                        text_edit.selected_text = text_edit.value[start_index..glyph_index].to_owned();
                    }
                }
                param.famiq_res.update_all_focus_states(false);
                param.famiq_res.update_or_insert_focus_state(input_entity.0, true);
                trigger.propagate(false);
            }
        }
    }

    pub(crate) fn handle_text_input_on_focused(
        mut input_q: Query<(Entity, &mut CursorBlinkTimer, &mut CosmicDataColor)>,
        famiq_res: Res<FamiqResource>
    ) {
        if !famiq_res.is_changed() || famiq_res.is_added() {
            return;
        }
        input_q.iter_mut().for_each(|(entity, mut cursor_blink, mut cosmic_color)| {
            if let Some(focused) = famiq_res.get_widget_focus_state(&entity) {
                if focused {
                    cosmic_color.cursor_color = cosmic_color.text_color;
                }
                let duration =  cursor_blink.timer.duration();
                cursor_blink.timer.set_elapsed(duration);
            }
        });
    }

    fn handle_on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut input_q: Query<
            (&mut BoxShadow, &BorderColor, Option<&FamiqWidgetId>, &GlobalTransform, Option<&FamiqTooltipEntity>),
            With<IsFamiqTextInput>
        >,
        mut param: InputPickingParam
    ) {
        if let Ok((mut box_shadow, border_color, id, transform, tooltip_entity)) = input_q.get_mut(trigger.entity()) {
            box_shadow.color = border_color.0.clone();
            show_tooltip(tooltip_entity, &mut param.tooltip_q, transform.translation());
            _change_cursor_icon(&mut param.commands, &param.cursor_icons, *param.window, CursorType::Text);
            FaMouseEvent::send_over_event(&mut param.mouse_writer, WidgetType::TextInput, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut input_q: Query<
            (&mut BoxShadow, Option<&FamiqWidgetId>, Option<&FamiqTooltipEntity>),
            With<IsFamiqTextInput>
        >,
        mut param: InputPickingParam
    ) {
        if let Ok((mut box_shadow, id, tooltip_entity)) = input_q.get_mut(trigger.entity()) {
            box_shadow.color = Color::NONE;
            hide_tooltip(tooltip_entity, &mut param.tooltip_q);
            _change_cursor_icon(&mut param.commands, &param.cursor_icons, *param.window, CursorType::Default);
            FaMouseEvent::send_out_event(&mut param.mouse_writer, WidgetType::TextInput, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut input_q: Query<(Option<&FamiqWidgetId>, &mut FaTextEdit, &mut CosmicData), With<IsFamiqTextInput>>,
        mut famiq_res: ResMut<FamiqResource>,
        mut writer: EventWriter<FaMouseEvent>,
    ) {
        if let Ok((id, mut text_edit, mut cosmic_data)) = input_q.get_mut(trigger.entity()) {
            famiq_res.update_all_focus_states(false);
            famiq_res.update_or_insert_focus_state(trigger.entity(), true);

            if let Some(editor) = cosmic_data.editor.as_mut() {
                editor.set_selection(Selection::None);
                text_edit.clear_selection();
            }
            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_down_event(&mut writer, WidgetType::TextInput, trigger.entity(), id, true);
            } else {
                FaMouseEvent::send_down_event(&mut writer, WidgetType::TextInput, trigger.entity(), id, false);
            }
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut input_q: Query<Option<&FamiqWidgetId>, With<IsFamiqTextInput>>,
        mut writer: EventWriter<FaMouseEvent>,
    ) {
        if let Ok(id) = input_q.get_mut(trigger.entity()) {
            FaMouseEvent::send_up_event(&mut writer, WidgetType::TextInput, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    pub(crate) fn on_request_redraw_editor_buffer(mut param: RequestRedrawBufferParam) {
        for request in param.request_redraw.read() {
            if let Ok((mut cosmic_data, cosmic_color, texture_entity)) = param.input_q.get_mut(request.input_entity) {
                let CosmicData {editor, buffer_dim, .. } = &mut *cosmic_data;

                if let Some(editor) = editor.as_mut() {
                    let pixels = helper::draw_editor_buffer(
                        buffer_dim,
                        &mut param.font_system.0,
                        &mut param.swash_cache.0,
                        editor,
                        cosmic_color.text_color,
                        cosmic_color.cursor_color,
                        cosmic_color.selection_color,
                        cosmic_color.selected_text_color,
                    );

                    let texture = param.texture_q.get(texture_entity.0).unwrap();
                    if let Some(image) = param.image_asset.get_mut(texture.image.id()) {
                        let new_size = Extent3d {
                            width: buffer_dim.x as u32,
                            height: buffer_dim.y as u32,
                            depth_or_array_layers: 1,
                        };
                        if image.texture_descriptor.size != new_size {
                            image.resize(new_size);
                        }
                        image.data.copy_from_slice(&pixels);
                    }
                }
            }
        }
    }

    pub(crate) fn detect_new_text_input_widget_system(
        mut input_q: Query<
            (
                Entity,
                Option<&FamiqWidgetId>,
                &CosmicTextData,
                &CosmicDataColor,
                &mut FaTextEdit,
                &mut CosmicData,
            ),
            Added<IsFamiqTextInput>
        >,
        mut font_system: ResMut<CosmicFontSystem>,
        mut swash_cache: ResMut<CosmicSwashCache>,
        mut input_res: ResMut<FaTextInputResource>,
        mut commands: Commands,
        mut image_assets: ResMut<Assets<Image>>,
        font_assets: Res<Assets<Font>>,
        window: Single<&Window>
    ) {
        input_q.iter_mut().for_each(|(entity, id, text_data, cosmic_color, mut text_edit, mut cosmic_data)| {
            if let Some(id) = id {
                if !input_res.exists(id.0.as_str()) {
                    input_res._insert(id.0.clone(), String::new());
                }
            }
            let mut attrs = Attrs::new();
            if let Some(font) = font_assets.get(&text_data.handle) {
                let data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new((*font.data).clone());

                let face_ids = font_system.0
                    .db_mut()
                    .load_font_source(cosmic_text::fontdb::Source::Binary(data));

                // Get face ID for Attrs.
                // ref: https://github.com/bevyengine/bevy/blob/main/crates/bevy_text/src/pipeline.rs#L170
                if let Some(&face_id) = face_ids.last() {
                    let face = font_system.0.db().face(face_id).unwrap().clone();
                    let family_name = face.families[0].0.clone();
                    let family_name: &'static str = Box::leak(family_name.into_boxed_str());

                    attrs = Attrs::new()
                        .family(Family::Name(family_name))
                        .weight(Weight(face.weight.0))
                        .stretch(face.stretch)
                        .style(face.style);
                }
            }

            let metrics = Metrics::relative(text_data.size, 1.2).scale(window.scale_factor());
            let mut buffer = Buffer::new(&mut font_system.0, metrics);
            let mut buffer = buffer.borrow_with(&mut font_system.0);
            buffer.set_text(&text_edit.placeholder, attrs, Shaping::Advanced);

            if let Some(layout) = buffer.line_layout(0) {
                text_edit.text_width = layout[0].w;
                text_edit.text_height = metrics.line_height;

                let buffer_dim = Vec2::new(
                    text_edit.text_width + 5.0,
                    text_edit.text_height + 5.0
                );

                buffer.set_size(Some(buffer_dim.x), Some(buffer_dim.y));
                buffer.shape_until_scroll(true);
                buffer.shape_until_cursor(Cursor::new(0, 0), true);

                let mut editor = Editor::new(buffer.clone());

                let texture_width = buffer_dim.x as u32;
                let texture_height = buffer_dim.y as u32;

                let pixels = helper::draw_editor_buffer(
                    &buffer_dim,
                    &mut font_system.0,
                    &mut swash_cache.0,
                    &mut editor,
                    cosmic_color.text_color,
                    cosmic_color.cursor_color,
                    cosmic_color.selection_color,
                    cosmic_color.selected_text_color
                );
                let mut texture = Image::new_fill(
                    Extent3d {
                        width: texture_width,
                        height: texture_height,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &pixels,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::default()
                );

                // - using linear makes text look clean but faint.
                // - using nearest makes text clear but ugly.
                // what's up window?
                // if cfg!(target_os = "windows") {
                    texture.sampler = ImageSampler::linear();
                // }

                let texture_handle = image_assets.add(texture);
                let texture_image = commands
                    .spawn((
                        ImageNode::new(texture_handle),
                        Node {
                            left: Val::Px(0.0),
                            ..default()
                        },
                        IsFamiqTextInputBufferTexture
                    ))
                    .observe(FaTextInput::handle_buffer_texture_on_mouse_down)
                    .observe(FaTextInput::handle_buffer_texture_on_start_selection)
                    .observe(FaTextInput::handle_buffer_texture_on_selecting)
                    .id();

                commands.entity(entity)
                    .insert(FaTextInputBufferTextureEntity(texture_image))
                    .add_child(texture_image);

                commands.entity(texture_image).insert(FaTextInputEntity(entity));

                cosmic_data.editor = Some(editor);
                cosmic_data.attrs = Some(attrs);
                cosmic_data.metrics = Some(metrics);
                cosmic_data.buffer_dim = buffer_dim;
            }
        });
    }

    /// Internal system to detect text's style (font-size & color) changes
    pub(crate) fn detect_text_input_text_style_change(mut param: DetectTextStyleChangeParam) {
        param.input_q.iter_mut().for_each(|(entity, mut cosmic_data, mut cosmic_color, mut text_edit, text_font, text_color)| {
            if let Some(_cosmic_color) = bevy_color_to_cosmic_rgba(text_color.0) {
                cosmic_color.text_color = _cosmic_color;
                cosmic_color.selected_text_color = _cosmic_color;

                if let Some(is_focused) = param.famiq_res.get_widget_focus_state(&entity) {
                    if is_focused {
                        cosmic_color.cursor_color = _cosmic_color;
                    } else {
                        cosmic_color.cursor_color = CURSOR_INVISIBLE;
                    }
                }
            }
            let CosmicData { editor, attrs, buffer_dim, .. } = &mut *cosmic_data;

            if text_font.font_size > 0.0 {
                if let Some(editor) = editor {
                    let current_cursor = editor.cursor();

                    let font_system = &mut param.font_system.0;

                    editor.with_buffer_mut(|buffer| {
                        buffer.set_size(font_system, None, None); // reset

                        let new_metrics = Metrics::relative(text_font.font_size, 1.2).scale(param.window.scale_factor());
                        buffer.set_metrics(font_system, new_metrics);

                        if text_edit.value.is_empty() {
                            buffer.set_text(font_system, &text_edit.placeholder, attrs.unwrap(), Shaping::Advanced);
                        } else {
                            buffer.set_text(font_system, &text_edit.value, attrs.unwrap(), Shaping::Advanced);
                        }

                        if let Some(layout) = buffer.line_layout(font_system, 0) {
                            text_edit.text_width = layout[0].w;
                            text_edit.text_height = new_metrics.line_height;

                            *buffer_dim = Vec2::new(
                                text_edit.text_width + 5.0,
                                text_edit.text_height + 5.0
                            );
                            buffer.set_size(font_system, Some(buffer_dim.x), Some(buffer_dim.y));
                            buffer.shape_until_scroll(font_system, true);
                            buffer.shape_until_cursor(font_system, current_cursor, true);
                        }
                    });
                    param.request_redraw.send(RequestRedrawBuffer::new(entity));
                }
            }
        });
    }

    pub(crate) fn handle_text_input_on_typing(mut param: TypingParam) {
        for e in param.evr_kbd.read() {
            if e.state == ButtonState::Released { // it's key up? nevermind
                continue;
            }

            for(entity, computed, texture_entity, mut blink_timer, mut cosmic_data, mut text_edit, id) in param.input_q.iter_mut() {
                let Some(focused) = param.famiq_res.get_widget_focus_state(&entity) else { continue };

                if !focused {
                    continue;
                }
                let mut texture_node = param.texture_q.get_mut(texture_entity.0).unwrap();

                text_edit.widget_computed = computed.clone();
                text_edit.set_min_max_cursor_pos();

                let CosmicData { buffer_dim, attrs, editor, .. } = &mut *cosmic_data;

                if let Some(mut editor) = editor.as_mut() {
                    let mut skip_typing = false;
                    let current_cursor = editor.cursor();
                    let font_system = &mut param.font_system.0;

                    if text_edit.is_ctrl_a_pressed(&param.keys, e.key_code) {
                        if text_edit.select_all(&mut editor) {
                            helper::scroll_left_end(&mut texture_node);
                            param.request_redraw.send(RequestRedrawBuffer::new(entity));
                            continue;
                        }
                    }
                    else if text_edit.is_ctrl_c_pressed(&param.keys, e.key_code) {
                        if let Some(copied_text) = text_edit.copy_text() {
                            param.famiq_res.copied_text = copied_text;
                            continue;
                        }
                    }
                    else if text_edit.is_ctrl_v_pressed(&param.keys, e.key_code) {
                        // TODO: proper scroll after pasted long text
                        let mut ctx = Clipboard::new().unwrap();
                        let mut copied_text = ctx.get_text().ok();

                        if copied_text.as_ref().map_or(true, |s| s.is_empty()) {
                            if !param.famiq_res.copied_text.is_empty() {
                                copied_text = Some(param.famiq_res.copied_text.clone());
                            }
                        }
                        if let Some(text) = copied_text {
                            if text.is_empty() {
                                continue;
                            }
                            helper::clear_buffer_before_insert(&mut editor, &mut text_edit, font_system, attrs.unwrap());

                            let index = text_edit.cursor_index;
                            text_edit.value.insert_str(index, &text);
                            text_edit.cursor_index += text.len();

                            editor.with_buffer_mut(|buffer| {
                                buffer.set_size(font_system, None, None); // reset
                                buffer.set_text(font_system, &text_edit.value, attrs.unwrap(), Shaping::Advanced);
                                helper::update_buffer_text_layout(
                                    font_system,
                                    &mut text_edit,
                                    buffer_dim,
                                    buffer,
                                    &texture_node
                                );
                            });
                            skip_typing = true;
                        }
                    }

                    if !skip_typing {
                        match &e.logical_key {
                            Key::Character(key_input) => {
                                helper::clear_buffer_before_insert(&mut editor, &mut text_edit, font_system, attrs.unwrap());

                                if !text_edit.selected_text.is_empty() {
                                    editor.delete_selection();
                                    text_edit.remove_selected_text();
                                }
                                text_edit.insert(key_input);

                                let b = key_input.as_bytes();
                                for c in b {
                                    let c: char = (*c).into();
                                    editor.action(font_system, Action::Insert(c));
                                }
                            }
                            Key::Space => {
                                helper::clear_buffer_before_insert(&mut editor, &mut text_edit, font_system, attrs.unwrap());

                                if !text_edit.selected_text.is_empty() {
                                    editor.delete_selection();
                                    text_edit.remove_selected_text();
                                }
                                text_edit.insert(&SmolStr::new(" "));
                                editor.action(font_system, Action::Insert(' '));
                            }
                            Key::Backspace => {
                                if !text_edit.selected_text.is_empty() {
                                    editor.delete_selection();
                                    text_edit.remove_selected_text();
                                }
                                else {
                                    text_edit.remove();
                                    editor.action(font_system, Action::Backspace);
                                }
                            }
                            Key::Escape => {
                                text_edit.clear_selection();
                                editor.action(font_system, Action::Escape);
                            }
                            Key::ArrowLeft => {
                                text_edit.move_cursor_left();
                                helper::update_selection_state_on_arrow_keys(&mut text_edit, &mut editor);
                            }
                            Key::ArrowRight => {
                                text_edit.move_cursor_right();
                                helper::update_selection_state_on_arrow_keys(&mut text_edit, &mut editor);
                            }
                            _ => {}
                        }
                    }

                    editor.with_buffer_mut(|buffer| {
                        buffer.set_size(font_system, None, None); // Reset

                        if text_edit.value.is_empty() {
                            text_edit.buffer_empty = false;
                            buffer.set_text(font_system, &text_edit.placeholder, attrs.unwrap(), Shaping::Advanced);
                        }
                        helper::update_buffer_text_layout(
                            font_system,
                            &mut text_edit,
                            buffer_dim,
                            buffer,
                            &texture_node
                        );
                        buffer.shape_until_scroll(font_system, true);
                        buffer.shape_until_cursor(font_system, current_cursor, true);
                    });
                    editor.set_cursor(Cursor::new(0, text_edit.cursor_index));

                    match text_edit.need_scroll {
                        NeedScroll::Right => helper::scroll_right(&mut texture_node, &text_edit),
                        NeedScroll::Left => helper::scroll_left(&mut texture_node, &text_edit),
                        _ => {}
                    }
                    blink_timer.can_blink = false;
                    param.change_writer.send(FaValueChangeEvent::new(
                        entity,
                        id.map(|_id| _id.0.clone()),
                        text_edit.value.clone(),
                        Vec::new()
                    ));
                    param.request_redraw.send(RequestRedrawBuffer::new(entity));
                    if let Some(id) = id {
                        param.input_res._insert(id.0.clone(), text_edit.value.clone());
                    }
                }
            }
        }
    }

    pub(crate) fn handle_cursor_blink_system(
        mut input_q: Query<
            (Entity, &mut CursorBlinkTimer, &mut CosmicDataColor),
            With<IsFamiqTextInput>
        >,
        mut request_redraw_buffer: EventWriter<RequestRedrawBuffer>,
        famiq_res: Res<FamiqResource>,
        time: Res<Time>,
    ) {
        for (entity, mut blink_timer, mut cosmic_color) in input_q.iter_mut() {
            let need_redraw;
            let is_focused = famiq_res.get_widget_focus_state(&entity) == Some(true);

            blink_timer.timer.tick(time.delta());

            if is_focused {
                need_redraw = helper::handle_cursor_blink_on_focused(
                    &mut blink_timer,
                    &mut cosmic_color
                );
            }
            else {
                need_redraw = helper::handle_cursor_blink_on_unfocused(
                    &mut blink_timer,
                    &mut cosmic_color
                );
            }
            if need_redraw {
                request_redraw_buffer.send(RequestRedrawBuffer::new(entity));
            }
            blink_timer.can_blink = true;
        }
    }
}

/// Builder for creating and customizing `FaTextInput` widgets.
pub struct FaTextInputBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub placeholder: String,
    pub root_node: EntityCommands<'a>,
    pub input_type: TextInputType
}

impl<'a> FaTextInputBuilder<'a> {
    pub fn new(
        placeholder: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>
    ) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            placeholder,
            root_node,
            input_type: TextInputType::Text
        }
    }

    /// Method to set text_input type as password
    pub fn is_password(mut self) -> Self {
        self.input_type = TextInputType::Password;
        self
    }

    /// Spawn text input into UI World.
    pub fn build(&mut self) -> Entity {
        self._process_built_in_size_class();
        self._process_built_in_color_class();
        self._node();
        FaTextInput::new(
            &self.attributes,
            self.placeholder.as_str(),
            &mut self.root_node,
            self.input_type.clone()
        )
    }
}

impl<'a> SetWidgetAttributes for FaTextInputBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_input_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaTextInputBuilder`
pub fn fa_text_input<'a>(
    builder: &'a mut FamiqBuilder,
    placeholder: &str
) -> FaTextInputBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaTextInputBuilder::new(
        placeholder.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

/// Determines if text_input internal system(s) can run.
///
/// True only if there is a text_input widget created.
pub fn can_run_text_input_systems(input_q: Query<&IsFamiqTextInput>) -> bool {
    !input_q.is_empty()
}
