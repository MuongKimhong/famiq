use bevy::prelude::*;
use crate::widgets::color::*;
use super::{CircularSize, CircularColor};

pub fn get_circular_size(size: &CircularSize) -> (Val, Val) {
    let size_small = Val::Px(40.0);
    let size_normal = Val::Px(50.0);
    let size_large = Val::Px(65.0);

    match size {
        CircularSize::Small => (size_small, size_small),
        CircularSize::Large => (size_large, size_large),
        CircularSize::CustomSize(v) => (Val::Px(*v), Val::Px(*v)),
        _ => (size_normal, size_normal)
    }
}

pub fn default_circular_node(size: &CircularSize) -> Node {
    let (width, height) = get_circular_size(size);
    Node {
        width,
        height,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(0.0)),
        margin: UiRect {
            top: Val::Px(2.0),
            bottom: Val::Px(2.0),
            ..default()
        },
        ..default()
    }
}

pub fn get_circular_color(color: &CircularColor) -> Color {
    let bg_color = match color {
        CircularColor::Default => BUTTON_DEFAULT_COLOR,
        CircularColor::Primary => PRIMARY_COLOR,
        CircularColor::PrimaryDark => PRIMARY_DARK_COLOR,
        CircularColor::Secondary => SECONDARY_COLOR,
        CircularColor::Success => SUCCESS_COLOR,
        CircularColor::SuccessDark => SUCCESS_DARK_COLOR,
        CircularColor::Danger => DANGER_COLOR,
        CircularColor::DangerDark => DANGER_DARK_COLOR,
        CircularColor::Warning => WARNING_COLOR,
        CircularColor::WarningDark => WARNING_DARK_COLOR,
        CircularColor::Info => INFO_COLOR,
        CircularColor::InfoDark => INFO_DARK_COLOR,
    };
    bg_color
}
