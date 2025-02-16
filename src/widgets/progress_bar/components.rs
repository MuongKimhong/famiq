use bevy::prelude::*;

/// Marker component for identifying an entity as a Famiq Progress bar.
#[derive(Component)]
pub struct IsFamiqProgressBar;

/// Marker component for identifying an entity as a Famiq Progress bar's value.
#[derive(Component)]
pub struct IsFamiqProgressValue;

/// Component storing the progress bar entity associated with its value.
#[derive(Component)]
pub struct FamiqProgressBarEntity(pub Entity);

/// Component storing the progress value entity associated with its bar.
#[derive(Component)]
pub struct FamiqProgressValueEntity(pub Entity);

/// Component storing percentage of a progress bar.
#[derive(Component)]
pub struct FaProgressValuePercentage(pub f32);

#[derive(Component)]
pub struct ProgressValueColor(pub Color);
