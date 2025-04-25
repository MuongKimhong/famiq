use bevy::prelude::*;

/// Marker component for identifying an entity as a Famiq Progress bar.
/// field 0 is for triggering Changed filter.
#[derive(Component)]
pub struct IsFamiqProgressBar(pub usize);

/// Marker component for identifying an entity as a Famiq Progress bar's value.
#[derive(Component)]
pub struct IsFamiqProgressValue;

/// Component storing the progress bar entity associated with its value.
#[derive(Component)]
pub struct ProgressBarEntity(pub Entity);

/// Component storing the progress value entity associated with its bar.
#[derive(Component)]
pub struct ProgressValueEntity(pub Entity);

/// Component storing percentage of a progress bar.
/// None means indeterminate.
#[derive(Component)]
pub struct ProgressValuePercentage(pub Option<f32>);

#[derive(Component)]
pub struct ProgressValueColor(pub Color);
