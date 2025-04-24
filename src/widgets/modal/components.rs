use bevy::prelude::*;

/// Marker component for identifying the modal background.
#[derive(Component)]
pub struct IsFamiqModalBackground;

/// Marker component for identifying the modal container that hold all the items provided.
#[derive(Component)]
pub struct IsFamiqModalContainer;

/// Component associating a modal background with its container entity.
#[derive(Component)]
pub struct FaModalContainerEntity(pub Entity);

#[derive(Component)]
pub struct IsModalChild;

/// Component that keep tracking of modal show/hide animation.
#[derive(Component)]
pub struct AnimationProgress(pub f32);
