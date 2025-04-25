use bevy::prelude::*;
use super::*;

/// Marker component for identifying a Famiq circular widget.
/// field 0 is for triggering Changed filter
#[derive(Component)]
pub struct IsFamiqCircular(pub usize);

#[derive(Component)]
pub struct CircularMaterialHandle(pub Option<Handle<CircularMaterial>>);

#[derive(Component)]
pub struct SpinnerColor(pub Color);
