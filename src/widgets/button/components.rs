use bevy::prelude::*;

/// Marker component for identifying an entity as a Famiq button.
#[derive(Component)]
pub struct IsFamiqButton;

/// Marker component for identifying an entity as a Famiq button's text.
#[derive(Component)]
pub struct IsFamiqButtonText;

#[derive(Component)]
pub struct IsFamiqButtonTextContainer;

#[derive(Component)]
pub struct ButtonTextContainerEntity(pub Entity);

/// Component storing the entity associated with a button's text.
///
/// Used to identify which text belongs to which button.
#[derive(Component)]
pub struct ButtonTextEntity(pub Entity);
