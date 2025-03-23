/// Suport single line only. Bugs are waiting for you somewhere.

pub mod styling;
pub mod components;
pub mod tests;
pub mod text_edit;
pub mod helper;

use bevy::image::ImageSampler;
use styling::*;
pub use components::*;
pub use text_edit::*;
use crate::event_writer::{FaMouseEvent, FaValueChangeEvent};
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::*;

use bevy::render::render_resource::{TextureDimension, Extent3d, TextureFormat};
use bevy::input::keyboard::{Key, KeyboardInput, KeyCode};
use bevy::asset::RenderAssetUsages;
use bevy::input::ButtonInput;
use bevy::ecs::system::EntityCommands;
use bevy::text::TextLayoutInfo;
use bevy::input::ButtonState;
use bevy::prelude::*;
use cosmic_text::{
    Attrs, Metrics, Buffer, Editor, Family, Edit, Shaping, Weight, Cursor, Selection, Action
};
use smol_str::SmolStr;
use arboard::Clipboard;

#[derive(Default)]
pub struct IsFamiqTextInputResource;
pub type FaTextInputResource = InputResource<IsFamiqTextInputResource>;

/// Handles the blinking behavior of the text input cursor.
#[derive(Resource, Debug)]
pub struct FaTextInputCursorBlinkTimer {
    pub timer: Timer,
    pub can_blink: bool,
    pub is_transparent: bool
}

impl Default for FaTextInputCursorBlinkTimer {
    fn default() -> Self {
        FaTextInputCursorBlinkTimer {
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
            can_blink: true,
            is_transparent: false
        }
    }
}

/// Represents the Famiq text input widget, which includes placeholder text, a blinking cursor, and customizable styles.
/// Support UTF-8 encoded only.
pub struct FaTextInput;

// Needs container
impl<'a> FaTextInput {
    fn _build_placeholder(
        attributes: &WidgetAttributes,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let use_color = get_text_color(&attributes.color);
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                TextColor(use_color),
                TextLayout::new(JustifyText::Left, LineBreak::NoWrap),
                DefaultTextEntity::new(txt, txt_font, TextColor(use_color), TextLayout::new(JustifyText::Left, LineBreak::NoWrap)),
                IsFamiqTextInputPlaceholder,
                Node::default(),
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    fn _build_input(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
        input_type: TextInputType
    ) -> Entity {
        let input_color = get_color(&attributes.color);
        let placeholder_color = get_text_color(&attributes.color);
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.border_color = BorderColor(input_color);
        style_components.background_color = BackgroundColor(input_color);
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));

        let entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqTextInput,
                IsFamiqMainWidget,
                DefaultWidgetEntity::from(style_components),
                FaTextInputInfo::new(placeholder, input_type),
                FaTextEdit::new(placeholder),
                CosmicData::new(placeholder_color)
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
        input_type: TextInputType
    ) -> Entity {
        let ph_entity = Self::_build_placeholder(attributes, placeholder, root_node);
        let input_entity = Self::_build_input(attributes, root_node, placeholder, input_type);

        if attributes.has_tooltip {
            build_tooltip_node(attributes, root_node, input_entity);
        }
        root_node.commands().entity(input_entity).insert(FaTextInputPlaceholderEntity(ph_entity));
        root_node.commands().entity(ph_entity).insert(FaTextInputEntity(input_entity));

        entity_add_children(root_node, &vec![ph_entity], input_entity);
        input_entity
    }

    pub(crate) fn handle_buffer_texture_on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut input_q: Query<
            (&GlobalTransform, &ComputedNode, &mut FaTextEdit, &mut CosmicData),
            With<IsFamiqTextInput>
        >,
        texture_q: Query<(&Node, &FaTextInputEntity), With<IsFamiqTextInputBufferTexture>>,
        mut famiq_res: ResMut<FamiqResource>,
        mut font_system: ResMut<CosmicFontSystem>
    ) {
        let (texture_node, input_entity) = texture_q.get(trigger.entity()).unwrap();
        let (transform, computed, mut text_edit, mut cosmic_data) = input_q.get_mut(input_entity.0).unwrap();

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
            let current_cursor = editor.cursor();

            editor.with_buffer_mut(|buffer| {
                closest_glyph_index = helper::find_glyph_index_on_mouse_down(
                    buffer,
                    &mut font_system.0,
                    texture_node,
                    &mut text_edit,
                    local_pointer_pos.x
                );
                buffer.shape_until_scroll(&mut font_system.0, true);
                buffer.shape_until_cursor(&mut font_system.0, current_cursor, true);
            });
            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
                text_edit.clear_selection();
                editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                editor.action(&mut font_system.0, Action::Escape);
                famiq_res.update_all_focus_states(false);
                famiq_res.update_or_insert_focus_state(input_entity.0, true);
                trigger.propagate(false);
            }
        }
    }

    pub(crate) fn handle_buffer_texture_on_start_selection(
        mut trigger: Trigger<Pointer<DragStart>>,
        mut input_q: Query<
            (&GlobalTransform, &ComputedNode, &mut FaTextEdit, &mut CosmicData),
            With<IsFamiqTextInput>
        >,
        texture_q: Query<(&Node, &FaTextInputEntity), With<IsFamiqTextInputBufferTexture>>,
        mut famiq_res: ResMut<FamiqResource>,
        mut font_system: ResMut<CosmicFontSystem>
    ) {
        let (texture_node, input_entity) = texture_q.get(trigger.entity()).unwrap();
        let (transform, computed, mut text_edit, mut cosmic_data) = input_q.get_mut(input_entity.0).unwrap();

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
            let current_cursor = editor.cursor();

            editor.with_buffer_mut(|buffer| {
                closest_glyph_index = helper::find_glyph_index_on_mouse_down(
                    buffer,
                    &mut font_system.0,
                    texture_node,
                    &mut text_edit,
                    local_pointer_pos.x
                );
                buffer.shape_until_scroll(&mut font_system.0, true);
                buffer.shape_until_cursor(&mut font_system.0, current_cursor, true);
            });
            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
                text_edit.selection_start_index = Some(glyph_index);
                editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                famiq_res.update_all_focus_states(false);
                famiq_res.update_or_insert_focus_state(input_entity.0, true);
                trigger.propagate(false);
            }
        }
    }

    pub(crate) fn handle_buffer_texture_on_selecting(
        mut trigger: Trigger<Pointer<Drag>>,
        mut input_q: Query<
            (&GlobalTransform, &ComputedNode, &mut FaTextEdit, &mut CosmicData),
            With<IsFamiqTextInput>
        >,
        texture_q: Query<(&Node, &FaTextInputEntity), With<IsFamiqTextInputBufferTexture>>,
        mut famiq_res: ResMut<FamiqResource>,
        mut font_system: ResMut<CosmicFontSystem>
    ) {
        let (texture_node, input_entity) = texture_q.get(trigger.entity()).unwrap();
        let (transform, computed, mut text_edit, mut cosmic_data) = input_q.get_mut(input_entity.0).unwrap();

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
            let current_cursor = editor.cursor();

            editor.with_buffer_mut(|buffer| {
                closest_glyph_index = helper::find_glyph_index_on_mouse_down(
                    buffer,
                    &mut font_system.0,
                    texture_node,
                    &mut text_edit,
                    local_pointer_pos.x
                );

            });

            if let Some(glyph_index) = closest_glyph_index {
                let start_index = text_edit.selection_start_index.unwrap();
                text_edit.cursor_index = glyph_index;
                text_edit.selection_end_index = Some(glyph_index);
                editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                editor.set_selection(Selection::Normal(Cursor::new(0, start_index)));

                if start_index > glyph_index {
                    text_edit.selected_text = text_edit.value[glyph_index..start_index].to_owned();
                }
                else if start_index < glyph_index {
                    text_edit.selected_text = text_edit.value[start_index..glyph_index].to_owned();
                }
                famiq_res.update_all_focus_states(false);
                famiq_res.update_or_insert_focus_state(input_entity.0, true);
                trigger.propagate(false);

                editor.with_buffer_mut(|buffer| {
                    buffer.shape_until_scroll(&mut font_system.0, true);
                    buffer.shape_until_cursor(&mut font_system.0, current_cursor, true);
                    buffer.line_shape(&mut font_system.0, 0);
                    buffer.set_redraw(true);
                });
            }
        }
    }

    pub(crate) fn redraw_glyph_buffer(
        mut input_q: Query<(&FaTextInputBufferTextureEntity, &mut CosmicData)>,
        mut font_system: ResMut<CosmicFontSystem>,
        mut swash_cache: ResMut<CosmicSwashCache>,
        mut image_asset: ResMut<Assets<Image>>,
        texture_q: Query<(&Node, &ImageNode), With<IsFamiqTextInputBufferTexture>>,
    ) {
        for (buf_texture_entity, mut cosmic_data) in input_q.iter_mut() {
            let (node, texture) = texture_q.get(buf_texture_entity.0).unwrap();
            if node.display == Display::None {
                continue;
            }
            let text_color = cosmic_data.text_color;
            let cursor_color = cosmic_data.cursor_color;
            let selection_color = cosmic_data.selection_color;
            let selected_text_color = cosmic_data.selected_text_color;
            let buf_dim = cosmic_data.buffer_dim;

            if let Some(editor) = cosmic_data.editor.as_mut() {
                let texture_asset_id = texture.image.id();
                let mut total_text_width = buf_dim.x;

                editor.with_buffer_mut(|buffer| {
                    if let Some(width) = buffer.size().0 {
                        total_text_width = width;
                    }
                    buffer.line_shape(&mut font_system.0, 0);
                });
                let pixels = draw_editor_buffer(
                    total_text_width,
                    buf_dim.y,
                    &mut font_system.0,
                    &mut swash_cache.0,
                    editor,
                    text_color,
                    cursor_color,
                    selection_color,
                    selected_text_color
                );

                if let Some(image) = image_asset.get_mut(texture_asset_id) {
                    image.resize(Extent3d {
                        width: total_text_width as u32,
                        height: buf_dim.y as u32,
                        depth_or_array_layers: 1,
                    });
                    image.data.clear();
                    image.data.extend_from_slice(&pixels);
                }
            }
        }
    }

    pub(crate) fn detect_placeholder_computed_change(
        mut placeholder_q: Query<
            (&mut Node, &Text, &TextFont, &ComputedNode, &FaTextInputEntity, &IsFamiqTextInputPlaceholder),
            Changed<ComputedNode>
        >,
        font_assets: Res<Assets<Font>>,
        asset_server: Res<AssetServer>,
        mut input_q: Query<&mut CosmicData, With<IsFamiqTextInput>>,
        mut font_system: ResMut<CosmicFontSystem>,
        famiq_res: Res<FamiqResource>,
        mut commands: Commands,
        window: Single<&Window>
    ) {
        for (mut node, text, text_font, computed, input_entity, _) in placeholder_q.iter_mut() {
            let mut cosmic_data = input_q.get_mut(input_entity.0).unwrap();
            let width  = computed.size().x * computed.inverse_scale_factor();
            let height = computed.size().y * computed.inverse_scale_factor();


            if height > 0.0 && width > 0.0 {
                cosmic_data.buffer_dim = Vec2::new(width, height);

                if cosmic_data.editor.is_some() {
                    let editor = cosmic_data.editor.as_mut().unwrap();
                    let cursor = editor.cursor();
                    editor.with_buffer_mut(|buffer| {
                        buffer.set_size(&mut font_system.0, Some(width), Some(height));
                        buffer.shape_until_scroll(&mut font_system.0, true);
                        buffer.shape_until_cursor(&mut font_system.0, cursor, true);
                    });
                    if let Some(focused) = famiq_res.get_widget_focus_state(&input_entity.0) {
                        if focused {
                            node.display = Display::None;
                        }
                    }
                    continue;
                }
                if let Some(font) = font_assets.get(&text_font.font) {
                    font_system.0.db_mut().load_font_data((*font.data).clone());
                }
                let attrs = Attrs::new().family(Family::Monospace).weight(Weight::MEDIUM);
                let metrics = Metrics::relative(text_font.font_size, 1.0).scale(window.scale_factor());

                let mut buffer = Buffer::new(&mut font_system.0, metrics);
                buffer.set_text(&mut font_system.0, &text.0, attrs, Shaping::Advanced);
                buffer.set_size(&mut font_system.0, Some(width), Some(height));
                buffer.set_redraw(true);
                buffer.shape_until_scroll(&mut font_system.0, true);
                buffer.shape_until_cursor(&mut font_system.0, Cursor::new(0, 0), true);

                let mut editor = Editor::new(buffer.clone());
                let pixels: Vec<u8> = vec![0; (width * height) as usize * 4];

                let mut texture = Image::new_fill(
                    Extent3d {
                        width: width as u32,
                        height: height as u32,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &pixels,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::default()
                );
                texture.sampler = ImageSampler::linear();
                let texture_handle = asset_server.add(texture);
                let texture_image = commands
                    .spawn((
                        ImageNode::new(texture_handle),
                        Node {
                            display: Display::None,
                            left: Val::Px(0.0),
                            ..default()
                        },
                        IsFamiqTextInputBufferTexture
                    ))
                    .observe(FaTextInput::handle_buffer_texture_on_mouse_down)
                    .observe(FaTextInput::handle_buffer_texture_on_start_selection)
                    .observe(FaTextInput::handle_buffer_texture_on_selecting)
                    .id();

                commands.entity(input_entity.0)
                    .insert(FaTextInputBufferTextureEntity(texture_image))
                    .add_child(texture_image);

                commands.entity(texture_image).insert(FaTextInputEntity(input_entity.0));

                editor.set_redraw(true);
                cosmic_data.editor = Some(editor);
                cosmic_data.attrs = Some(attrs);
                cosmic_data.metrics = Some(metrics);
                // request_redraw.send(RequestBufferRedraw::new(input_entity.0));
            }
        }
    }

    pub(crate) fn handle_text_input_on_focused(
        mut input_q: Query<(
            Entity,
            &FaTextInputBufferTextureEntity,
            &FaTextInputPlaceholderEntity,
            &mut FaTextEdit,
            &mut CosmicData
        )>,
        mut texture_q: Query<&mut Node, (With<IsFamiqTextInputBufferTexture>, Without<IsFamiqTextInputPlaceholder>)>,
        mut placeholder_q: Query<
            (
                &mut Node,
                &Text,
                &TextLayoutInfo
            ),
            (With<IsFamiqTextInputPlaceholder>, Without<IsFamiqTextInputBufferTexture>)
        >,
        mut cursor_blink_timer: ResMut<FaTextInputCursorBlinkTimer>,
        famiq_res: Res<FamiqResource>
    ) {
        if !famiq_res.is_changed() || famiq_res.is_added() {
            return;
        }

        for (input_entity, buf_texture_entity, ph_entity, mut text_edit, mut cosmic_data) in input_q.iter_mut() {
            let mut texture_node = texture_q.get_mut(buf_texture_entity.0).unwrap();

            let Some(focused) = famiq_res.get_widget_focus_state(&input_entity) else { continue };

            if let Ok((mut ph_node, ph_text, ph_layout)) = placeholder_q.get_mut(ph_entity.0) {

                if focused {
                    if let Some(editor) = cosmic_data.editor.as_mut() {
                        editor.set_redraw(true);
                    }
                    text_edit.scroll_width = ph_layout.size.x / ph_text.0.len() as f32;
                    texture_node.display = Display::default();
                    ph_node.display = Display::None;

                    let duration =  cursor_blink_timer.timer.duration();
                    cursor_blink_timer.timer.set_elapsed(duration);
                }
                else {
                    if text_edit.value.is_empty() {
                        ph_node.display = Display::default();
                        texture_node.display = Display::None;
                    }
                }
            }
        }
    }

    fn handle_on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut input_q: Query<
            (&mut BoxShadow, &BorderColor, Option<&FamiqWidgetId>, &GlobalTransform, Option<&FamiqTooltipEntity>),
            With<IsFamiqTextInput>
        >,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        if let Ok((mut box_shadow, border_color, id, transform, tooltip_entity)) = input_q.get_mut(trigger.entity()) {
            box_shadow.color = border_color.0.clone();
            show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
            FaMouseEvent::send_over_event(&mut writer, WidgetType::TextInput, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut input_q: Query<
            (&mut BoxShadow, Option<&FamiqWidgetId>, Option<&FamiqTooltipEntity>),
            With<IsFamiqTextInput>
        >,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        if let Ok((mut box_shadow, id, tooltip_entity)) = input_q.get_mut(trigger.entity()) {
            box_shadow.color = Color::NONE;
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
            FaMouseEvent::send_out_event(&mut writer, WidgetType::TextInput, trigger.entity(), id);
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

    /// Internal system to detect new text_input being created.
    pub(crate) fn detect_new_text_input_widget_system(
        mut input_q: Query<Option<&FamiqWidgetId>, Added<IsFamiqTextInput>>,
        mut input_res: ResMut<FaTextInputResource>,
    ) {
        for id in input_q.iter_mut() {
            if let Some(id) = id {
                if !input_res.exists(id.0.as_str()) {
                    input_res._insert(id.0.clone(), String::new());
                }
            }
        }
    }

    /// Internal system to detect placeholder's style (font-size & color) changes
    pub(crate) fn detect_placeholder_style_change(
        mut placeholder_q: Query<
            (&mut Node, &TextFont, &TextColor, &FaTextInputEntity, &IsFamiqTextInputPlaceholder),
            Or<(Changed<TextFont>, Changed<TextColor>)>
        >,
        mut input_q: Query<&mut CosmicData, With<IsFamiqTextInput>>,
        mut font_system: ResMut<CosmicFontSystem>
    ) {
        placeholder_q.iter_mut().for_each(|(mut node, font, color, input_entity, _)| {
            let mut cosmic_data = input_q.get_mut(input_entity.0).unwrap();
            if let Some(cosmic_color) = bevy_color_to_cosmic_rgba(color.0) {
                cosmic_data.text_color = cosmic_color;
                cosmic_data.cursor_color = cosmic_color;
                cosmic_data.selected_text_color = cosmic_color;
            }

            if font.font_size > 0.0 {
                if let Some(editor) = cosmic_data.editor.as_mut() {
                    let cursor = editor.cursor();
                    editor.set_redraw(true);
                    editor.with_buffer_mut(|buffer| {
                        buffer.set_metrics(&mut font_system.0, Metrics::relative(font.font_size, 1.0));
                        buffer.shape_until_scroll(&mut font_system.0, true);
                        buffer.shape_until_cursor(&mut font_system.0, cursor, true);

                        // set placeholder node back to default to make its ComputedNode
                        // change inside detect_placeholder_computed_change system.
                        // Once buffer size is set, change the display back to None.
                        node.display = Display::default();
                    });
                }
            }
        });
    }

    pub(crate) fn handle_text_input_on_typing(
        mut input_q: Query<
            (
                Entity,
                &ComputedNode,
                &FaTextInputBufferTextureEntity,
                &mut CosmicData,
                &mut FaTextEdit,
                Option<&FamiqWidgetId>
            ),
            With<IsFamiqTextInput>
        >,
        mut texture_q: Query<&mut Node, With<IsFamiqTextInputBufferTexture>>,
        mut evr_kbd: EventReader<KeyboardInput>,
        mut input_res: ResMut<FaTextInputResource>,
        mut famiq_res: ResMut<FamiqResource>,
        mut font_system: ResMut<CosmicFontSystem>,
        mut change_writer: EventWriter<FaValueChangeEvent>,
        mut blink_timer: ResMut<FaTextInputCursorBlinkTimer>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released { // it's key up? nevermind
                continue;
            }

            for(entity, computed, texture_entity, mut cosmic_data, mut text_edit, id) in input_q.iter_mut() {
                let Some(focused) = famiq_res.get_widget_focus_state(&entity) else { continue };

                if !focused {
                    continue;
                }
                let mut texture_node = texture_q.get_mut(texture_entity.0).unwrap();

                text_edit.widget_computed = computed.clone();
                text_edit.set_min_max_cursor_pos();
                let buf_dim = cosmic_data.buffer_dim;
                let attrs = cosmic_data.attrs.unwrap();

                if let Some(editor) = cosmic_data.editor.as_mut() {
                    let mut skip_typing = false;
                    let current_cursor = editor.cursor();

                    if text_edit.is_ctrl_a_pressed(&keys, e.key_code) {
                        if text_edit.select_all(editor) {
                            helper::scroll_left_end(&mut texture_node);
                            continue;
                        }
                    }
                    else if text_edit.is_ctrl_c_pressed(&keys, e.key_code) {
                        if let Some(copied_text) = text_edit.copy_text() {
                            famiq_res.copied_text = copied_text;
                            continue;
                        }
                    }
                    else if text_edit.is_ctrl_v_pressed(&keys, e.key_code) {
                        // TODO: proper scroll after pasted long text
                        let mut ctx = Clipboard::new().unwrap();
                        let mut copied_text = ctx.get_text().ok();

                        if copied_text.as_ref().map_or(true, |s| s.is_empty()) {
                            if !famiq_res.copied_text.is_empty() {
                                copied_text = Some(famiq_res.copied_text.clone());
                            }
                        }
                        if let Some(text) = copied_text {
                            if text.is_empty() {
                                continue;
                            }
                            helper::clear_buffer_before_insert(editor, &mut text_edit, &mut font_system.0, attrs);

                            let index = text_edit.cursor_index;
                            text_edit.value.insert_str(index, &text);
                            text_edit.cursor_index += text.len();

                            editor.with_buffer_mut(|buffer| {
                                buffer.set_text(&mut font_system.0, &text_edit.value, attrs, Shaping::Advanced);
                                buffer.set_size(
                                    &mut font_system.0,
                                    Some(3000.0), // random, make sure it's enough for set_text().
                                    Some(buf_dim.y)
                                );
                            });
                            editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                            skip_typing = true;
                        }
                    }

                    if !skip_typing {
                        match &e.logical_key {
                            Key::Character(key_input) => {
                                helper::clear_buffer_before_insert(editor, &mut text_edit, &mut font_system.0, attrs);

                                if !text_edit.selected_text.is_empty() {
                                    editor.delete_selection();
                                    text_edit.remove_selected_text();
                                }
                                text_edit.insert(key_input);

                                let b = key_input.as_bytes();
                                for c in b {
                                    let c: char = (*c).into();
                                    editor.action(&mut font_system.0, Action::Insert(c));
                                }
                            }
                            Key::Space => {
                                helper::clear_buffer_before_insert(editor, &mut text_edit, &mut font_system.0, attrs);

                                if !text_edit.selected_text.is_empty() {
                                    editor.delete_selection();
                                    text_edit.remove_selected_text();
                                }
                                text_edit.insert(&SmolStr::new(" "));
                                editor.action(&mut font_system.0, Action::Insert(' '));
                            }
                            Key::Backspace => {
                                if !text_edit.selected_text.is_empty() {
                                    editor.delete_selection();
                                    text_edit.remove_selected_text();
                                }
                                else {
                                    text_edit.remove();
                                    editor.action(&mut font_system.0, Action::Backspace);
                                }
                            }
                            Key::Escape => {
                                text_edit.clear_selection();
                                editor.action(&mut font_system.0, Action::Escape);
                            }
                            Key::ArrowLeft => {
                                text_edit.move_cursor_left();
                                helper::update_selection_state_on_arrow_keys(&mut text_edit, editor);
                            }
                            Key::ArrowRight => {
                                text_edit.move_cursor_right();
                                helper::update_selection_state_on_arrow_keys(&mut text_edit, editor);
                            }
                            _ => {}
                        }
                    }

                    editor.with_buffer_mut(|buffer| {
                        if text_edit.value.is_empty() {
                            // if text empty, use buffer_dim.x which is placeholder computed width
                            text_edit.buffer_empty = false;
                            buffer.set_text(&mut font_system.0, &text_edit.placeholder_value, attrs, Shaping::Advanced);
                            buffer.set_size(&mut font_system.0, Some(buf_dim.x), Some(buf_dim.y));
                        }
                        else {
                            // line index is 0 because fa_text_input is single line.
                            if let Some(layout) = buffer.line_layout(&mut font_system.0, 0) {
                                text_edit.text_width = layout[0].w;
                                let glyphs = &layout[0].glyphs;
                                text_edit.check_need_scroll(glyphs, &texture_node);

                                // - give buffer width extra space to avoid weird rendering and also,
                                //   buffer width is not useful anywhere else.
                                buffer.set_size(&mut font_system.0, Some(text_edit.text_width + (text_edit.glyph_width * 2.0)), Some(buf_dim.y));
                            }
                        }
                        buffer.shape_until_scroll(&mut font_system.0, true);
                        buffer.shape_until_cursor(&mut font_system.0, current_cursor, true);
                    });
                    editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
                    editor.set_redraw(true);

                    match text_edit.need_scroll {
                        NeedScroll::Right => helper::scroll_right(&mut texture_node, &text_edit),
                        NeedScroll::Left => helper::scroll_left(&mut texture_node, &text_edit),
                        _ => {}
                    }
                    blink_timer.can_blink = false;
                    change_writer.send(FaValueChangeEvent::new(
                        entity,
                        id.map(|_id| _id.0.clone()),
                        text_edit.value.clone(),
                        Vec::new()
                    ));
                    if let Some(id) = id {
                        input_res._insert(id.0.clone(), text_edit.value.clone());
                    }
                }
            }
        }
    }

    pub(crate) fn handle_cursor_blink_system(
        mut input_q: Query<(Entity, &mut CosmicData), With<IsFamiqTextInput>>,
        mut blink_timer: ResMut<FaTextInputCursorBlinkTimer>,
        builder_res: Res<FamiqResource>,
        time: Res<Time>,
    ) {
        for (entity, mut cosmic_data) in input_q.iter_mut() {
            match builder_res.get_widget_focus_state(&entity) {
                Some(true) => {
                    if !blink_timer.can_blink {
                        blink_timer.is_transparent = false;
                        blink_timer.timer.reset();
                        cosmic_data.cursor_color = cosmic_data.text_color;
                        continue;
                    }

                    blink_timer.timer.tick(time.delta());

                    if blink_timer.timer.finished(){
                        if blink_timer.is_transparent {
                            cosmic_data.cursor_color = cosmic_data.text_color;
                        }
                        else {
                            cosmic_data.cursor_color = CURSOR_INVISIBLE;
                        }
                        blink_timer.is_transparent = !blink_timer.is_transparent;
                    }
                },
                _ => {
                    cosmic_data.cursor_color = CURSOR_INVISIBLE;
                }
            }
        }
        blink_timer.can_blink = true;
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
