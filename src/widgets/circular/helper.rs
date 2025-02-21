use bevy::prelude::*;
use crate::widgets::color::*;
use super::*;

pub fn get_circular_size(size: &WidgetSize) -> (Val, Val) {
    let size_small = Val::Px(40.0);
    let size_normal = Val::Px(50.0);
    let size_large = Val::Px(65.0);

    match size {
        WidgetSize::Small => (size_small, size_small),
        WidgetSize::Large => (size_large, size_large),
        WidgetSize::Custom(v) => (Val::Px(*v), Val::Px(*v)),
        _ => (size_normal, size_normal)
    }
}

pub fn default_circular_node(size: &WidgetSize) -> Node {
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

pub fn get_circular_color(color: &WidgetColor) -> Color {
    let bg_color = match color {
        WidgetColor::Primary => PRIMARY_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_DARK_COLOR,
        WidgetColor::Secondary => SECONDARY_COLOR,
        WidgetColor::Success => SUCCESS_COLOR,
        WidgetColor::SuccessDark => SUCCESS_DARK_COLOR,
        WidgetColor::Danger => DANGER_COLOR,
        WidgetColor::DangerDark => DANGER_DARK_COLOR,
        WidgetColor::Warning => WARNING_COLOR,
        WidgetColor::WarningDark => WARNING_DARK_COLOR,
        WidgetColor::Info => INFO_COLOR,
        WidgetColor::InfoDark => INFO_DARK_COLOR,
        WidgetColor::Custom(color) => {
            if let Some(parsed_color) = built_in_color_parser(color) {
                parsed_color
            } else {
                DEFAULT_COLOR
            }
        },
        _ => DEFAULT_COLOR,
    };
    bg_color
}
