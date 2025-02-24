use crate::widgets::{button::*, color::*};
use bevy::prelude::*;

pub fn get_text_size(size: &WidgetSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    match size {
        WidgetSize::Small => size_small,
        WidgetSize::Large => size_large,
        _ => size_normal
    }
}

pub fn get_text_color(variant: &WidgetColor) -> Color {
    match variant {
        WidgetColor::Secondary => WHITE_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_COLOR,
        WidgetColor::SuccessDark => SUCCESS_COLOR,
        WidgetColor::DangerDark => DANGER_COLOR,
        WidgetColor::WarningDark => WARNING_COLOR,
        WidgetColor::InfoDark => INFO_COLOR,
        _ => BLACK_COLOR,
    }
}

pub fn default_button_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(0.0)),
        padding: UiRect {
            left: Val::Px(6.0),
            right: Val::Px(6.0),
            top: Val::Px(2.0),
            bottom: Val::Px(2.0)
        },
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        ..default()
    }
}

pub fn default_button_overlay_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect::all(Val::Px(0.0)),
        margin: UiRect::all(Val::Px(0.0)),
        ..default()
    }
}
