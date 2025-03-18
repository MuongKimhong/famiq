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

/// Marker component for identifying the highlighter in a text input widget.
#[derive(Component)]
pub struct IsFamiqTextInputHighlighter;

/// Links a placeholder entity to its corresponding text input entity.
#[derive(Component)]
pub struct FaTextInputPlaceholderEntity(pub Entity);

/// Links a cursor entity to its corresponding text input entity.
#[derive(Component)]
pub struct FaTextInputCursorEntity(pub Entity);

#[derive(Component)]
pub struct FaTextInputHighlighterEntity(pub Entity);

/// Link a toggle icon entity to its corresponding text input entity;
#[derive(Component)]
pub struct FaTextInputToggleIconEntity(pub Entity);

#[derive(Component)]
pub struct FaTextInputEntity(pub Entity);

/// Type options for text input widget.
#[derive(PartialEq, Clone)]
pub enum TextInputType {
    Text,
    Password
}

/// Represents the text input field containing the user-entered text and placeholder.
#[derive(Component)]
pub struct FaTextInputInfo {
    pub placeholder: String,
    pub input_type: TextInputType
}

impl FaTextInputInfo {
    pub fn new(placeholder: &str, input_type: TextInputType) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            input_type
        }
    }
}
