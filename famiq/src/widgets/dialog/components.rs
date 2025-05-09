use bevy::prelude::*;

/// Marker component for identifying the modal background.
#[derive(Component)]
pub struct IsFamiqDialog;

/// Component that keep tracking of modal show/hide animation.
#[derive(Component)]
pub struct AnimationProgress(pub f32);
