use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqSelectionContainer;

#[derive(Component)]
pub struct IsFamiqSelection;

#[derive(Component)]
pub struct IsFamiqSelectionItemsPanel;

#[derive(Component)]
pub struct ArrowIcon;

#[derive(Component)]
pub struct SelectorPlaceHolder;

#[derive(Component, Clone)]
pub struct SelectorPlaceHolderEntity(pub Entity);

#[derive(Component)]
pub struct SelectorArrowIconEntity(pub Entity);

#[derive(Component)]
pub struct SelectionItemsPanelEntity(pub Entity);

#[derive(Component, Debug)]
pub struct Selection {
    pub placeholder: String,
    pub focused: bool,
}

impl Selection {
    pub fn new(placeholder: String) -> Self {
        Self {
            placeholder,
            focused: false,
        }
    }
}

#[derive(Component)]
pub struct IsFamiqSelectionItem;

#[derive(Component, Deref, Debug)]
pub struct SelectionItemTextEntity(pub Entity);

#[derive(Component)]
pub struct IsFamiqSelectionLabel;

#[derive(Component, Deref, Debug)]
pub struct SelectionLabelEntity(pub Entity);
