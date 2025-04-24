use bevy::prelude::*;

/// Marker component for identifying the label part of the FPS text (e.g., "FPS:").
#[derive(Component)]
pub struct IsFPSTextLabel;

/// Marker component for identifying the FPS count text (e.g., "60.0").
#[derive(Component)]
pub struct IsFPSTextCount;

/// Component to indicate whether the FPS text color can change dynamically.
/// - `true`: The FPS text will change color based on the FPS value.
/// - `false`: The FPS text color remains constant.
#[derive(Component)]
pub struct CanChangeColor(pub bool);
