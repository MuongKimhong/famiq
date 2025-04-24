use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqCheckbox;

#[derive(Component)]
pub struct IsFamiqCheckboxItem;

#[derive(Component)]
pub struct IsFamiqCheckboxItemBox;

#[derive(Component)]
pub struct CheckBoxMainContainerEntity(pub Entity);

#[derive(Component)]
pub struct CheckBoxItemText(pub String);

#[derive(Component)]
pub struct CheckBoxItemBoxEntity(pub Entity);

#[derive(Component)]
pub struct CheckBoxChoiceTicked(pub bool);
