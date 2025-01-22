use bevy::prelude::*;
use crate::widgets::color::*;
use crate::utils::extract_val;
use super::{CircularSize, CircularColor};

pub fn get_outer_circle_size(size: &CircularSize) -> (Val, Val) {
    let size_small = Val::Px(40.0);
    let size_normal = Val::Px(46.0);
    let size_large = Val::Px(52.0);

    match size {
        CircularSize::Small => (size_small, size_small),
        CircularSize::Large => (size_large, size_large),
        CircularSize::CustomSize(v) => (Val::Px(*v), Val::Px(*v)),
        _ => (size_normal, size_normal)
    }
}

pub fn get_spinner_color(variant: &CircularColor) -> BorderColor {
    match variant {
        CircularColor::Default => BorderColor(Color::srgba(0.812, 0.812, 0.812, 1.0)),
        CircularColor::Primary => BorderColor(PRIMARY_COLOR),
        CircularColor::Secondary => BorderColor(SECONDARY_COLOR),
        CircularColor::Success => BorderColor(SUCCESS_COLOR),
        CircularColor::Danger => BorderColor(DANGER_COLOR),
        CircularColor::Warning => BorderColor(WARNING_COLOR),
        CircularColor::Info => BorderColor(INFO_COLOR),
    }
}

// all Circular size have same border width
pub fn get_outer_circle_border_width(size: &CircularSize) -> Val {
    match size {
        CircularSize::Small => Val::Px(3.0),
        CircularSize::Large => Val::Px(4.0),
        _ => Val::Px(3.5)
    }
}

pub fn default_outer_circle_node(size: &CircularSize) -> Node {
    let (width, height) = get_outer_circle_size(size);
    let border_width = get_outer_circle_border_width(size);
    Node {
        width,
        height,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(border_width),
        margin: UiRect {
            top: Val::Px(2.0),
            bottom: Val::Px(2.0),
            ..default()
        },
        ..default()
    }
}

pub fn default_spinner_node(size: &CircularSize) -> Node {
    let outer_border_width = get_outer_circle_border_width(size);

    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        margin: UiRect::all(Val::Px(-extract_val(outer_border_width).unwrap())),
        border: UiRect {
            left: Val::Px(0.0),
            right: outer_border_width,
            top: outer_border_width,
            bottom: outer_border_width,
        },
        ..default()
    }
}
