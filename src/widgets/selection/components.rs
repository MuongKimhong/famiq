use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqSelectionContainer;

#[derive(Component)]
pub struct IsFamiqSelectionSelector;

#[derive(Component)]
pub struct IsFamiqSelectionChoicesPanel;

#[derive(Component)]
pub struct ArrowIcon;

#[derive(Component)]
pub struct SelectorPlaceHolder;

#[derive(Component, Clone)]
pub struct SelectorPlaceHolderEntity(pub Entity);

#[derive(Component)]
pub struct SelectorArrowIconEntity(pub Entity);

#[derive(Component)]
pub struct SelectionChoicesPanelEntity(pub Entity);

/// Component storing selector entity, to be used in _build_choice_container
#[derive(Component)]
pub struct SelectorEntity(pub Entity);

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
pub struct IsFamiqSelectionChoice;

#[derive(Component, Deref, Debug)]
pub struct SelectionChoiceTextEntity(pub Entity);

#[derive(Component)]
pub struct IsFamiqSelectionLabel;

#[derive(Component, Deref, Debug)]
pub struct SelectionLabelEntity(pub Entity);
