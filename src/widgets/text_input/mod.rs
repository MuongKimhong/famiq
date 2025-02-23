pub mod helper;
pub mod tests;

use bevy::ui::FocusPolicy;
use helper::*;
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::color::WHITE_COLOR;
use crate::widgets::*;
use crate::event_writer::FaInteractionEvent;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::ecs::system::EntityCommands;
use bevy::text::TextLayoutInfo;
use bevy::input::ButtonState;
use bevy::prelude::*;
use smol_str::SmolStr;

use super::color::BLACK_COLOR;

/// Represents the text input field containing the user-entered text and placeholder.
#[derive(Component)]
pub struct TextInput {
    pub text: String,
    pub placeholder: String,
    pub cursor_index: usize
}

impl TextInput {
    pub fn new(text: &str, placeholder: &str) -> Self {
        Self {
            text: text.to_string(),
            placeholder: placeholder.to_string(),
            cursor_index: 0
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

/// Link a text input entity to its corresponding toggle icon entity;
#[derive(Component)]
pub struct FamiqTextInputEntity(pub Entity);

/// Represents the size of a single character in the text input field.
#[derive(Component)]
pub struct CharacterSize {
    pub width: f32,
    pub height: f32
}

/// Marker component for identifying a toggle password visibility icon in a text input widget.
#[derive(Component, Default)]
pub struct TogglePasswordIcon {
    pub can_see_text: bool
}

/// The width of the text input cursor.
pub const CURSOR_WIDTH: f32 = 2.0;

/// Type options for text input widget.
#[derive(PartialEq)]
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
        let mut use_color = WHITE_COLOR;

        if attributes.color == WidgetColor::Default {
            use_color = BLACK_COLOR;
        }
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };
        let txt_color = TextColor(use_color);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Left);

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqTextInputPlaceholder,
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    fn _build_toggle_password_icon(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        input_entity: Entity,
    ) -> Entity {
        let mut use_color = WHITE_COLOR;

        if attributes.color == WidgetColor::Default {
            use_color = BLACK_COLOR;
        }

        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };

        root_node
            .commands()
            .spawn((
                Text::new("<?>"),
                txt_font.clone(),
                TextColor(use_color),
                TextLayout::new_with_justify(JustifyText::Right),
                TogglePasswordIcon::default(),
                FamiqTextInputEntity(input_entity),
                FocusPolicy::Block,
                Interaction::default()
            ))
            .id()
    }

    fn _build_cursor(root_node: &'a mut EntityCommands, color: &WidgetColor) -> Entity {
        let mut use_color = WHITE_COLOR;

        if *color == WidgetColor::Default {
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
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
        placeholder_entity: Entity,
        cursor_entity: Entity
    ) -> Entity {
        let input_color = get_input_color(&attributes.color);
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.border_color = BorderColor(input_color);
        style_components.background_color = BackgroundColor(input_color);
        style_components.visibility = Visibility::Visible;
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));

        let entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqTextInput,
                DefaultWidgetEntity::from(style_components),
                TextInput::new("", placeholder),
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
        input_type: &TextInputType
    ) -> Entity {
        let cursor_entity = Self::_build_cursor(root_node, &attributes.color);
        let ph_entity = Self::_build_placeholder(attributes, placeholder, root_node);
        let input_entity = Self::_build_input(
            attributes,
            root_node,
            placeholder,
            ph_entity,
            cursor_entity
        );

        let mut children = vec![ph_entity, cursor_entity];

        if *input_type == TextInputType::Password {
            let toggle_icon = Self::_build_toggle_password_icon(attributes, root_node, input_entity);
            root_node.commands().entity(input_entity).insert(FamiqTextInputToggleIconEntity(toggle_icon));
            children.push(toggle_icon);
        }

        entity_add_children(root_node, &children, input_entity);
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
        if !builder_res.is_changed() {
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
        mut events: EventReader<FaInteractionEvent>,
        mut input_q_for_hover: Query<
            (&mut BoxShadow, &mut TextInput, &DefaultWidgetEntity)
        >,
        mut builder_res: ResMut<FamiqResource>,
        mut cursor_blink_timer: ResMut<FaTextInputCursorBlinkTimer>,

        window: Single<Entity, With<Window>>,
        mut commands: Commands,
        cursor_icons: Res<CursorIcons>,
    ) {
        for e in events.read() {
            if e.widget == WidgetType::TextInput {
                if let Ok((mut box_shadow, mut text_input, default_style)) = input_q_for_hover.get_mut(e.entity) {
                    match e.interaction {
                        Interaction::Hovered => {
                            box_shadow.color = default_style.border_color.0.clone();
                            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
                        },
                        Interaction::Pressed => {
                            // global focus
                            builder_res.update_all_focus_states(false);
                            builder_res.update_or_insert_focus_state(e.entity, true);
                            cursor_blink_timer.is_transparent = false;

                            if text_input.cursor_index > 0 {
                                text_input.cursor_index = text_input.text.len();
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
        }
    }

    /// Internal system to detect new text_input being created.
    pub fn detect_new_text_input_widget_system(
        input_q: Query<(Entity, Option<&FamiqWidgetId>), Added<IsFamiqTextInput>>,
        mut input_res: ResMut<FaTextInputResource>
    ) {
        for (entity, id) in input_q.iter() {
            if let Some(id) = id {
                if !input_res.exists_by_id(id.0.as_str()) {
                    input_res._insert_by_id(id.0.clone(), String::new());
                }
            }

            if !input_res.exists_by_entity(entity) {
                input_res._insert_by_entity(entity, String::new());
            }
        }
    }

    pub fn handle_text_input_on_typing_system(
        mut evr_kbd: EventReader<KeyboardInput>,
        mut input_res: ResMut<FaTextInputResource>,
        mut input_q: Query<(
            Entity,
            &mut TextInput,
            &CharacterSize,
            &FamiqTextInputPlaceholderEntity,
            &FamiqTextInputCursorEntity,
            Option<&FamiqTextInputToggleIconEntity>,
            Option<&FamiqWidgetId>
        )>,
        mut text_q: Query<(&mut Text, &IsFamiqTextInputPlaceholder)>,
        toggle_icon_q: Query<&TogglePasswordIcon>,
        mut cursor_q: Query<(&mut Node, &mut Visibility, &IsFamiqTextInputCursor), Without<CharacterSize>>,
        builder_res: Res<FamiqResource>
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released {
                continue;
            }

            match &e.logical_key {
                Key::Character(input) => {
                    for (
                            entity,
                            mut text_input,
                            char_size,
                            placeholder_entity,
                            cursor_entity,
                            toggle_icon_entity,
                            id,
                        ) in input_q.iter_mut()
                    {
                        if let Some(true) = builder_res.get_widget_focus_state(&entity) {
                            _update_text_input_value(entity, id, &mut input_res, &mut text_input, true, Some(input));

                            if let Ok((mut placeholder_text, _)) = text_q.get_mut(placeholder_entity.0) {
                                placeholder_text.0 = text_input.text.clone();
                                _handle_mask_placeholder(toggle_icon_entity, &toggle_icon_q, &text_input, &mut placeholder_text);
                                _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                            }

                            break;
                        }
                    }
                }
                Key::Space => {
                    for (
                            entity,
                            mut text_input,
                            char_size,
                            placeholder_entity,
                            cursor_entity,
                            toggle_icon_entity,
                            id,
                        ) in input_q.iter_mut()
                    {
                        if let Some(true) = builder_res.get_widget_focus_state(&entity) {
                            _update_text_input_value(entity, id, &mut input_res, &mut text_input, true, Some(&SmolStr::new(" ")));

                            if let Ok((mut placeholder_text, _)) = text_q.get_mut(placeholder_entity.0) {
                                placeholder_text.0 = text_input.text.clone();
                                _handle_mask_placeholder(toggle_icon_entity, &toggle_icon_q, &text_input, &mut placeholder_text);
                                _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                            }

                            break;
                        }
                    }
                }
                Key::Backspace => {
                    for (
                            entity,
                            mut text_input,
                            char_size,
                            placeholder_entity,
                            cursor_entity,
                            toggle_icon_entity,
                            id,
                        ) in input_q.iter_mut()
                    {
                        if let Some(true) = builder_res.get_widget_focus_state(&entity) {
                            _update_text_input_value(entity, id, &mut input_res, &mut text_input, false, None);

                            if let Ok((mut placeholder_text, _)) = text_q.get_mut(placeholder_entity.0) {
                                if placeholder_text.0 != text_input.placeholder {
                                    _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, false);
                                }

                                if text_input.text.is_empty() {
                                    placeholder_text.0 = text_input.placeholder.clone();
                                }
                                else {
                                    placeholder_text.0 = text_input.text.clone();
                                    _handle_mask_placeholder(toggle_icon_entity, &toggle_icon_q, &text_input, &mut placeholder_text);
                                }
                            }

                            break;
                        }
                    }
                },
                Key::ArrowLeft => {
                    for (entity, mut text_input, char_size, _, cursor_entity, _, _,) in input_q.iter_mut() {
                        if let Some(true) = builder_res.get_widget_focus_state(&entity) {
                            if text_input.cursor_index > 0 {
                                text_input.cursor_index -= 1;
                                _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, false);
                            }
                        }
                    }
                }
                Key::ArrowRight => {
                    for (entity, mut text_input, char_size, _, cursor_entity, _, _,) in input_q.iter_mut() {
                        if let Some(true) = builder_res.get_widget_focus_state(&entity) {
                            if text_input.cursor_index < text_input.text.len() {
                                text_input.cursor_index += 1;
                                _update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                            }
                        }
                    }
                }
                _ => continue
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
                },
                _ => {}
            }
        }
    }

    pub fn handle_toggle_password_icon_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut icon_q: Query<(&mut TogglePasswordIcon, &FamiqTextInputEntity)>,
        input_q: Query<(&FamiqTextInputPlaceholderEntity, &TextInput)>,
        mut placeholder_q: Query<&mut Text, With<IsFamiqTextInputPlaceholder>>
    ) {
        for e in events.read() {
            if !e.is_pressed(WidgetType::TextInputTogglePasswordIcon) {
                continue;
            }

            let (mut toggle_icon, input_entity) = match icon_q.get_mut(e.entity) {
                Ok(data) => data,
                Err(_) => continue,
            };

            toggle_icon.can_see_text = !toggle_icon.can_see_text;

            let (placeholder_entity, text_input) = match input_q.get(input_entity.0) {
                Ok(data) => data,
                Err(_) => continue,
            };

            let mut placeholder_text = match placeholder_q.get_mut(placeholder_entity.0) {
                Ok(data) => data,
                Err(_) => continue,
            };

            if text_input.text.trim().is_empty() {
                continue;
            }

            if toggle_icon.can_see_text {
                placeholder_text.0 = text_input.text.clone();
            } else {
                placeholder_text.0 = mask_string(text_input.text.as_str());
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
            &self.input_type
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
