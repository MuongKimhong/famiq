pub mod helper;

use helper::*;
use crate::utils;
use crate::widgets::color::WHITE_COLOR;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses, WidgetType};
use crate::event_writer::FaInteractionEvent;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::ecs::system::EntityCommands;
use bevy::text::TextLayoutInfo;
use bevy::input::ButtonState;
use bevy::utils::HashMap;
use bevy::prelude::*;
use smol_str::SmolStr;

use super::color::BLACK_COLOR;

#[derive(Component)]
pub struct TextInput {
    pub text: String,
    pub placeholder: String,
    pub focused: bool,
}

impl TextInput {
    pub fn new(text: &str, placeholder: &str) -> Self {
        Self {
            text: text.to_string(),
            placeholder: placeholder.to_string(),
            focused: false,
        }
    }
}

#[derive(Resource, Debug)]
pub struct FaTextInputResource {
    pub inputs: HashMap<String, String>,
}

impl FaTextInputResource {
    pub fn update_or_insert(&mut self, id: String, new_value: String) {
        if let Some(old_value) = self.inputs.get_mut(&id) {
            *old_value = new_value;
        } else {
            self.inputs.insert(id, "".to_string());
        }
    }
}

#[derive(Resource, Debug)]
pub struct FaTextInputCursorBlinkTimer {
    pub timer: Timer, // change bg color every 0.5 second
    pub is_transparent: bool
}

#[derive(Component)]
pub struct IsFamiqTextInput;

#[derive(Component)]
pub struct IsFamiqTextInputPlaceholder;

#[derive(Component)]
pub struct IsFamiqTextInputCursor;

#[derive(Component)]
pub struct FamiqTextInputPlaceholderEntity(pub Entity);

#[derive(Component)]
pub struct FamiqTextInputCursorEntity(pub Entity);

#[derive(Component)]
pub struct CharacterSize {
    pub width: f32,
    pub height: f32
}

pub const CURSOR_WIDTH: f32 = 2.0;

#[derive(PartialEq)]
pub enum TextInputColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

pub enum TextInputVariant {
    Default,
    Outlined,
    Underlined,
}

pub enum TextInputSize {
    Small,
    Normal,
    Large,
}

pub enum TextInputShape {
    Default,
    Round,
    Rectangle
}

pub struct FaTextInput;

// Needs container
impl<'a> FaTextInput {
    fn _build_placeholder(
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: &TextInputSize,
    ) -> Entity {
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                // FamiqWidgetId(format!("{id}_placeholder")),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqTextInputPlaceholder
            ))
            .id()
    }

    fn _build_cursor(root_node: &'a mut EntityCommands, color: &TextInputColor) -> Entity {
        let mut use_color = WHITE_COLOR;

        if *color == TextInputColor::Default {
            use_color = BLACK_COLOR;
        }
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
                BorderColor(WHITE_COLOR),
                ZIndex(10),
                Visibility::Hidden,
                IsFamiqTextInputCursor
            ))
            .id()
    }

    fn _build_input(
        id: &str,
        classes: &str,
        root_node: &'a mut EntityCommands,
        variant: TextInputVariant,
        color: TextInputColor,
        shape: TextInputShape,
        placeholder: &str,
        placeholder_entity: Entity,
        cursor_entity: Entity
    ) -> Entity {
        let mut node = default_input_node();
        let border_color = get_input_border_color(&color);
        let bg_color = get_input_background_color(&color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;
        let mut border_radius = outlined_border_radius();

        match shape {
            TextInputShape::Round => border_radius = round_border_radius(),
            TextInputShape::Rectangle => border_radius = rectangle_border_radius(),
            _ => ()
        }

        match variant {
            TextInputVariant::Underlined => {
                border_radius = underlined_border_radius();
                node.border = UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(2.0),
                }
            }
            _ => (),
        }

        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                bg_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                BoxShadow::default(),
                FamiqWidgetId(id.to_string()),
                FamiqWidgetClasses(classes.to_string()),
                IsFamiqTextInput,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                TextInput::new("", placeholder),
                Interaction::default(),
                FamiqTextInputPlaceholderEntity(placeholder_entity),
                FamiqTextInputCursorEntity(cursor_entity),
            ))
            .insert(CharacterSize { width: 0.0, height: 0.0 })
            .id()
    }

    pub fn new(
        id: &str,
        classes: &str,
        ph: &str, // placeholder
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: TextInputSize,
        variant: TextInputVariant,
        color: TextInputColor,
        shape: TextInputShape
    ) -> Entity {
        let cursor_entity = Self::_build_cursor(root_node, &color);
        let ph_entity = Self::_build_placeholder(ph, root_node, asset_server, font_path, &size);
        let input_entity = Self::_build_input(
            id,
            classes,
            root_node,
            variant,
            color,
            shape,
            ph,
            ph_entity,
            cursor_entity
        );

        utils::entity_add_children(root_node, &vec![ph_entity, cursor_entity], input_entity);
        input_entity
    }

    pub fn handle_text_input_on_hover_system(
        mut events: EventReader<FaInteractionEvent>,
        mut input_q: Query<(&mut BoxShadow, &DefaultWidgetEntity, &IsFamiqTextInput)>
    ) {
        for e in events.read() {
            if e.widget == WidgetType::TextInput {
                if let Ok((mut box_shadow, default_style, _)) = input_q.get_mut(e.entity) {
                    match e.interaction {
                        Interaction::Hovered => {
                            box_shadow.color = default_style.border_color.0.clone();
                            box_shadow.x_offset = Val::Px(0.0);
                            box_shadow.y_offset = Val::Px(0.0);
                            box_shadow.spread_radius = Val::Px(0.5);
                            box_shadow.blur_radius = Val::Px(1.0);
                        },
                        Interaction::None => {
                            *box_shadow = BoxShadow::default();
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn handle_text_input_cursor_on_focused_system(
        mut input_q: Query<(
            &TextInput,
            &Node,
            &BackgroundColor,
            &FamiqTextInputCursorEntity,
            &FamiqTextInputPlaceholderEntity,
            &mut CharacterSize
        )>,
        mut cursor_q: Query<
            (
                &mut Node,
                &mut Visibility,
                &IsFamiqTextInputCursor
            ),
            Without<CharacterSize>
        >,
        mut text_q: Query<(&Text, &mut TextColor, &TextLayoutInfo, &IsFamiqTextInputPlaceholder)>,
    ) {
        for (text_input, text_input_node, bg_color, cursor_entity, placeholder_entity, mut char_size) in input_q.iter_mut() {
            if let Ok((mut cursor_node, mut visibility, _)) = cursor_q.get_mut(cursor_entity.0) {

                if let Ok((text, mut text_color, text_info, _)) = text_q.get_mut(placeholder_entity.0) {
                    if text_input.focused {
                        if bg_color.0 == WHITE_COLOR {
                            text_color.0 = BLACK_COLOR;
                        }
                        else {
                            text_color.0 = TEXT_INPUT_VALUE_COLOR;
                        }

                        *visibility = Visibility::Visible;

                        // update char_size resource every time as users might change
                        // the TextInputSize
                        char_size.width = text_info.size.x / text.0.len() as f32;
                        char_size.height = text_info.size.y;

                        // initial value of cursor node left position is 0.0.
                        // if it's 0.0, update position to be at input padding
                        if utils::extract_val(cursor_node.left).unwrap() == 0.0 {
                            cursor_node.left = text_input_node.padding.left.clone();
                            cursor_node.top = text_input_node.padding.top.clone();
                            cursor_node.width = Val::Px(CURSOR_WIDTH);
                            cursor_node.height = Val::Px(text_info.size.y);
                        }
                    }
                    else {
                        *visibility = Visibility::Hidden;
                        text_color.0 = PLACEHOLDER_COLOR;
                    }
                }
            }
        }
    }

    pub fn handle_text_input_on_click_system(
        mut events: EventReader<FaInteractionEvent>,
        mut input_q: Query<(&mut TextInput, &IsFamiqTextInput)>
    ) {
        for e in events.read() {
            if e.interaction == Interaction::Pressed && e.widget == WidgetType::TextInput {
                // set all to unfocused
                for (mut text_input, _) in input_q.iter_mut() {
                    text_input.focused = false;
                }
                if let Ok((mut text_input, _)) = input_q.get_mut(e.entity) {
                    text_input.focused = true;
                }
                break;
            }
        }
    }

    pub fn handle_text_input_on_typing_system(
        mut evr_kbd: EventReader<KeyboardInput>,
        mut input_resource: ResMut<FaTextInputResource>,
        mut input_q: Query<(
            &mut TextInput,
            &CharacterSize,
            &FamiqTextInputPlaceholderEntity,
            &FamiqTextInputCursorEntity,
            &FamiqWidgetId
        )>,
        mut text_q: Query<(&mut Text, &IsFamiqTextInputPlaceholder)>,
        mut cursor_q: Query<(&mut Node, &mut Visibility, &IsFamiqTextInputCursor)>,
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released {
                continue;
            }
            match &e.logical_key {
                Key::Character(input) => {
                    for (mut text_input, char_size, placeholder_entity, cursor_entity, id) in input_q.iter_mut() {
                        if text_input.focused {
                            text_input.text.push_str(input);
                            input_resource.update_or_insert(id.0.clone(), text_input.text.clone());

                            if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                text.0 = text_input.text.clone();
                                update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                            }
                            break;
                        }
                    }
                }
                Key::Space => {
                    for (mut text_input, char_size, placeholder_entity, cursor_entity, id) in input_q.iter_mut() {
                        if text_input.focused {
                            text_input.text.push_str(&SmolStr::new(" "));
                            input_resource.update_or_insert(id.0.clone(), text_input.text.clone());

                            if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                text.0 = text_input.text.clone();
                                update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                            }
                            break;
                        }
                    }
                }
                Key::Backspace => {
                    for (mut text_input, char_size, placeholder_entity, cursor_entity, id) in input_q.iter_mut() {
                        if text_input.focused {
                            text_input.text.pop();
                            input_resource.update_or_insert(id.0.clone(), text_input.text.clone());

                            if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                if text.0 != text_input.placeholder {
                                    update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, false);
                                }
                                if text_input.text.is_empty() {
                                    text.0 = text_input.placeholder.clone();
                                }
                                else {
                                    text.0 = text_input.text.clone();
                                }
                            }
                            break;
                        }
                    }
                }
                _ => continue
            }
        }
    }

    pub fn handle_cursor_blink_system(
        time: Res<Time>,
        input_q: Query<(&FamiqTextInputCursorEntity, &TextInput, &BackgroundColor), With<IsFamiqTextInput>>,
        mut cursor_q: Query<&mut BackgroundColor, Without<TextInput>>,
        mut cursor_blink_timer: ResMut<FaTextInputCursorBlinkTimer>,
    ) {
        for (cursor_entity, text_input, input_bg_color) in input_q.iter() {
            if text_input.focused {
                if let Ok(mut bg_color) = cursor_q.get_mut(cursor_entity.0) {
                    cursor_blink_timer.timer.tick(time.delta());

                    if cursor_blink_timer.timer.finished() {
                        if cursor_blink_timer.is_transparent {
                            if input_bg_color.0 == WHITE_COLOR {
                                bg_color.0 = BLACK_COLOR;
                            }
                            else {
                                bg_color.0 = WHITE_COLOR;
                            }
                        }
                        else {
                            *bg_color = BackgroundColor::default();
                        }
                        cursor_blink_timer.is_transparent = !cursor_blink_timer.is_transparent;
                    }
                    break;
                }
            }
        }
    }
}
