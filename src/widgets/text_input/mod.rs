pub mod helper;

use bevy::utils::HashMap;
use helper::*;

use crate::utils;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses, WidgetType};
use crate::event_writer::FaInteractionEvent;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use smol_str::SmolStr;

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

#[derive(Component)]
pub struct IsFamiqTextInput;

#[derive(Component)]
pub struct IsFamiqTextInputPlaceholder;

#[derive(Component)]
pub struct FamiqTextInputPlaceholderEntity(pub Entity);

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

    fn _build_input(
        id: &str,
        classes: &str,
        root_node: &'a mut EntityCommands,
        variant: TextInputVariant,
        color: TextInputColor,
        shape: TextInputShape,
        placeholder: &str,
        placeholder_entity: Entity
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
                FamiqTextInputPlaceholderEntity(placeholder_entity)
            ))
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
        let ph_entity = Self::_build_placeholder(ph, root_node, asset_server, font_path, &size);
        let input_entity = Self::_build_input(id, classes, root_node, variant, color, shape, ph, ph_entity);

        utils::entity_add_child(root_node, ph_entity, input_entity);
        input_entity
    }

    pub fn update_input_text_color_system(
        input_q: Query<(&TextInput, &FamiqTextInputPlaceholderEntity, &IsFamiqTextInput)>,
        mut text_q: Query<(&mut TextColor, &IsFamiqTextInputPlaceholder)>
    ) {
        for (text_input, placeholder_entity, _) in input_q.iter() {
            if let Ok((mut text_color, _)) = text_q.get_mut(placeholder_entity.0) {
                if text_input.focused {
                    text_color.0 = TEXT_INPUT_VALUE_COLOR;
                }
                else {
                    text_color.0 = PLACEHOLDER_COLOR;
                }
            }
        }
    }

    pub fn handle_text_input_on_click_system(
        mut events: EventReader<FaInteractionEvent>,
        mut input_q: Query<(&mut TextInput, &IsFamiqTextInput, &FamiqWidgetId)>
    ) {
        for e in events.read() {
            if e.interaction == Interaction::Pressed && e.widget == WidgetType::TextInput {
                // set all to unfocused
                for (mut text_input, _, _) in input_q.iter_mut() {
                    text_input.focused = false;
                }
                if let Ok((mut text_input, _, _)) = input_q.get_mut(e.entity) {
                    text_input.focused = true;
                }
                break;
            }
        }
    }

    pub fn handle_text_input_on_typing_system(
        mut evr_kbd: EventReader<KeyboardInput>,
        mut input_q: Query<(&mut TextInput, &FamiqTextInputPlaceholderEntity, &IsFamiqTextInput, &FamiqWidgetId)>,
        mut text_q: Query<(&mut Text, &IsFamiqTextInputPlaceholder)>,
        mut input_resource: ResMut<FaTextInputResource>,
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released {
                continue;
            }
            match &e.logical_key {
                Key::Character(input) => {
                    for (mut text_input, placeholder_entity, _, id) in input_q.iter_mut() {
                        if text_input.focused {
                            text_input.text.push_str(input);
                            input_resource.update_or_insert(id.0.clone(), text_input.text.clone());

                            if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                text.0 = text_input.text.clone();
                            }
                            break;
                        }
                    }
                }
                Key::Space => {
                    for (mut text_input, placeholder_entity, _, id) in input_q.iter_mut() {
                        if text_input.focused {
                            text_input.text.push_str(&SmolStr::new(" "));
                            input_resource.update_or_insert(id.0.clone(), text_input.text.clone());

                            if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                text.0 = text_input.text.clone();
                            }
                            break;
                        }
                    }
                }
                Key::Backspace => {
                    for (mut text_input, placeholder_entity, _, id) in input_q.iter_mut() {
                        if text_input.focused {
                            text_input.text.pop();
                            input_resource.update_or_insert(id.0.clone(), text_input.text.clone());

                            if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
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
                _ => {
                    continue;
                }
            }
        }
    }
}
