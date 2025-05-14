use bevy::prelude::*;

/// Marker component indentifying Famiq scroll widget.
#[derive(Component)]
pub struct IsFamiqScroll;

/// Marker component identifying scroll move panel.
#[derive(Component)]
pub struct IsFamiqScrollMovePanel;

#[derive(Component)]
pub struct ScrollMovePanelEntity(pub Entity);

#[derive(Component)]
pub struct ScrollList {
    /// current scroll position
    pub position: f32,
    /// target scroll position
    pub target_position: f32,
    pub scroll_height: f32,
}

impl ScrollList {
    pub fn new(scroll_height: f32) -> Self {
        Self {
            position: 0.0,
            scroll_height,
            target_position: 0.0
        }
    }
}
