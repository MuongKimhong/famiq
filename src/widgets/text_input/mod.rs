pub mod styling;
pub mod tests;

use styling::*;
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::color::WHITE_COLOR;
use crate::widgets::*;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::ecs::system::EntityCommands;
use bevy::text::TextLayoutInfo;
use bevy::input::ButtonState;
use bevy::prelude::*;
use smol_str::SmolStr;

use super::color::{BLACK_COLOR, SECONDARY_COLOR};

#[derive(Component, Default)]
pub struct TextInputValue(pub String);


#[derive(Default)]
pub enum CursorMove {
    #[default]
    Right,
    Left
}

pub enum EditingType {
    Adding,
    Removing,
    Nothing
}

/// Represents the text input field containing the user-entered text and placeholder.
#[derive(Component)]
pub struct TextInput {
    pub placeholder: String,
    pub cursor_index: usize,
    pub input_type: TextInputType
}

impl TextInput {
    pub fn new(placeholder: &str, input_type: TextInputType) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            cursor_index: 0,
            input_type
        }
    }
}

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

/// Marker component for identifying a text input widget.
#[derive(Component)]
pub struct IsFamiqTextInput;

/// Marker component for identifying a placeholder text in a text input widget.
#[derive(Component)]
pub struct IsFamiqTextInputPlaceholder;

/// Marker component for identifying the cursor in a text input widget.
#[derive(Component)]
pub struct IsFamiqTextInputCursor;

/// Links a placeholder entity to its corresponding text input entity.
#[derive(Component)]
pub struct FamiqTextInputPlaceholderEntity(pub Entity);

/// Links a cursor entity to its corresponding text input entity.
#[derive(Component)]
pub struct FamiqTextInputCursorEntity(pub Entity);

/// Link a toggle icon entity to its corresponding text input entity;
#[derive(Component)]
pub struct FamiqTextInputToggleIconEntity(pub Entity);

#[derive(Component)]
pub struct FamiqTextInputEntity(pub Entity);

/// Represents the size of a single character in the text input field.
#[derive(Component)]
pub struct CharacterSize {
    pub width: f32,
    pub height: f32
}

/// The width of the text input cursor.
pub const CURSOR_WIDTH: f32 = 2.0;

/// Type options for text input widget.
#[derive(PartialEq, Clone)]
pub enum TextInputType {
    Text,
    Password
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
                Node {
                    left: Val::Px(0.0),
                    ..default()
                }
            ))
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
                    left: Val::Px(0.0),
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

    fn _build_input(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
        placeholder_entity: Entity,
        cursor_entity: Entity,
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
                DefaultWidgetEntity::from(style_components),
                TextInput::new(placeholder, input_type),
                TextInputValue::default(),
                FamiqTextInputPlaceholderEntity(placeholder_entity),
                FamiqTextInputCursorEntity(cursor_entity),
                CharacterSize { width: 0.0, height: 0.0 },
            ))
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
        let ph_entity = Self::_build_placeholder(attributes, placeholder, root_node);
        let input_entity = Self::_build_input(
            attributes,
            root_node,
            placeholder,
            ph_entity,
            cursor_entity,
            input_type
        );
        entity_add_children(root_node, &vec![ph_entity, cursor_entity], input_entity);
        input_entity
    }

    pub fn handle_text_input_on_focused_system(
        mut input_q: Query<(
            Entity,
            &Node,
            &TextInput,
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
        mut placeholder_q: Query<(&Text, &TextLayoutInfo), With<IsFamiqTextInputPlaceholder>>,
        builder_res: Res<FamiqResource>
    ) {
        if !builder_res.is_changed() || builder_res.is_added() {
            return;
        }
        for (
            input_entity,
            text_input_node,
            text_input,
            cursor_entity,
            placeholder_entity,
            mut char_size
        ) in input_q.iter_mut() {

            let Some(focused) = builder_res.get_widget_focus_state(&input_entity) else { continue };

            let Ok((mut cursor_node, mut visibility, _)) = cursor_q.get_mut(cursor_entity.0) else {continue};

            if let Ok((placeholder_text, placeholder_text_info)) = placeholder_q.get_mut(placeholder_entity.0) {
                if focused {
                    *visibility = Visibility::Visible;
                    _handle_cursor_on_focused(
                        &mut cursor_node,
                        text_input_node,
                        &placeholder_text_info,
                        &placeholder_text.0,
                        &mut char_size,
                        &text_input
                    );
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    // hovered, pressed, none
    pub fn handle_text_input_interaction_system(
        mut input_q: Query<
            (Entity, &mut BoxShadow, &mut TextInput, &TextInputValue, &Interaction, &DefaultWidgetEntity),
            Changed<Interaction>
        >,
        mut builder_res: ResMut<FamiqResource>,

        window: Single<Entity, With<Window>>,
        mut commands: Commands,
        cursor_icons: Res<CursorIcons>,
    ) {
        for (entity, mut box_shadow, mut text_input, value, interaction, default_style) in input_q.iter_mut() {
            match interaction {
                Interaction::Hovered => {
                    box_shadow.color = default_style.border_color.0.clone();
                    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
                },
                Interaction::Pressed => {
                    // global focus
                    builder_res.update_all_focus_states(false);
                    builder_res.update_or_insert_focus_state(entity, true);

                    if text_input.cursor_index > 0 {
                        text_input.cursor_index = value.0.len();
                    }
                    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
                },
                _ => {
                    box_shadow.color = Color::NONE;
                    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
                }
            }
        }
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

    pub(crate) fn _handle_update_placeholder(
        placeholder_text: &mut Text,
        text_input: &TextInput,
        value: &TextInputValue
    ) {

        if value.0.is_empty() {
            placeholder_text.0 = text_input.placeholder.clone();
            return;
        }

        if text_input.input_type == TextInputType::Password {
            placeholder_text.0 = mask_string(&value.0);
        }
        else {
            placeholder_text.0 = value.0.clone();
        }
    }

    fn _is_text_overflow(
        char_width: f32,
        input_computed: &ComputedNode,
        placeholder_computed: &ComputedNode
    ) -> bool {
        let input_size = input_computed.size();
        let input_padding = input_computed.padding();
        let input_scale = input_computed.inverse_scale_factor();

        let input_width = (input_size.x - input_padding.left - input_padding.right) * input_scale;
        let placeholder_width = placeholder_computed.size().x * placeholder_computed.inverse_scale_factor();

        return placeholder_width >= input_width - char_width;
    }

    pub(crate) fn handle_text_input_on_typing_system(
        mut evr_kbd: EventReader<KeyboardInput>,
        mut input_res: ResMut<FaTextInputResource>,
        mut input_q: Query<(
            Entity,
            &mut TextInput,
            &mut TextInputValue,
            &ComputedNode,
            &CharacterSize,
            &FamiqTextInputPlaceholderEntity,
            &FamiqTextInputCursorEntity,
            Option<&FamiqWidgetId>
        )>,
        mut placeholder_q: Query<
            (&mut Text, &mut Node, &ComputedNode),
            (With<IsFamiqTextInputPlaceholder>, Without<IsFamiqTextInputCursor>)
        >,
        mut cursor_q: Query<
            &mut Node,
            (With<IsFamiqTextInputCursor>, Without<IsFamiqTextInputPlaceholder>)
        >,
        builder_res: Res<FamiqResource>
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released {
                continue;
            }

            for (input_entity, mut input, mut input_value, input_computed, char_size, ph_entity, cursor_entity, input_id) in input_q.iter_mut() {

                let Some(focused) = builder_res.get_widget_focus_state(&input_entity) else { continue };

                if focused {
                    if let Ok((mut ph_text, mut ph_node, ph_computed)) = placeholder_q.get_mut(ph_entity.0) {

                        match &e.logical_key {
                            Key::Character(key_input) => {
                                _update_text_input_value(input_id, &mut input_res, &mut input, &mut input_value, true, Some(key_input));
                                FaTextInput::_handle_update_placeholder(&mut ph_text, &input, &input_value);

                                if input.cursor_index < input_value.0.len() {
                                    input.cursor_index += 1;
                                }
                                if FaTextInput::_is_text_overflow(char_size.width, input_computed, ph_computed) {
                                    ph_node.left = Val::Px(extract_val(ph_node.left).unwrap() - char_size.width);
                                }
                                else {
                                    _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                                }
                            }
                            Key::Space => {
                                _update_text_input_value(input_id, &mut input_res, &mut input, &mut input_value, true, Some(&SmolStr::new(" ")));
                                FaTextInput::_handle_update_placeholder(&mut ph_text, &input, &input_value);

                                if input.cursor_index < input_value.0.len() {
                                    input.cursor_index += 1;
                                }
                                if FaTextInput::_is_text_overflow(char_size.width, input_computed, ph_computed) {
                                    ph_node.left = Val::Px(extract_val(ph_node.left).unwrap() - char_size.width);
                                }
                                else {
                                    _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                                }
                            }
                            Key::Backspace => {
                                _update_text_input_value(input_id, &mut input_res, &mut input, &mut input_value, false, None);
                                FaTextInput::_handle_update_placeholder(&mut ph_text, &input, &input_value);

                                if extract_val(ph_node.left).unwrap() <= (-char_size.width) / 2.0 {
                                    ph_node.left = Val::Px(extract_val(ph_node.left).unwrap() + char_size.width);
                                }
                                else {
                                    if input.cursor_index > 0 {
                                        _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, false);
                                    }
                                }

                                if input.cursor_index > 0 {
                                    input.cursor_index -= 1;
                                }
                            }
                            // Key::ArrowLeft => {}
                            // Key::ArrowRight => {}
                            _ => continue
                        }

                    }
                }
            }
        }
    }

    pub fn handle_cursor_blink_system(
        time: Res<Time>,
        input_q: Query<(Entity, &FamiqTextInputCursorEntity, &BackgroundColor)>,
        mut cursor_q: Query<(&mut BackgroundColor, &IsFamiqTextInputCursor), Without<FamiqTextInputCursorEntity>>,
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
