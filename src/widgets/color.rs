use bevy::prelude::*;
use bevy::color::palettes::{
    basic,
    basic::*,
    css::*
};

pub const PRIMARY_COLOR: Color = Color::srgba(0.537, 0.686, 0.969, 1.0);
pub const PRIMARY_DARK_COLOR: Color = Color::srgba(0.118, 0.302, 0.639, 1.0);
pub const SECONDARY_COLOR: Color = Color::srgba(0.49, 0.49, 0.49, 1.0);
pub const SECONDARY_DARK_COLOR: Color = Color::srgba(0.251, 0.251, 0.251, 1.0);
pub const SUCCESS_COLOR: Color = Color::srgba(0.043, 1.0, 0.039, 1.0);
pub const SUCCESS_DARK_COLOR: Color = Color::srgba(0.067, 0.6, 0.18, 1.0);
pub const GREEN_COLOR: Color = Color::srgba(0.027, 1.0, 0.0, 0.961);
pub const LIGHT_GREEN_COLOR: Color = Color::srgba(0.784, 1.0, 0.137, 0.961);
pub const DANGER_COLOR: Color = Color::srgba(0.961, 0.0, 0.0, 1.0);
pub const DANGER_DARK_COLOR: Color = Color::srgba(0.529, 0.0, 0.0, 1.0);
pub const WARNING_COLOR: Color = Color::srgba(1.0, 0.722, 0.2, 1.0);
pub const WARNING_DARK_COLOR: Color = Color::srgba(0.69, 0.447, 0.0, 1.0);
pub const INFO_COLOR: Color = Color::srgba(0.0, 1.0, 1.0, 1.0);
pub const INFO_DARK_COLOR: Color = Color::srgba(0.0, 0.541, 0.541, 1.0);
pub const BLACK_COLOR: Color = Color::srgba(0.11, 0.11, 0.11, 0.902);
pub const WHITE_COLOR: Color = Color::srgba(0.969, 0.969, 0.969, 0.902);
pub const BUTTON_DEFAULT_COLOR: Color = Color::srgba(0.95, 0.95, 0.95, 0.902);

pub fn built_in_color_parser(value: &str) -> Option<Color> {
    match value.trim().to_lowercase().as_str() {
        "white" => Some(WHITE_COLOR),
        "black" => Some(BLACK_COLOR),
        "red" => Some(Color::from(RED)),
        "aqua" => Some(Color::from(basic::AQUA)),
        "blue" => Some(Color::from(BLUE)),
        "fuchsia" => Some(Color::from(FUCHSIA)),
        "gray" => Some(Color::from(GREY)),
        "green" => Some(Color::from(GREEN)),
        "lime" => Some(Color::from(LIME)),
        "maroon" => Some(Color::from(MAROON)),
        "navy" => Some(Color::from(NAVY)),
        "olive" => Some(Color::from(OLIVE)),
        "purple" => Some(Color::from(PURPLE)),
        "silver" => Some(Color::from(SILVER)),
        "teal" => Some(Color::from(TEAL)),
        "yellow" => Some(Color::from(YELLOW)),
        "alice_blue" => Some(Color::from(ALICE_BLUE)),
        "antique_white" => Some(Color::from(ANTIQUE_WHITE)),
        "aquamarine" => Some(Color::from(AQUAMARINE)),
        "azure" => Some(Color::from(AZURE)),
        "beige" => Some(Color::from(BEIGE)),
        "blue_violet" => Some(Color::from(BLUE_VIOLET)),
        "brown" => Some(Color::from(BROWN)),
        "burlywood" => Some(Color::from(BURLYWOOD)),
        "cadet_blue" => Some(Color::from(CADET_BLUE)),
        "chocolate" => Some(Color::from(CHOCOLATE)),
        "coral" => Some(Color::from(CORAL)),
        "dark_blue" => Some(Color::from(DARK_BLUE)),
        "dark_cyan" => Some(Color::from(DARK_CYAN)),
        "dark_gray" => Some(Color::from(DARK_GRAY)),
        "dark_green" => Some(Color::from(DARK_GREEN)),
        "dark_grey" => Some(Color::from(DARK_GREY)),
        "dark_khaki" => Some(Color::from(DARK_KHAKI)),
        "dark_magenta" => Some(Color::from(DARK_MAGENTA)),
        "dark_olivegreen" => Some(Color::from(DARK_OLIVEGREEN)),
        "dark_orange" => Some(Color::from(DARK_ORANGE)),
        "dark_red" => Some(Color::from(DARK_RED)),
        "dark_salmon" => Some(Color::from(DARK_SALMON)),
        "dark_violet" => Some(Color::from(DARK_VIOLET)),
        "gold" => Some(Color::from(GOLD)),
        "pink" => Some(Color::from(PINK)),
        "violet" => Some(Color::from(VIOLET)),
        _ => None
    }
}
