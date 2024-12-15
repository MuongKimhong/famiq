use crate::utils::strip_assets_prefix;
use crate::widgets::{button::*, color::*};
use bevy::prelude::*;

pub fn get_text_size(size: &Option<BtnSize>) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        Some(BtnSize::Small) => size_small,
        Some(BtnSize::Normal) => size_normal,
        Some(BtnSize::Large) => size_large,
        None => size_normal,
    };
    text_size
}

pub fn get_text_color(variant: &Option<BtnVariant>) -> Color {
    // all variants have text color white except Warning & Defauklt variant which
    // has text color black

    match variant {
        Some(BtnVariant::Warning) => BLACK_COLOR,
        Some(BtnVariant::Default) => BLACK_COLOR,
        None => BLACK_COLOR,
        _ => WHITE_COLOR,
    }
}

pub fn create_button_text(
    text: &str,
    btn_size: &Option<BtnSize>,
    variant: &Option<BtnVariant>,
    asset_server: &ResMut<AssetServer>,
    font_path: &String,
) -> TextBundle {
    let path = strip_assets_prefix(font_path).unwrap();
    let font_handle = asset_server.load(path);

    TextBundle::from_section(
        text,
        TextStyle {
            font: font_handle,
            font_size: get_text_size(btn_size),
            color: get_text_color(variant),
        },
    )
}
