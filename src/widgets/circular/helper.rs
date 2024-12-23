use bevy::prelude::*;
use crate::widgets::color::*;
use crate::utils;
use super::{CircularSize, CircularVariant};

pub fn get_outer_circle_size(size: &CircularSize) -> (Val, Val) {
    let size_small = Val::Px(32.0);
    let size_normal = Val::Px(38.0);
    let size_large = Val::Px(44.0);

    match size {
        CircularSize::Small => (size_small, size_small),
        CircularSize::Large => (size_large, size_large),
        _ => (size_normal, size_normal)
    }
}

pub fn get_outer_circle_border_color(variant: &CircularVariant) -> BorderColor {
    match variant {
        CircularVariant::Default => BorderColor(Color::srgba(0.812, 0.812, 0.812, 1.0)),
        CircularVariant::Primary => BorderColor(PRIMARY_COLOR),
        CircularVariant::Secondary => BorderColor(SECONDARY_COLOR),
        CircularVariant::Success => BorderColor(SUCCESS_COLOR),
        CircularVariant::Danger => BorderColor(DANGER_COLOR),
        CircularVariant::Warning => BorderColor(WARNING_COLOR),
        CircularVariant::Info => BorderColor(INFO_COLOR),
    }
}

// all Circular size have same border width
pub fn get_outer_circle_border_width() -> Val {
    return Val::Px(5.0);
}

pub fn get_inner_circle_size(size: &CircularSize) -> (Val, Val) {
    let size_small = Val::Px(4.0);
    let size_normal = Val::Px(6.0);
    let size_large = Val::Px(8.0);

    match size {
        CircularSize::Small => (size_small, size_small),
        CircularSize::Large => (size_large, size_large),
        _ => (size_normal, size_normal)
    }
}

pub fn default_outer_circle_node(size: &CircularSize) -> Node {
    let (width, height) = get_outer_circle_size(size);
    let border_width = get_outer_circle_border_width();
    Node {
        width,
        height,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(border_width),
        ..default()
    }
}

pub fn default_inner_circle_node(size: &CircularSize) -> Node {
    let mut height: f32 = 0.0;

    let (_, outer_height) = get_outer_circle_size(size);
    let outer_border_width = get_outer_circle_border_width();

    if let Some((out_height)) = utils::extract_val(outer_height) {
        height = out_height;
    }
    if let Some((border_width)) = utils::extract_val(outer_border_width) {
        height = height - (border_width * 2.0);
    }

    Node {
        width: Val::Percent(100.0),
        height: Val::Px(height),
        position_type: PositionType::Absolute,
        ..default()
    }
}
