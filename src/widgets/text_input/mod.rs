pub mod helper;
pub mod tests;

use bevy::ui::FocusPolicy;
use helper::*;
use crate::utils::{self, process_spacing_built_in_class, mask_string};
use crate::widgets::color::WHITE_COLOR;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId,
    FamiqWidgetClasses, WidgetType, FamiqWidgetResource, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged, ResourceMap
};
use crate::event_writer::FaInteractionEvent;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::ecs::system::EntityCommands;
use bevy::text::TextLayoutInfo;
use bevy::input::ButtonState;
use bevy::utils::HashMap;
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

/// Stores the text input values in a `HashMap` where keys are IDs of the inputs.
#[derive(Resource, Default, Debug)]
pub struct FaTextInputResource {
    pub inputs: HashMap<String, String>,
}

/// Implement the trait for FaTextInputResource
impl ResourceMap for FaTextInputResource {
    fn _update_or_insert(&mut self, id: String, value: String) {
        self.inputs.insert(id, value);
    }

    fn get_value(&self, id: &str) -> String {
        self.inputs.get(id).cloned().unwrap_or_default()
    }

    fn exists(&self, id: &str) -> bool {
        self.inputs.contains_key(id)
    }
}

/// Handles the blinking behavior of the text input cursor.
#[derive(Resource, Debug)]
pub struct FaTextInputCursorBlinkTimer {
    pub timer: Timer, // change bg color every 0.5 second
    pub is_transparent: bool
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

/// Color options for the text input widget.
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

/// Appearance variants for the text input widget.
pub enum TextInputVariant {
    Default,
    Outlined,
    Underlined,
}

/// Size options for the text input widget.
pub enum TextInputSize {
    Small,
    Normal,
    Large,
}

/// Shape options for the text input widget.
pub enum TextInputShape {
    Default,
    Round,
    Rectangle
}

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
        id: &Option<String>,
        class: &Option<String>,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &TextInputSize,
    ) -> Entity {
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR);
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

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(id.to_owned()));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class.to_owned()));
        }
        entity
    }

    fn _build_toggle_password_icon(
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &TextInputSize,
        input_entity: Entity
    ) -> Entity {
        let txt = Text::new("<?>");
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(BLACK_COLOR);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Right);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                TogglePasswordIcon::default(),
                FamiqTextInputEntity(input_entity),
                FocusPolicy::Block,
                Interaction::default()
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
        id: &Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        variant: TextInputVariant,
        color: TextInputColor,
        shape: TextInputShape,
        placeholder: &str,
        placeholder_entity: Entity,
        cursor_entity: Entity
    ) -> Entity {
        let mut node = default_input_node();
        process_spacing_built_in_class(&mut node, &class);

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

        let entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                bg_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                BoxShadow {
                    color: Color::NONE,
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(0.5),
                    blur_radius: Val::Px(1.0)
                },
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
            .insert((
                CharacterSize { width: 0.0, height: 0.0 },
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(id.to_string()));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        placeholder: &str, // placeholder
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: TextInputSize,
        variant: TextInputVariant,
        color: TextInputColor,
        shape: TextInputShape,
        input_type: &TextInputType
    ) -> Entity {
        let cursor_entity = Self::_build_cursor(root_node, &color);
        let ph_entity = Self::_build_placeholder(
            &id,
            &class,
            placeholder,
            root_node,
            font_handle.clone(),
            &size
        );
        let input_entity = Self::_build_input(
            &id,
            class,
            root_node,
            variant,
            color,
            shape,
            placeholder,
            ph_entity,
            cursor_entity
        );

        let mut children = vec![ph_entity, cursor_entity];

        if *input_type == TextInputType::Password {
            let toggle_icon = Self::_build_toggle_password_icon(root_node, font_handle, &size, input_entity);
            root_node.commands().entity(input_entity).insert(FamiqTextInputToggleIconEntity(toggle_icon));
            children.push(toggle_icon);
        }

        utils::entity_add_children(root_node, &children, input_entity);
        input_entity
    }

    pub fn handle_text_input_on_focused_system(
        mut input_q: Query<(
            Entity,
            &Node,
            &BackgroundColor,
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
        mut placeholder_q: Query<
            (&Text, &mut TextColor, &TextLayoutInfo, &WidgetStyle),
            With<IsFamiqTextInputPlaceholder>
        >,
        builder_res: Res<FamiqWidgetResource>
    ) {
        if !builder_res.is_changed() {
            return;
        }
        for (
            input_entity,
            text_input_node,
            input_bg_color,
            text_input,
            cursor_entity,
            placeholder_entity,
            mut char_size
        ) in input_q.iter_mut() {

            let Some(focused) = builder_res.get_widget_focus_state(&input_entity) else { continue };

            let Ok((mut cursor_node, mut visibility, _)) = cursor_q.get_mut(cursor_entity.0) else {continue};

            if let Ok((
                placeholder_text,
                mut placeholder_text_color,
                placeholder_text_info,
                placeholder_internal_widget_style
            )) = placeholder_q.get_mut(placeholder_entity.0) {
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

                _handle_update_placeholder_color(
                    &mut placeholder_text_color,
                    input_bg_color,
                    placeholder_internal_widget_style,
                    focused
                );
            }
        }
    }

    // hovered, pressed, none
    pub fn handle_text_input_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut input_q_for_hover: Query<
            (&mut BoxShadow, &mut TextInput, Option<&FamiqWidgetId>, &DefaultWidgetEntity)
        >,
        mut builder_res: ResMut<FamiqWidgetResource>,
        mut input_resource: ResMut<FaTextInputResource>,
    ) {
        for e in events.read() {
            if e.widget == WidgetType::TextInput {
                if let Ok((mut box_shadow, mut text_input, id, default_style)) = input_q_for_hover.get_mut(e.entity) {
                    match e.interaction {
                        Interaction::Hovered => {
                            box_shadow.color = default_style.border_color.0.clone();
                        },
                        Interaction::Pressed => {
                            // global focus
                            builder_res.update_all_focus_states(false);
                            builder_res.update_or_insert_focus_state(e.entity, true);

                            if text_input.cursor_index > 0 {
                                text_input.cursor_index = text_input.text.len();
                            }

                            if let Some(id) = id {
                                if !input_resource.exists(id.0.as_str()) {
                                    input_resource._update_or_insert(id.0.clone(), "".to_string());
                                }
                            }
                        },
                        _ => box_shadow.color = Color::NONE
                    }
                }
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
        builder_res: Res<FamiqWidgetResource>
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
                            _update_text_input_value(id, &mut input_res, &mut text_input, true, Some(input));

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
                            _update_text_input_value(id, &mut input_res, &mut text_input, true, Some(&SmolStr::new(" ")));

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
                            _update_text_input_value(id, &mut input_res, &mut text_input, false, None);

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
        builder_res: Res<FamiqWidgetResource>
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
            if !e.is_password_toggle_icon_pressed() {
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
    pub id: Option<String>,
    pub placeholder: String,
    pub class: Option<String>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>,
    pub input_type: TextInputType
}

impl<'a> FaTextInputBuilder<'a> {
    pub fn new(
        placeholder: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>
    ) -> Self {
        Self {
            id: None,
            placeholder,
            class: None,
            font_handle,
            root_node,
            input_type: TextInputType::Text
        }
    }

    fn _process_built_in_classes(&mut self) -> (TextInputVariant, TextInputColor, TextInputSize, TextInputShape) {
        let mut use_color = TextInputColor::Default;
        let mut use_size = TextInputSize::Normal;
        let mut use_shape = TextInputShape::Default;
        let mut use_variant = TextInputVariant::Default;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-underlined" => use_variant = TextInputVariant::Underlined,
                    "is-outlined" => use_variant = TextInputVariant::Outlined,

                    "is-small" => use_size = TextInputSize::Small,
                    "is-large" => use_size = TextInputSize::Large,

                    "is-round" => use_shape = TextInputShape::Round,
                    "is-rectangle" => use_shape = TextInputShape::Rectangle,

                    "is-primary" => use_color = TextInputColor::Primary,
                    "is-secondary" => use_color = TextInputColor::Secondary,
                    "is-danger" => use_color = TextInputColor::Danger,
                    "is-success" => use_color = TextInputColor::Success,
                    "is-warning" => use_color = TextInputColor::Warning,
                    "is-info" => use_color = TextInputColor::Info,

                    "is-password" => self.input_type = TextInputType::Password,
                    _ => ()
                }
            }
        }
        (use_variant, use_color, use_size, use_shape)
    }

    /// Method to add class to text_input
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to text_input
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Method to set text_input type as password
    pub fn is_password(mut self) -> Self {
        self.input_type = TextInputType::Password;
        self
    }

    /// Spawn text input into UI World.
    pub fn build(&mut self) -> Entity {
        let (variant, color, size, shape) = self._process_built_in_classes();
        FaTextInput::new(
            self.id.clone(),
            self.class.clone(),
            self.placeholder.as_str(),
            &mut self.root_node,
            self.font_handle.clone(),
            size,
            variant,
            color,
            shape,
            &self.input_type
        )
    }
}

/// API to create `FaTextInputBuilder`
pub fn fa_text_input<'a>(
    builder: &'a mut FamiqWidgetBuilder,
    placeholder: &str
) -> FaTextInputBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
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
    input_q.iter().count() > 0
}
