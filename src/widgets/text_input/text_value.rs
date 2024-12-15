use crate::utils::strip_assets_prefix;
use crate::widgets::text_input::*;
use bevy::prelude::*;

pub const PLACEHOLDER_COLOR: Color = Color::srgba(0.651, 0.651, 0.651, 0.6);
pub const TEXT_INPUT_VALUE_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.922);

pub fn get_text_size(size: &Option<TextInputSize>) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        Some(TextInputSize::Small) => size_small,
        Some(TextInputSize::Normal) => size_normal,
        Some(TextInputSize::Large) => size_large,
        None => size_normal,
    };
    text_size
}

pub fn create_text_input_value(
    text: &str,
    input_size: &Option<TextInputSize>,
    asset_server: &ResMut<AssetServer>,
    font_path: &String,
) -> TextBundle {
    // default value in text input is placeholder value
    // when focused & users start typing, change color to TEXT_INPUT_VALUE_COLOR
    let path = strip_assets_prefix(font_path).unwrap();
    TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load(path),
            font_size: get_text_size(input_size),
            color: PLACEHOLDER_COLOR,
        },
    )
}
