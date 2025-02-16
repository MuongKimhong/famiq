use bevy::prelude::*;

/// Marker component for identifying a Famiq circular widget.
#[derive(Component)]
pub struct IsFamiqCircular;

#[derive(Component)]
pub struct SpinnerColor(pub Color);
