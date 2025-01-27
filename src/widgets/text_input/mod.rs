pub mod helper;

use helper::*;
use crate::utils;
use crate::widgets::color::WHITE_COLOR;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId,
    FamiqWidgetClasses, WidgetType, FamiqWidgetResource, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged
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
    pub placeholder: String
}

impl TextInput {
    pub fn new(text: &str, placeholder: &str) -> Self {
        Self {
            text: text.to_string(),
            placeholder: placeholder.to_string()
        }
    }
}

/// Stores the text input values in a `HashMap` where keys are IDs of the inputs.
#[derive(Resource, Debug)]
pub struct FaTextInputResource {
    pub inputs: HashMap<String, String>,
}

impl FaTextInputResource {
    /// Updates an existing input value or inserts a new one if it doesn't exist.
    ///
    /// # Parameters
    /// - `id`: The ID of the input field.
    /// - `new_value`: The new text value for the input.
    pub fn update_or_insert(&mut self, id: String, new_value: String) {
        if let Some(old_value) = self.inputs.get_mut(&id) {
            *old_value = new_value;
        } else {
            self.inputs.insert(id, "".to_string());
        }
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

/// Represents the size of a single character in the text input field.
#[derive(Component)]
pub struct CharacterSize {
    pub width: f32,
    pub height: f32
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

/// Represents the Famiq text input widget, which includes placeholder text, a blinking cursor, and customizable styles.
pub struct FaTextInput;

// Needs container
impl<'a> FaTextInput {
    fn _build_placeholder(
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
        shape: TextInputShape
    ) -> Entity {
        let cursor_entity = Self::_build_cursor(root_node, &color);
        let ph_entity = Self::_build_placeholder(placeholder, root_node, font_handle, &size);
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

        utils::entity_add_children(root_node, &vec![ph_entity, cursor_entity], input_entity);
        input_entity
    }

    pub fn handle_text_input_cursor_on_focused_system(
        mut input_q: Query<(
            Entity,
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
        mut text_q: Query<(&Text, &mut TextColor, &TextLayoutInfo), With<IsFamiqTextInputPlaceholder>>,
        builder_res: Res<FamiqWidgetResource>
    ) {
        for (input_entity, text_input_node, bg_color, cursor_entity, placeholder_entity, mut char_size) in input_q.iter_mut() {
            if let Ok((mut cursor_node, mut visibility, _)) = cursor_q.get_mut(cursor_entity.0) {

                if let Ok((text, mut text_color, text_info)) = text_q.get_mut(placeholder_entity.0) {
                    match builder_res.get_widget_focus_state(&input_entity) {
                        Some(true) => {
                            handle_on_focused(
                                &mut text_color,
                                bg_color,
                                &mut visibility,
                                &mut cursor_node,
                                text_input_node,
                                &text_info,
                                &text.0,
                                &mut char_size,
                            );
                        }
                        _ => {
                            // Handle unfocused state
                            *visibility = Visibility::Hidden;
                            text_color.0 = PLACEHOLDER_COLOR;
                        }
                    }
                }
            }
        }
    }

    // hovered, pressed, none
    pub fn handle_text_input_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut input_q_for_hover: Query<
            (&mut BoxShadow, Option<&FamiqWidgetId>, &DefaultWidgetEntity),
            With<TextInput>
        >,
        mut builder_res: ResMut<FamiqWidgetResource>,
        mut input_resource: ResMut<FaTextInputResource>,
    ) {
        for e in events.read() {
            if e.widget == WidgetType::TextInput {
                if let Ok((mut box_shadow, id, default_style)) = input_q_for_hover.get_mut(e.entity) {
                    match e.interaction {
                        Interaction::Hovered => {
                            box_shadow.color = default_style.border_color.0.clone();
                        },
                        Interaction::Pressed => {
                            // global focus
                            builder_res.update_all_focus_states(false);
                            builder_res.update_or_insert_focus_state(e.entity, true);

                            if let Some(id) = id {
                                input_resource.update_or_insert(id.0.clone(), "".to_string());
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
        mut input_resource: ResMut<FaTextInputResource>,
        mut input_q: Query<(
            Entity,
            &mut TextInput,
            &CharacterSize,
            &FamiqTextInputPlaceholderEntity,
            &FamiqTextInputCursorEntity,
            Option<&FamiqWidgetId>
        )>,
        mut text_q: Query<(&mut Text, &IsFamiqTextInputPlaceholder)>,
        mut cursor_q: Query<(&mut Node, &mut Visibility, &IsFamiqTextInputCursor)>,
        builder_res: Res<FamiqWidgetResource>
    ) {
        for e in evr_kbd.read() {
            if e.state == ButtonState::Released {
                continue;
            }

            match &e.logical_key {
                Key::Character(input) => {
                    for (entity, mut text_input, char_size, placeholder_entity, cursor_entity, id) in input_q.iter_mut() {
                        match builder_res.get_widget_focus_state(&entity) {
                            Some(true) => {
                                text_input.text.push_str(input);

                                if let Some(id) = id {
                                    input_resource.update_or_insert(id.0.clone(), text_input.text.clone());
                                }

                                if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                    text.0 = text_input.text.clone();
                                    update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                                }
                                break;
                            },
                            _ => {}
                        }
                    }
                }
                Key::Space => {
                    for (entity, mut text_input, char_size, placeholder_entity, cursor_entity, id) in input_q.iter_mut() {
                        match builder_res.get_widget_focus_state(&entity) {
                            Some(true) => {
                                text_input.text.push_str(&SmolStr::new(" "));

                                if let Some(id) = id {
                                    input_resource.update_or_insert(id.0.clone(), text_input.text.clone());
                                }

                                if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                    text.0 = text_input.text.clone();
                                    update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, true);
                                }
                                break;
                            },
                            _ => {}
                        }
                    }
                }
                Key::Backspace => {
                    for (entity, mut text_input, char_size, placeholder_entity, cursor_entity, id) in input_q.iter_mut() {
                        match builder_res.get_widget_focus_state(&entity) {
                            Some(true) => {
                                text_input.text.pop();

                                if let Some(id) = id {
                                    input_resource.update_or_insert(id.0.clone(), text_input.text.clone());
                                }

                                if let Ok((mut text, _)) = text_q.get_mut(placeholder_entity.0) {
                                    if text.0 != text_input.placeholder {
                                        update_cursor_position(&mut cursor_q, cursor_entity.0, char_size.width, false);
                                    }
                                    if text_input.text.is_empty() {
                                        text.0 = text_input.placeholder.clone();
                                    } else {
                                        text.0 = text_input.text.clone();
                                    }
                                }
                                break;
                            },
                            _ => {}
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
}

/// Builder for creating and customizing `FaTextInput` widgets.
pub struct FaTextInputBuilder<'a> {
    pub id: Option<String>,
    pub placeholder: String,
    pub class: Option<String>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
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
            root_node
        }
    }

    fn _process_built_in_classes(&self) -> (TextInputVariant, TextInputColor, TextInputSize, TextInputShape) {
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
            shape
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

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::widgets::color::PRIMARY_DARK_COLOR;
    use bevy::input::InputPlugin;
    use super::*;

    fn setup_test_default_input(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_text_input(&mut builder, "First name").id("#test-input").build();
    }

    fn setup_test_input_with_built_in_class(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_text_input(&mut builder, "First name")
            .class("is-primary is-rectangle")
            .build();
    }

    #[test]
    fn test_create_default_input() {
        let mut app = utils::create_test_app();
        app.add_plugins(InputPlugin::default());
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_input);
        app.update();

        let input_q = app.world_mut()
            .query::<(&FamiqWidgetId, &IsFamiqTextInput)>()
            .get_single(app.world());

        let input_id = input_q.unwrap().0;
        assert_eq!("#test-input".to_string(), input_id.0);
    }

    #[test]
    fn test_create_input_with_built_in_class() {
        let mut app = utils::create_test_app();
        app.add_plugins(InputPlugin::default());
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_input_with_built_in_class);
        app.update();

        let input_q = app.world_mut()
            .query::<(&FamiqWidgetClasses, &BackgroundColor, &BorderRadius, &IsFamiqTextInput)>()
            .get_single(app.world());

        let input_class = input_q.as_ref().unwrap().0;
        assert_eq!("is-primary is-rectangle".to_string(), input_class.0);

        let input_bg = input_q.as_ref().unwrap().1;
        assert_eq!(BackgroundColor(PRIMARY_DARK_COLOR), *input_bg);

        let input_border_radius = input_q.unwrap().2;
        assert_eq!(
            BorderRadius::all(Val::Px(0.0)),
            *input_border_radius
        );
    }
}
