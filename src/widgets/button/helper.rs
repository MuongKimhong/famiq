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

pub fn get_text_color(variant: &BtnVariant) -> Color {
    // all variants have text color white except Warning & Defauklt variant which
    // has text color black

    match variant {
        BtnVariant::Warning => BLACK_COLOR,
        BtnVariant::Default => BLACK_COLOR,
        _ => WHITE_COLOR,
    }
}

pub fn get_button_background_color(variant: &BtnVariant) -> BackgroundColor {
    let bg_color: BackgroundColor = match variant {
        BtnVariant::Default => BackgroundColor(BUTTON_DEFAULT_COLOR),
        BtnVariant::Primary => BackgroundColor(PRIMARY_COLOR),
        BtnVariant::Secondary => BackgroundColor(SECONDARY_COLOR),
        BtnVariant::Success => BackgroundColor(SUCCESS_COLOR),
        BtnVariant::Danger => BackgroundColor(DANGER_COLOR),
        BtnVariant::Warning => BackgroundColor(WARNING_COLOR),
        BtnVariant::Info => BackgroundColor(INFO_COLOR),
    };
    bg_color
}

pub fn get_button_border_color(variant: &BtnVariant) -> BorderColor {
    let border_color: BorderColor = match variant {
        BtnVariant::Default => BorderColor(BUTTON_DEFAULT_COLOR),
        BtnVariant::Primary => BorderColor(PRIMARY_COLOR),
        BtnVariant::Secondary => BorderColor(SECONDARY_COLOR),
        BtnVariant::Success => BorderColor(SUCCESS_COLOR),
        BtnVariant::Danger => BorderColor(DANGER_COLOR),
        BtnVariant::Warning => BorderColor(WARNING_COLOR),
        BtnVariant::Info => BorderColor(INFO_COLOR),
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
