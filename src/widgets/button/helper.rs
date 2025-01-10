use crate::widgets::{button::*, color::*};
use bevy::prelude::*;

pub fn get_text_size(size: &BtnSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    match size {
        BtnSize::Small => size_small,
        BtnSize::Normal => size_normal,
        BtnSize::Large => size_large,
    }
}

pub fn get_text_color(variant: &BtnColor) -> Color {
    // all variants have text color white except Warning & Defauklt variant which
    // has text color black

    match variant {
        BtnColor::Secondary => WHITE_COLOR,
        BtnColor::PrimaryDark => PRIMARY_COLOR,
        BtnColor::SuccessDark => SUCCESS_COLOR,
        BtnColor::DangerDark => DANGER_COLOR,
        BtnColor::WarningDark => WARNING_COLOR,
        BtnColor::InfoDark => INFO_COLOR,
        _ => BLACK_COLOR,
    }
}

pub fn get_button_background_color(variant: &BtnColor) -> BackgroundColor {
    let bg_color: BackgroundColor = match variant {
        BtnColor::Default => BackgroundColor(BUTTON_DEFAULT_COLOR),
        BtnColor::Primary => BackgroundColor(PRIMARY_COLOR),
        BtnColor::PrimaryDark => BackgroundColor(PRIMARY_DARK_COLOR),
        BtnColor::Secondary => BackgroundColor(SECONDARY_COLOR),
        BtnColor::Success => BackgroundColor(SUCCESS_COLOR),
        BtnColor::SuccessDark => BackgroundColor(SUCCESS_DARK_COLOR),
        BtnColor::Danger => BackgroundColor(DANGER_COLOR),
        BtnColor::DangerDark => BackgroundColor(DANGER_DARK_COLOR),
        BtnColor::Warning => BackgroundColor(WARNING_COLOR),
        BtnColor::WarningDark => BackgroundColor(WARNING_DARK_COLOR),
        BtnColor::Info => BackgroundColor(INFO_COLOR),
        BtnColor::InfoDark => BackgroundColor(INFO_DARK_COLOR),
    };
    bg_color
}

pub fn get_button_border_color(variant: &BtnColor) -> BorderColor {
    let border_color: BorderColor = match variant {
        BtnColor::Default => BorderColor(BUTTON_DEFAULT_COLOR),
        BtnColor::Primary => BorderColor(PRIMARY_COLOR),
        BtnColor::PrimaryDark => BorderColor(PRIMARY_DARK_COLOR),
        BtnColor::Secondary => BorderColor(SECONDARY_COLOR),
        BtnColor::Success => BorderColor(SUCCESS_COLOR),
        BtnColor::SuccessDark => BorderColor(SUCCESS_DARK_COLOR),
        BtnColor::Danger => BorderColor(DANGER_COLOR),
        BtnColor::DangerDark => BorderColor(DANGER_DARK_COLOR),
        BtnColor::Warning => BorderColor(WARNING_COLOR),
        BtnColor::WarningDark => BorderColor(WARNING_DARK_COLOR),
        BtnColor::Info => BorderColor(INFO_COLOR),
        BtnColor::InfoDark => BorderColor(INFO_DARK_COLOR),
    };
    border_color
}

pub fn get_button_size(size: BtnSize) -> (Val, Val) {
    let size_small = (Val::Px(24.0), Val::Px(2.0));
    let size_normal = (Val::Px(30.0), Val::Px(5.0));
    let size_large = (Val::Px(36.0), Val::Px(5.0));

    let (height, border_width) = match size {
        BtnSize::Small => size_small,
        BtnSize::Normal => size_normal,
        BtnSize::Large => size_large,
    };

    (height, border_width)
}

pub fn default_button_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect {
            left: Val::Px(6.0),
            right: Val::Px(6.0),
            top: Val::Px(2.0),
            bottom: Val::Px(2.0)
        },
        margin: UiRect {
            top: Val::Px(5.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(5.0),
        },
        ..default()
    }
}
