use bevy::prelude::*;

/// Marker component for identifying an entity as a Famiq button.
///
/// This component can be used to distinguish buttons in the ECS system.
#[derive(Component)]
pub struct IsFamiqButton;

/// Marker component for identifying an entity as a Famiq button's text.
///
/// This component can be used to distinguish the text associated with a button in the ECS system.
#[derive(Component)]
pub struct IsFamiqButtonText;

/// Component storing the entity associated with a button's text.
///
/// This can be used to establish a relationship between a button and its text entity.
///
/// # Fields
/// - `0`: The entity representing the button's text.
#[derive(Component)]
pub struct ButtonTextEntity(pub Entity);
