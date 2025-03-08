use bevy::prelude::*;

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

#[derive(Component, Default)]
pub struct TextInputValue(pub String);


/// Type options for text input widget.
#[derive(PartialEq, Clone)]
pub enum TextInputType {
    Text,
    Password
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
