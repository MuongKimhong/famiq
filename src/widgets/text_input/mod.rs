/// Suport single line only. Bugs are waiting for you somewhere.

pub mod styling;
pub mod components;
pub mod tests;
pub mod text_edit;

use styling::*;
pub use components::*;
pub use text_edit::*;
use crate::event_writer::{FaMouseEvent, FaValueChangeEvent};
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::color::WHITE_COLOR;
use crate::widgets::*;

use bevy::input::keyboard::{Key, KeyboardInput, KeyCode};
use bevy::input::ButtonInput;
use bevy::ecs::system::EntityCommands;
use bevy::text::TextLayoutInfo;
use bevy::input::ButtonState;
use bevy::prelude::*;
use smol_str::SmolStr;
use arboard::Clipboard;

#[cfg(target_os = "linux")]
use arboard::{SetExtLinux, LinuxClipboardKind};

use super::color::{BLACK_COLOR, SECONDARY_COLOR};


#[derive(Default)]
pub struct IsFamiqTextInputResource;
pub type FaTextInputResource = InputResource<IsFamiqTextInputResource>;

/// Handles the blinking behavior of the text input cursor.
#[derive(Resource, Debug)]
pub struct FaTextInputCursorBlinkTimer {
    pub timer: Timer, // change bg color every 0.5 second
    pub is_transparent: bool
}

impl Default for FaTextInputCursorBlinkTimer {
    fn default() -> Self {
        FaTextInputCursorBlinkTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            is_transparent: false
        }
    }
}

/// The width of the text input cursor.
pub const CURSOR_WIDTH: f32 = 1.0;

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
                Node {
                    left: Val::Px(1.0),
                    ..default()
                }
            ))
            .observe(FaTextInput::handle_placeholder_on_mouse_down)
            .observe(FaTextInput::handle_placeholder_start_selection)
            .observe(FaTextInput::handle_placeholder_selecting)
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    fn _build_cursor(root_node: &'a mut EntityCommands, color: &WidgetColor) -> Entity {
        let use_color = get_text_color(color);
        root_node
            .commands()
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(1.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(use_color),
                BorderRadius::all(Val::Px(0.0)),
                BorderColor(use_color),
                ZIndex(10),
                Visibility::Hidden,
                IsFamiqTextInputCursor
            ))
            .id()
    }

    fn _build_highlighter(root_node: &'a mut EntityCommands) -> Entity {
        let use_color = Color::srgba(0.537, 0.686, 0.969, 0.5);
        root_node
            .commands()
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(use_color),
                BorderRadius::all(Val::Px(0.0)),
                BorderColor(use_color),
                ZIndex(11),
                Visibility::Hidden,
                IsFamiqTextInputHighlighter
            ))
            .id()
    }

    fn _build_input(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
        input_type: TextInputType
    ) -> Entity {
        let input_color = get_color(&attributes.color);
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
                FaTextEdit::default()
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
        let cursor_entity = Self::_build_cursor(root_node, &attributes.color);
        let highlighter_entity = Self::_build_highlighter(root_node);
        let ph_entity = Self::_build_placeholder(attributes, placeholder, root_node);
        let input_entity = Self::_build_input(attributes, root_node, placeholder, input_type);

        if attributes.has_tooltip {
            build_tooltip_node(attributes, root_node, input_entity);
        }
        root_node.commands().entity(input_entity).insert((
            FaTextInputPlaceholderEntity(ph_entity),
            FaTextInputCursorEntity(cursor_entity),
            FaTextInputHighlighterEntity(highlighter_entity)
        ));
        root_node.commands().entity(ph_entity).insert(FaTextInputEntity(input_entity));

        entity_add_children(root_node, &vec![ph_entity, cursor_entity, highlighter_entity], input_entity);
        input_entity
    }

    pub(crate) fn handle_placeholder_on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut input_q: Query<(&GlobalTransform, &ComputedNode, &mut FaTextEdit, &FaTextInputHighlighterEntity), With<IsFamiqTextInput>>,
        mut highlighter_q: Query<&mut Visibility, (With<IsFamiqTextInputHighlighter>, Without<IsFamiqTextInputPlaceholder>)>,
        mut famiq_res: ResMut<FamiqResource>,
        ph_q: Query<(&Node, &FaTextInputEntity), With<IsFamiqTextInputPlaceholder>>,
    ) {
        let (ph_node, input_entity) = ph_q.get(trigger.entity()).unwrap();
        let (input_transform, input_computed, mut text_edit, hl_entity) = input_q.get_mut(input_entity.0).unwrap();

        if !text_edit.value.is_empty() {
            let mut hl_visibility = highlighter_q.get_mut(hl_entity.0).unwrap();

            let local_pointer_pos = mouse_pos_to_local_node_pos(
                &trigger.pointer_location.position,
                input_computed,
                input_transform
            );
            let mut closest_glyph_index = None;
            let mut closest_distance = f32::MAX;

            for i in 0..=text_edit.value.len() {
                let cursor_pos = text_edit.calculate_cursor_pos(&ph_node, i as f32);
                let distance = (local_pointer_pos.x - cursor_pos).abs();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_glyph_index = Some(i);
                }
            }

            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
            }

            famiq_res.update_all_focus_states(false);
            famiq_res.update_or_insert_focus_state(input_entity.0, true);
            *hl_visibility = Visibility::Hidden;
            trigger.propagate(false);
        }
        else {
            trigger.propagate(true);
        }
    }

    pub(crate) fn handle_placeholder_start_selection(
        mut trigger: Trigger<Pointer<DragStart>>,
        mut input_q: Query<(&GlobalTransform, &ComputedNode, &mut FaTextEdit, &FaTextInputHighlighterEntity), With<IsFamiqTextInput>>,
        mut highlighter_q: Query<(&mut Node, &mut Visibility), (With<IsFamiqTextInputHighlighter>, Without<IsFamiqTextInputPlaceholder>)>,
        ph_q: Query<(&Node, &FaTextInputEntity), With<IsFamiqTextInputPlaceholder>>
    ) {
        let (ph_node, input_entity) = ph_q.get(trigger.entity()).unwrap();
        let (input_transform, input_computed, mut text_edit, highlighter_entity) = input_q.get_mut(input_entity.0).unwrap();
        let (mut highlighter_node, mut highlighter_visibility) = highlighter_q.get_mut(highlighter_entity.0).unwrap();

        if !text_edit.value.is_empty() {
            let local_pointer_pos = mouse_pos_to_local_node_pos(
                &trigger.pointer_location.position,
                input_computed,
                input_transform
            );
            let mut closest_glyph_index = None;
            let mut closest_distance = f32::MAX;

            for i in 0..=text_edit.value.len() {
                let cursor_pos = text_edit.calculate_cursor_pos(&ph_node, i as f32);
                let distance = (local_pointer_pos.x - cursor_pos).abs();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_glyph_index = Some(i);
                }
            }


            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
                text_edit.clear_selection();
                text_edit.selection_start_index = Some(glyph_index);

                *highlighter_visibility = Visibility::Visible;
                highlighter_node.top = Val::Px(input_computed.padding().top * input_computed.inverse_scale_factor());
            }
            trigger.propagate(false);
        }
        else {
            trigger.propagate(true);
        }
    }

    pub(crate) fn handle_placeholder_selecting(
        mut trigger: Trigger<Pointer<Drag>>,
        mut input_q: Query<(&GlobalTransform, &ComputedNode, &mut FaTextEdit, &FaTextInputHighlighterEntity), With<IsFamiqTextInput>>,
        mut highlighter_q: Query<&mut Node, (With<IsFamiqTextInputHighlighter>, Without<IsFamiqTextInputPlaceholder>)>,
        ph_q: Query<(&Node, &FaTextInputEntity), With<IsFamiqTextInputPlaceholder>>
    ) {
        let (ph_node, input_entity) = ph_q.get(trigger.entity()).unwrap();
        let (input_transform, input_computed, mut text_edit, highlighter_entity) = input_q.get_mut(input_entity.0).unwrap();
        let mut highlighter_node = highlighter_q.get_mut(highlighter_entity.0).unwrap();

        if !text_edit.value.is_empty() {
            let local_pointer_pos = mouse_pos_to_local_node_pos(
                &trigger.pointer_location.position,
                input_computed,
                input_transform
            );
            let mut closest_glyph_index = None;
            let mut closest_distance = f32::MAX;

            for i in 0..=text_edit.value.len() {
                let cursor_pos = text_edit.calculate_cursor_pos(&ph_node, i as f32);
                let distance = (local_pointer_pos.x - cursor_pos).abs();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_glyph_index = Some(i);
                }
            }

            if let Some(glyph_index) = closest_glyph_index {
                text_edit.cursor_index = glyph_index;
                text_edit.selection_end_index = Some(glyph_index);

                if let Some(start_index) = text_edit.selection_start_index {
                    let glyph_index_pos = text_edit.calculate_cursor_pos(ph_node, glyph_index as f32);
                    let start_index_pos = text_edit.calculate_cursor_pos(ph_node, start_index as f32);

                    if glyph_index < start_index {
                        highlighter_node.left = Val::Px(glyph_index_pos);
                        highlighter_node.width = Val::Px(start_index_pos - glyph_index_pos);
                        text_edit.selected_text = text_edit.value[glyph_index..start_index].to_string();

                    }
                    else if glyph_index > start_index {
                        highlighter_node.left = Val::Px(start_index_pos);
                        highlighter_node.width = Val::Px(glyph_index_pos - start_index_pos);
                        text_edit.selected_text = text_edit.value[start_index..glyph_index].to_string();
                    }
                }
            }
            trigger.propagate(false);
        }
        else {
            trigger.propagate(true);
        }
    }

    pub(crate) fn handle_text_input_on_focused(
        mut input_q: Query<(
            Entity,
            &FaTextInputCursorEntity,
            &FaTextInputPlaceholderEntity,
            &FaTextInputHighlighterEntity,
            &mut FaTextEdit
        )>,
        mut cursor_q: Query<(&mut Node, &mut Visibility), (With<IsFamiqTextInputCursor>, Without<IsFamiqTextInputHighlighter>)>,
        mut highlighter_q: Query<(&mut Node, &mut Visibility), (With<IsFamiqTextInputHighlighter>, Without<IsFamiqTextInputCursor>)>,
        mut placeholder_q: Query<(&Text, &TextLayoutInfo), With<IsFamiqTextInputPlaceholder>>,
        famiq_res: Res<FamiqResource>
    ) {
        if !famiq_res.is_changed() || famiq_res.is_added() {
            return;
        }

        for (
            input_entity,
            cursor_entity,
            placeholder_entity,
            highlighter_entity,
            mut text_edit
        ) in input_q.iter_mut() {

            let Some(focused) = famiq_res.get_widget_focus_state(&input_entity) else { continue };

            let (mut cursor_node, mut cursor_visibility) = cursor_q.get_mut(cursor_entity.0).unwrap();
            let (mut highlighter_node, mut highlighter_visbility) = highlighter_q.get_mut(highlighter_entity.0).unwrap();

            if let Ok((ph_text, ph_layout)) = placeholder_q.get_mut(placeholder_entity.0) {
                if focused {
                    *cursor_visibility = Visibility::Visible;
                    text_edit.char_width = ph_layout.size.x / ph_text.0.len() as f32;
                    text_edit.char_height = ph_layout.size.y;
                    cursor_node.width = Val::Px(CURSOR_WIDTH);
                    cursor_node.height = Val::Px(text_edit.char_height);
                    highlighter_node.height = Val::Px(text_edit.char_height);
                }
                else {
                    *cursor_visibility = Visibility::Hidden;
                    *highlighter_visbility = Visibility::Hidden;
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
        mut input_q: Query<(Option<&FamiqWidgetId>, &mut FaTextEdit, &FaTextInputHighlighterEntity), With<IsFamiqTextInput>>,
        mut highlighter_q: Query<&mut Visibility, With<IsFamiqTextInputHighlighter>>,
        mut famiq_res: ResMut<FamiqResource>,
        mut writer: EventWriter<FaMouseEvent>,
    ) {
        if let Ok((id, mut text_edit, highlighter_entity)) = input_q.get_mut(trigger.entity()) {
            famiq_res.update_all_focus_states(false);
            famiq_res.update_or_insert_focus_state(trigger.entity(), true);

            let mut highlighter_visibility = highlighter_q.get_mut(highlighter_entity.0).unwrap();
            *highlighter_visibility = Visibility::Hidden;
            text_edit.clear_selection();

            if text_edit.cursor_index > 0 {
                text_edit.cursor_index = text_edit.value.len();
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
    pub fn detect_new_text_input_widget_system(
        input_q: Query<Option<&FamiqWidgetId>, Added<IsFamiqTextInput>>,
        mut input_res: ResMut<FaTextInputResource>
    ) {
        for id in input_q.iter() {
            if let Some(id) = id {
                if !input_res.exists(id.0.as_str()) {
                    input_res._insert(id.0.clone(), String::new());
                }
            }
        }
    }

    pub(crate) fn update_placeholder(
        placeholder_text: &mut Text,
        text_input: &FaTextInputInfo,
        text_edit: &FaTextEdit
    ) {
        if text_edit.value.is_empty() {
            placeholder_text.0 = text_input.placeholder.clone();
            return;
        }

        if text_input.input_type == TextInputType::Password {
            placeholder_text.0 = mask_string(&text_edit.value);
        }
        else {
            placeholder_text.0 = text_edit.value.clone();
        }
    }

    fn scroll_right(ph_node: &mut Node, text_edit: &FaTextEdit) {
        let left_val = extract_val(ph_node.left).unwrap();

        if left_val > -text_edit.max_scroll_right() {
            ph_node.left = Val::Px(left_val - text_edit.char_width);
        }
    }

    fn scroll_left(ph_node: &mut Node, text_edit: &FaTextEdit) {
        let left_val = extract_val(ph_node.left).unwrap();

        if left_val < text_edit.max_scroll_left() - text_edit.char_width {
            ph_node.left = Val::Px(left_val + text_edit.char_width);
        } else {
            ph_node.left = Val::Px(1.0);
        }
    }

    fn scroll_left_end(ph_node: &mut Node) {
        ph_node.left = Val::Px(1.0);
    }

    pub(crate) fn detect_cursor_index_change(
        mut cursor_q: Query<&mut Node, (With<IsFamiqTextInputCursor>, Without<IsFamiqTextInputPlaceholder>)>,
        input_q: Query<(Ref<FaTextEdit>, &ComputedNode, &FaTextInputPlaceholderEntity, &FaTextInputCursorEntity)>,
        placeholder_q: Query<&Node, (With<IsFamiqTextInputPlaceholder>, Without<IsFamiqTextInputCursor>)>
    ) {
        for (text_edit, input_computed, ph_entity, cursor_entity) in input_q.iter() {
            if text_edit.is_changed() && !text_edit.is_added() {
                let padding_left = input_computed.padding().left * input_computed.inverse_scale_factor();
                let padding_top = input_computed.padding().top * input_computed.inverse_scale_factor();

                let mut cursor_node = cursor_q.get_mut(cursor_entity.0).unwrap();

                let ph_node = placeholder_q.get(ph_entity.0).unwrap();
                let ph_left = extract_val(ph_node.left).unwrap();

                let cursor_pos = (ph_left + padding_left) + (text_edit.cursor_index as f32 * text_edit.char_width);
                cursor_node.left = Val::Px(cursor_pos);
                cursor_node.top = Val::Px(padding_top);
                break;
            }
        }
    }

    pub(crate) fn handle_text_input_on_typing(
        mut evr_kbd: EventReader<KeyboardInput>,
        mut input_res: ResMut<FaTextInputResource>,
        mut input_q: Query<
            (
                Entity,
                &mut FaTextEdit,
                &FaTextInputInfo,
                &ComputedNode,
                &FaTextInputPlaceholderEntity,
                &FaTextInputHighlighterEntity,
                Option<&FamiqWidgetId>
            ),
            Without<IsFamiqTextInputCursor>
        >,
        mut placeholder_q: Query<(&mut Text, &mut Node), (With<IsFamiqTextInputPlaceholder>, Without<IsFamiqTextInputHighlighter>)>,
        mut highlighter_q: Query<(&mut Node, &mut Visibility), (With<IsFamiqTextInputHighlighter>, Without<IsFamiqTextInputPlaceholder>)>,
        mut change_writer: EventWriter<FaValueChangeEvent>,
        keys: Res<ButtonInput<KeyCode>>,
        mut famiq_res: ResMut<FamiqResource>
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released {
                continue;
            }

            for (input_entity, mut text_edit, input, input_computed, ph_entity, hl_entity, input_id) in input_q.iter_mut() {
                let Some(focused) = famiq_res.get_widget_focus_state(&input_entity) else { continue };

                if focused {
                    text_edit.widget_computed = input_computed.clone();
                    text_edit.set_min_max_cursor_pos();

                    if let Ok((mut ph_text, mut ph_node)) = placeholder_q.get_mut(ph_entity.0) {
                        let mut value_changed = false;
                        let mut skip_typing = false;
                        let (mut hl_node, mut hl_visibility) = highlighter_q.get_mut(hl_entity.0).unwrap();

                        if text_edit.is_ctrl_a_pressed(&keys, e.key_code) && !text_edit.value.is_empty() {
                            text_edit.select_all(&ph_node, &mut hl_node, &mut hl_visibility);
                            FaTextInput::scroll_left_end(&mut ph_node);
                            break;
                        }
                        else if text_edit.is_ctrl_c_pressed(&keys, e.key_code) {
                            if text_edit.selected_text.trim().is_empty() {
                                break;
                            }

                            let mut ctx = Clipboard::new().unwrap();

                            #[cfg(target_os = "linux")]
                            ctx.set().clipboard(LinuxClipboardKind::Clipboard).text(text_edit.selected_text.clone()).unwrap();

                            #[cfg(not(target_os = "linux"))]
                            ctx.set_text(text_edit.selected_text.clone()).unwrap();

                            if let Ok(copied_text) = ctx.get_text() {
                                famiq_res.copied_text = copied_text;
                            }
                            break;
                        }
                        else if text_edit.is_ctrl_v_pressed(&keys, e.key_code) {
                            let mut ctx = Clipboard::new().unwrap();
                            let mut copied_text = ctx.get_text().ok();

                            if copied_text.as_ref().map_or(true, |s| s.is_empty()) {
                                if !famiq_res.copied_text.is_empty() {
                                    copied_text = Some(famiq_res.copied_text.clone());
                                }
                            }
                            if let Some(text) = copied_text {
                                let text_len = text.len();
                                text_edit.insert(&SmolStr::new(&text));
                                text_edit.cursor_index += text_len.saturating_sub(1);

                                for _ in 0..text_len {
                                    text_edit.move_cursor_pos_right(&ph_node);
                                    FaTextInput::scroll_right(&mut ph_node, &text_edit);
                                }
                                value_changed = true;
                                skip_typing = true;
                            }
                        }

                        if !skip_typing {
                            match &e.logical_key {
                                Key::Character(key_input) => {
                                    if !text_edit.selected_text.is_empty() {
                                        text_edit.remove_selected_text(&mut hl_visibility);
                                    }
                                    text_edit.insert(key_input);
                                    text_edit.move_cursor_pos_right(&ph_node);
                                    value_changed = true;
                                }
                                Key::Space => {
                                    if !text_edit.selected_text.is_empty() {
                                        text_edit.remove_selected_text(&mut hl_visibility);
                                    }
                                    text_edit.insert(&SmolStr::new(" "));
                                    text_edit.move_cursor_pos_right(&ph_node);
                                    value_changed = true;
                                }
                                Key::Backspace => {
                                    if !text_edit.selected_text.is_empty() {
                                        text_edit.remove_selected_text(&mut hl_visibility);
                                    } else {
                                        text_edit.remove();
                                    }

                                    text_edit.move_cursor_pos_left_as_delete(&ph_node);
                                    value_changed = true;
                                }
                                Key::Escape => {
                                    *hl_visibility = Visibility::Hidden;
                                    text_edit.clear_selection();
                                }
                                Key::ArrowLeft => {
                                    let before = text_edit.cursor_index;
                                    text_edit.move_cursor_left();

                                    if text_edit.cursor_index < before {
                                        text_edit.move_cursor_pos_left(&ph_node);
                                    }
                                }
                                Key::ArrowRight => {
                                    let before = text_edit.cursor_index;
                                    text_edit.move_cursor_right();

                                    if text_edit.cursor_index > before {
                                        text_edit.move_cursor_pos_right(&ph_node);
                                    }
                                }
                                _ => continue
                            }
                        }

                        if value_changed {
                            FaTextInput::update_placeholder(&mut ph_text, &input, &text_edit);
                            change_writer.send(FaValueChangeEvent::new(
                                input_entity,
                                input_id.map(|_id| _id.0.clone()),
                                text_edit.value.clone(),
                                Vec::new()
                            ));
                            if let Some(id) = input_id {
                                input_res._insert(id.0.clone(), text_edit.value.clone());
                            }
                        }
                        if !skip_typing {
                            match text_edit.need_scroll {
                                NeedToScroll::Right =>  FaTextInput::scroll_right(&mut ph_node, &text_edit),
                                NeedToScroll::Left =>  FaTextInput::scroll_left(&mut ph_node, &text_edit),
                                _ => {}
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    pub fn handle_cursor_blink_system(
        time: Res<Time>,
        input_q: Query<(Entity, &FaTextInputCursorEntity, &BackgroundColor)>,
        mut cursor_q: Query<(&mut BackgroundColor, &IsFamiqTextInputCursor), Without<FaTextInputCursorEntity>>,
        mut cursor_blink_timer: ResMut<FaTextInputCursorBlinkTimer>,
        builder_res: Res<FamiqResource>
    ) {
        for (entity, cursor_entity, input_bg_color) in input_q.iter() {
            match builder_res.get_widget_focus_state(&entity) {
                Some(true) => {
                    if let Ok((mut bg_color, _)) = cursor_q.get_mut(cursor_entity.0) {
                        cursor_blink_timer.timer.tick(time.delta());

                        if cursor_blink_timer.timer.finished() {
                            if cursor_blink_timer.is_transparent {
                                if (input_bg_color.0 == SECONDARY_COLOR) || (input_bg_color.0 == BLACK_COLOR) {
                                    bg_color.0 = WHITE_COLOR;
                                }
                                else {
                                    bg_color.0 = BLACK_COLOR;
                                }
                            }
                            else {
                                *bg_color = BackgroundColor::default();
                            }
                            cursor_blink_timer.is_transparent = !cursor_blink_timer.is_transparent;
                        }
                        break;
                    }
                },
                _ => {}
            }
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
