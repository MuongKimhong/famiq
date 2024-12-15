use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ListView {
    pub focused: bool,
    pub items: Vec<Entity>,
}

impl ListView {
    pub fn new(items: &Vec<Entity>) -> Self {
        Self {
            focused: false,
            items: items.clone(),
        }
    }
}

#[derive(Component)]
pub struct IsFamiqListView;

#[derive(Component)]
pub struct IsFamiqListViewMovePanel;

#[derive(Component)]
pub struct ListViewMovePanelEntity(pub Entity);

#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

#[derive(Component)]
pub struct IsFamiqListViewItem;
