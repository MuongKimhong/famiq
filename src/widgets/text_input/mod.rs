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
use crate::event_writer::*;
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::*;

use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::image::ImageSampler;
use bevy::asset::RenderAssetUsages;
use bevy::ecs::system::EntityCommands;
use bevy::input::ButtonState;
use bevy::prelude::*;
use cosmic_text::{
    Attrs, Metrics, Buffer, Editor, Family, Edit, Shaping, Weight, Cursor, Selection, Action
};
use arboard::Clipboard;
use std::sync::Arc;

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

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct TextInputMaterial {
    #[uniform(1)]
    color: Vec3,
    #[texture(2)]
    #[sampler(3)]
    texture: Handle<Image>,
}

impl UiMaterial for TextInputMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/text_input.wgsl").into()
    }
}

/// Represents the Famiq text input widget, which includes placeholder text, a blinking cursor, and customizable styles.
/// Support UTF-8 encoded only.
pub struct FaTextInput;

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
            color: placeholder_color
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
                CursorBlinkTimer::default(),
                text_data.clone(),
                DefaultCosmicTextEntity {
                    text_data
                }
            ))
            .observe(FaTextInput::handle_on_mouse_over)
            .observe(FaTextInput::handle_on_mouse_out)
            .observe(FaTextInput::handle_on_mouse_down)
            .observe(FaTextInput::handle_on_mouse_up)
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);

        if let Some(model_key) = attributes.model_key.as_ref() {
            root_node.commands().entity(entity).insert(ReactiveModelKey(model_key.to_owned()));
        }
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
            FaMouseEvent::send_event(
                &mut param.mouse_writer,
                EventType::Over,
                WidgetType::TextInput,
                trigger.entity(),
                id
            );
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
            FaMouseEvent::send_event(
                &mut param.mouse_writer,
                EventType::Out,
                WidgetType::TextInput,
                trigger.entity(),
                id
            );
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
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::TextInput, trigger.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::TextInput, trigger.entity(), id);
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
            FaMouseEvent::send_event(
                &mut writer,
                EventType::Up,
                WidgetType::TextInput,
                trigger.entity(),
                id
            );
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
                        cosmic_color
                    );

                    let (material_handle, image_node)= param.texture_q.get(texture_entity.0).unwrap();
                    if let Some(material) = param.materials.get_mut(material_handle) {
                        if let Color::Srgba(value) = cosmic_rgba_to_bevy_srgba(cosmic_color.text_color) {
                            material.color = Vec3::new(value.red, value.green, value.blue);
                        }
                        if let Some(texture) = param.image_asset.get_mut(material.texture.id()) {
                            let new_size = Extent3d {
                                width: buffer_dim.x as u32,
                                height: buffer_dim.y as u32,
                                depth_or_array_layers: 1,
                            };
                            if texture.texture_descriptor.size != new_size {
                                texture.resize(new_size);
                            }
                            texture.data.copy_from_slice(&pixels);
                        }
                    }
                    // resize ImageNode so that it can grow the Node size automatically.
                    // resizing Node directly will cause text shaking.
                    if let Some(image) = param.image_asset.get_mut(image_node.image.id()) {
                        let new_size = Extent3d {
                            width: buffer_dim.x as u32,
                            height: buffer_dim.y as u32,
                            depth_or_array_layers: 1,
                        };
                        if image.texture_descriptor.size != new_size {
                            image.resize(new_size);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn detect_new_text_input_widget_system(
        mut input_q: Query<
            (
                Entity,
                &CosmicTextData,
                &CosmicDataColor,
                &mut FaTextEdit,
                &mut CosmicData,
            ),
            Added<IsFamiqTextInput>
        >,
        mut font_system: ResMut<CosmicFontSystem>,
        mut swash_cache: ResMut<CosmicSwashCache>,
        mut commands: Commands,
        mut image_assets: ResMut<Assets<Image>>,
        mut materials: ResMut<Assets<TextInputMaterial>>,
        font_assets: Res<Assets<Font>>,
        window: Single<&Window>
    ) {
        input_q.iter_mut().for_each(|(entity, text_data, cosmic_color, mut text_edit, mut cosmic_data)| {
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

            let metrics = Metrics::relative(text_data.size, 1.2);

            #[cfg(not(target_os = "macos"))] {
                metrics.scale(window.scale_factor());
            }

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

                let pixels= helper::draw_editor_buffer(
                    &buffer_dim,
                    &mut font_system.0,
                    &mut swash_cache.0,
                    &mut editor,
                    cosmic_color
                );

                // need empty pixels at buffer size for ImageNode. see 'on_request_redraw_editor_buffer' system
                let empty_texture_handle = helper::create_empty_buffer_texture(&buffer_dim, &mut image_assets);
                let texture_handle = helper::create_buffer_texture(&buffer_dim, &pixels, &mut image_assets);

                if let Color::Srgba(value) = cosmic_rgba_to_bevy_srgba(cosmic_color.text_color) {
                    let texture_image = commands
                    .spawn((
                        ImageNode::new(empty_texture_handle),
                        MaterialNode(materials.add(TextInputMaterial {
                            color: Vec3::new(value.red, value.green, value.blue),
                            texture: texture_handle
                        })),
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
                }
                cosmic_data.editor = Some(editor);
                cosmic_data.attrs = Some(attrs);
                cosmic_data.metrics = Some(metrics);
                cosmic_data.buffer_dim = buffer_dim;
            }
        });
    }

    /// Internal system to detect text's style (font-size & color) changes
    pub(crate) fn detect_text_input_text_style_change(mut param: DetectTextStyleChangeParam) {
        param.input_q.iter_mut().for_each(|(
            entity,
            mut cosmic_data,
            mut cosmic_color,
            mut text_edit,
            cosmic_text_data
        )| {
            if let Some(_cosmic_color) = bevy_color_to_cosmic_rgba(cosmic_text_data.color) {
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

            if cosmic_text_data.size > 0.0 {
                if let Some(editor) = editor {
                    let current_cursor = editor.cursor();
                    let font_system = &mut param.font_system.0;

                    editor.with_buffer_mut(|buffer| {
                        buffer.set_size(font_system, None, None); // reset

                        let new_metrics = Metrics::relative(cosmic_text_data.size, 1.2);
                        #[cfg(not(target_os = "macos"))] {
                            new_metrics.scale(param.window.scale_factor());
                        }
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
                }
            }
            param.request_redraw.send(RequestRedrawBuffer::new(entity));
        });
    }

    pub(crate) fn handle_text_input_on_typing(mut param: TypingParam) {
        for e in param.evr_kbd.read() {
            if e.state == ButtonState::Released { // it's key up? nevermind
                continue;
            }

            for(
                entity,
                computed,
                texture_entity,
                mut blink_timer,
                mut cosmic_data,
                mut text_edit,
                model_key
            ) in param.input_q.iter_mut() {
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
                            Key::Character(key_input) => text_edit.insert_char(&mut editor, font_system, key_input, *attrs),
                            Key::Space => text_edit.insert_space(&mut editor, font_system, *attrs),
                            Key::Backspace => text_edit.backspace(&mut editor, font_system),
                            Key::Escape => text_edit.escape(&mut editor, font_system),
                            Key::ArrowLeft => text_edit.arrow_left(&mut editor),
                            Key::ArrowRight => text_edit.arrow_right(&mut editor),
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
                    if let Some(value) = param.fa_query.get_data_mut(&model_key.0) {
                        match value {
                            RVal::Str(v) => {
                                *v = text_edit.value.to_owned();
                            }
                            _ => {}
                        }
                    }
                    blink_timer.can_blink = false;
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
pub fn fa_text_input_builder<'a>(
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

#[macro_export]
macro_rules! fa_text_input {
    (
        $builder:expr,
        placeholder: $placeholder:expr
        $(, $($rest:tt)+)?
    ) => {{
        let mut text_input = fa_text_input_builder($builder, $placeholder);
        $(
            $crate::fa_text_input_attributes!(text_input, $($rest)+);
        )?
        text_input.build()
    }};
}

#[macro_export]
macro_rules! fa_text_input_attributes {
    ($text_input:ident, id: $id:expr $(, $($rest:tt)+)?) => {{
        $text_input = $text_input.id($id);
        $(
            $crate::fa_text_input_attributes!($text_input, $($rest)+);
        )?
    }};

    ($text_input:ident, class: $class:expr $(, $($rest:tt)+)?) => {{
        $text_input = $text_input.class($class);
        $(
            $crate::fa_text_input_attributes!($text_input, $($rest)+);
        )?
    }};

    ($text_input:ident, tooltip: $tooltip:expr $(, $($rest:tt)+)?) => {{
        $text_input = $text_input.tooltip($tooltip);
        $(
            $crate::fa_text_input_attributes!($text_input, $($rest)+);
        )?
    }};

    ($text_input:ident, display: $display:expr $(, $($rest:tt)+)?) => {{
        $text_input = $text_input.display($display);
        $(
            $crate::fa_text_input_attributes!($text_input, $($rest)+);
        )?
    }};

    ($text_input:ident, model: $model:expr $(, $($rest:tt)+)?) => {{
        $text_input = $text_input.model($model);
        $(
            $crate::fa_text_input_attributes!($text_input, $($rest)+);
        )?
    }};

    ($text_input:ident, password: $password:expr $(, $($rest:tt)+)?) => {{
        $text_input = $text_input.password($password);
        $(
            $crate::fa_text_input_attributes!($text_input, $($rest)+);
        )?
    }};
}


/// Determines if text_input internal system(s) can run.
///
/// True only if there is a text_input widget created.
pub fn can_run_text_input_systems(input_q: Query<&IsFamiqTextInput>) -> bool {
    !input_q.is_empty()
}
