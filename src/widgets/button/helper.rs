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
    // all color have text color white except Warning & Default color which
    // has text color black

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

pub fn get_button_background_color(color: &WidgetColor) -> BackgroundColor {
    let bg_color: BackgroundColor = match color {
        WidgetColor::Primary => BackgroundColor(PRIMARY_COLOR),
        WidgetColor::PrimaryDark => BackgroundColor(PRIMARY_DARK_COLOR),
        WidgetColor::Secondary => BackgroundColor(SECONDARY_COLOR),
        WidgetColor::Success => BackgroundColor(SUCCESS_COLOR),
        WidgetColor::SuccessDark => BackgroundColor(SUCCESS_DARK_COLOR),
        WidgetColor::Danger => BackgroundColor(DANGER_COLOR),
        WidgetColor::DangerDark => BackgroundColor(DANGER_DARK_COLOR),
        WidgetColor::Warning => BackgroundColor(WARNING_COLOR),
        WidgetColor::WarningDark => BackgroundColor(WARNING_DARK_COLOR),
        WidgetColor::Info => BackgroundColor(INFO_COLOR),
        WidgetColor::InfoDark => BackgroundColor(INFO_DARK_COLOR),
        _ => BackgroundColor(DEFAULT_COLOR)
    };
    bg_color
}

pub fn get_button_border_color(variant: &WidgetColor) -> BorderColor {
    let border_color: BorderColor = match variant {
        WidgetColor::Primary => BorderColor(PRIMARY_COLOR),
        WidgetColor::PrimaryDark => BorderColor(PRIMARY_DARK_COLOR),
        WidgetColor::Secondary => BorderColor(SECONDARY_COLOR),
        WidgetColor::Success => BorderColor(SUCCESS_COLOR),
        WidgetColor::SuccessDark => BorderColor(SUCCESS_DARK_COLOR),
        WidgetColor::Danger => BorderColor(DANGER_COLOR),
        WidgetColor::DangerDark => BorderColor(DANGER_DARK_COLOR),
        WidgetColor::Warning => BorderColor(WARNING_COLOR),
        WidgetColor::WarningDark => BorderColor(WARNING_DARK_COLOR),
        WidgetColor::Info => BorderColor(INFO_COLOR),
        WidgetColor::InfoDark => BorderColor(INFO_DARK_COLOR),
        _ => BorderColor(DEFAULT_COLOR)
    };
    border_color
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
