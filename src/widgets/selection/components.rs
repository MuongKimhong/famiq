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

#[derive(Component)]
pub struct SelectorEntity(pub Entity);

#[derive(Component, Default)]
pub struct SelectionValue(pub String);

#[derive(Component, Debug, Default)]
pub struct SelectorPlaceholderText(pub String);

#[derive(Component)]
pub struct IsFamiqSelectionChoice;

#[derive(Component, Deref, Debug)]
pub struct SelectionChoiceTextEntity(pub Entity);

#[derive(Component)]
pub struct IsFamiqSelectionLabel;

#[derive(Component, Deref, Debug)]
pub struct SelectionLabelEntity(pub Entity);
