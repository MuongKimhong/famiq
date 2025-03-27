use bevy::prelude::*;

/// Marker component for identifying a text input widget.
#[derive(Component)]
pub struct IsFamiqTextInput;

/// Marker component for identifying a placeholder text in a text input widget.
#[derive(Component)]
pub struct IsFamiqTextInputPlaceholder;

/// Marker component for identifying buffer texture.
#[derive(Component)]
pub struct IsFamiqTextInputBufferTexture;

/// Links a placeholder entity to its corresponding text input entity.
#[derive(Component)]
pub struct FaTextInputPlaceholderEntity(pub Entity);

/// Links a cursor entity to its corresponding text input entity.
#[derive(Component)]
pub struct FaTextInputBufferTextureEntity(pub Entity);

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

/// Handles the blinking behavior of the text input cursor.
#[derive(Component, Debug)]
pub struct CursorBlinkTimer {
    pub timer: Timer,
    pub can_blink: bool,
    pub is_transparent: bool
}

impl Default for CursorBlinkTimer {
    fn default() -> Self {
        CursorBlinkTimer {
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
            can_blink: true,
            is_transparent: false
        }
    }
}
