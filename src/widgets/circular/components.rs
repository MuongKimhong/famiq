use bevy::prelude::*;

/// Marker component for identifying a Famiq circular widget.
#[derive(Component)]
pub struct IsFamiqCircular;

/// Marker component for identifying a Famiq circular spinner.
#[derive(Component)]
pub struct IsFamiqCircularSpinner;

/// Component for managing the rotation sequence of a spinner.
///
/// # Fields
/// - `speed`: Current rotation speed in degrees per second.
/// - `timer`: Timer for managing speed transitions.
/// - `speed_sequence`: Sequence of rotation speeds.
/// - `current_index`: Current index in the rotation speed sequence.
#[derive(Component)]
pub struct RotatingSequence {
    pub speed: f32,
    pub timer: Timer,
    pub speed_sequence: Vec<f32>,
    pub current_index: usize,
}

impl Default for RotatingSequence {
    fn default() -> Self {
        RotatingSequence {
            speed: 300.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            speed_sequence: vec![300.0, 500.0, 300.0],
            current_index: 0,
        }
    }
}

/// Component for associating a spinner entity with its parent circular widget.
#[derive(Component)]
pub struct CircularSpinnerEntity(pub Entity);
