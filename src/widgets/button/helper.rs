use crate::widgets::{button::*, color::*};
use bevy::prelude::*;

pub fn get_text_size(size: &BtnSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        BtnSize::Small => size_small,
        BtnSize::Normal => size_normal,
        BtnSize::Large => size_large,
    };
    text_size
}

pub fn get_text_color(variant: &BtnColor) -> Color {
    // all variants have text color white except Warning & Defauklt variant which
    // has text color black

    match variant {
        BtnColor::Warning => BLACK_COLOR,
        BtnColor::Default => BLACK_COLOR,
        _ => WHITE_COLOR,
    }
}

pub fn get_button_background_color(variant: &BtnColor) -> BackgroundColor {
    let bg_color: BackgroundColor = match variant {
        BtnColor::Default => BackgroundColor(BUTTON_DEFAULT_COLOR),
        BtnColor::Primary => BackgroundColor(PRIMARY_COLOR),
        BtnColor::Secondary => BackgroundColor(SECONDARY_COLOR),
        BtnColor::Success => BackgroundColor(SUCCESS_COLOR),
        BtnColor::Danger => BackgroundColor(DANGER_COLOR),
        BtnColor::Warning => BackgroundColor(WARNING_COLOR),
        BtnColor::Info => BackgroundColor(INFO_COLOR),
    };
    bg_color
}

pub fn get_button_border_color(variant: &BtnColor) -> BorderColor {
    let border_color: BorderColor = match variant {
        BtnColor::Default => BorderColor(BUTTON_DEFAULT_COLOR),
        BtnColor::Primary => BorderColor(PRIMARY_COLOR),
        BtnColor::Secondary => BorderColor(SECONDARY_COLOR),
        BtnColor::Success => BorderColor(SUCCESS_COLOR),
        BtnColor::Danger => BorderColor(DANGER_COLOR),
        BtnColor::Warning => BorderColor(WARNING_COLOR),
        BtnColor::Info => BorderColor(INFO_COLOR),
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

pub fn default_button_text_container_node(height: Val, border_width: Val) -> Node {
    Node {
        width: Val::Auto,
        height,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(border_width),
        ..default()
    }
}

pub fn default_button_node(height: Val) -> Node {
    Node {
        width: Val::Auto,
        height,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(0.0)),
        ..default()
    }
}
