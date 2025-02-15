use bevy::prelude::*;

#[derive(Component)]
pub struct FaButtonText(pub String);

/// Marker component for identifying an entity as a Famiq button.
#[derive(Component)]
pub struct IsFamiqButton;

/// Marker component for identifying an entity as a Famiq button's text.
#[derive(Component)]
pub struct IsFamiqButtonText;

#[derive(Component)]
pub struct IsFamiqButtonOverlay;

#[derive(Component)]
pub struct ButtonOverlayEntity(pub Entity);

/// Component storing the entity associated with a button's text.
///
/// Used to identify which text belongs to which button.
#[derive(Component)]
pub struct ButtonTextEntity(pub Entity);

/// Built-in button color options for `fa_button`.
#[derive(Component)]
pub enum BtnColor {
    Default,
    Primary,
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark
}

/// Built-in button size options for `fa_button`.
#[derive(Component)]
pub enum BtnSize {
    Small,
    Normal,
    Large,
}
/// Built-in shape options for `fa_button`.
#[derive(Component)]
pub enum BtnShape {
    Default,
    Round,
    Rectangle
}
