use bevy::color::palettes::tailwind::{AMBER_50, CYAN_50};
use bevy::prelude::*;
use bevy::color::palettes::{
    basic,
    basic::*,
    tailwind::*,
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
pub const BLACK_COLOR: Color = Color::srgba(0.11, 0.11, 0.11, 1.0);
pub const WHITE_COLOR: Color = Color::srgba(0.969, 0.969, 0.969, 1.0);
pub const DEFAULT_COLOR: Color = Color::srgba(0.95, 0.95, 0.95, 1.0);

/// Convert Hex color to bevy srgba
pub fn hex_color_parser(hex: &str) -> Option<Color> {
    // remove # if contains any
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 8 {
        return None;
    }

    // srgba expects 1 - 0 range
    let r_value = match u8::from_str_radix(&hex[0..2], 16) {
        Ok(v) => v as f32 / 255.0,
        Err(_) => return None
    };
    let g_value = match u8::from_str_radix(&hex[2..4], 16) {
        Ok(v) => v as f32 / 255.0,
        Err(_) => return None
    };
    let b_value = match u8::from_str_radix(&hex[4..6], 16) {
        Ok(v) => v as f32 / 255.0,
        Err(_) => return None
    };

    // Check if hex code contains alpha channel
    let alpha_value;
    if hex.len() == 8 {
        alpha_value = match u8::from_str_radix(&hex[6..8], 16) {
            Ok(v) => v as f32 / 255.0,
            Err(_) => return None
        };
    } else {
        alpha_value = 1.0 as f32;
    }

    Some(Color::srgba(r_value, g_value, b_value, alpha_value))
}

/// Supported colors via json style and widget buiders.
pub fn built_in_color_parser(value: &str) -> Option<Color> {
    match value.trim().to_lowercase().as_str() {
        "white" => Some(WHITE_COLOR),
        "black" => Some(BLACK_COLOR),
        "red" => Some(Color::from(RED)),
        "aqua" => Some(Color::from(basic::AQUA)),
        "cyan" => Some(Color::from(CYAN_50)),
        "light_cyan" => Some(Color::from(LIGHT_CYAN)),
        "dark_cyan" => Some(Color::from(DARK_CYAN)),
        "blue" => Some(Color::from(BLUE)),
        "fuchsia" => Some(Color::from(FUCHSIA)),
        "gray" => Some(Color::from(GRAY)),
        "grey" => Some(Color::from(GREY)),
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
        "ivory" => Some(Color::from(IVORY)),
        "steel_blue" => Some(Color::from(STEEL_BLUE)),
        "wheat" => Some(Color::from(WHEAT)),
        "rosy_brown" => Some(Color::from(ROSY_BROWN)),

        // tailwind
        "amber_50" => Some(Color::from(AMBER_50)),
        "amber_100" => Some(Color::from(AMBER_100)),
        "amber_200" => Some(Color::from(AMBER_200)),
        "amber_300" => Some(Color::from(AMBER_300)),
        "amber_400" => Some(Color::from(AMBER_400)),
        "amber_500" => Some(Color::from(AMBER_500)),
        "amber_600" => Some(Color::from(AMBER_600)),
        "amber_700" => Some(Color::from(AMBER_700)),
        "amber_800" => Some(Color::from(AMBER_800)),
        "amber_900" => Some(Color::from(AMBER_900)),
        "amber_950" => Some(Color::from(AMBER_950)),
        "blue_50" => Some(Color::from(BLUE_50)),
        "blue_100" => Some(Color::from(BLUE_100)),
        "blue_200" => Some(Color::from(BLUE_200)),
        "blue_300" => Some(Color::from(BLUE_300)),
        "blue_400" => Some(Color::from(BLUE_400)),
        "blue_500" => Some(Color::from(BLUE_500)),
        "blue_600" => Some(Color::from(BLUE_600)),
        "blue_700" => Some(Color::from(BLUE_700)),
        "blue_800" => Some(Color::from(BLUE_800)),
        "blue_900" => Some(Color::from(BLUE_900)),
        "blue_950" => Some(Color::from(BLUE_950)),
        "cyan_50" => Some(Color::from(CYAN_50)),
        "cyan_100" => Some(Color::from(CYAN_100)),
        "cyan_200" => Some(Color::from(CYAN_200)),
        "cyan_300" => Some(Color::from(CYAN_300)),
        "cyan_400" => Some(Color::from(CYAN_400)),
        "cyan_500" => Some(Color::from(CYAN_500)),
        "cyan_600" => Some(Color::from(CYAN_600)),
        "cyan_700" => Some(Color::from(CYAN_700)),
        "cyan_800" => Some(Color::from(CYAN_800)),
        "cyan_900" => Some(Color::from(CYAN_900)),
        "cyan_950" => Some(Color::from(CYAN_950)),
        "emerald_50" => Some(Color::from(EMERALD_50)),
        "emerald_100" => Some(Color::from(EMERALD_100)),
        "emerald_200" => Some(Color::from(EMERALD_200)),
        "emerald_300" => Some(Color::from(EMERALD_300)),
        "emerald_400" => Some(Color::from(EMERALD_400)),
        "emerald_500" => Some(Color::from(EMERALD_500)),
        "emerald_600" => Some(Color::from(EMERALD_600)),
        "emerald_700" => Some(Color::from(EMERALD_700)),
        "emerald_800" => Some(Color::from(EMERALD_800)),
        "emerald_900" => Some(Color::from(EMERALD_900)),
        "emerald_950" => Some(Color::from(EMERALD_950)),
        "fuchsia_50" => Some(Color::from(FUCHSIA_50)),
        "fuchsia_100" => Some(Color::from(FUCHSIA_100)),
        "fuchsia_200" => Some(Color::from(FUCHSIA_200)),
        "fuchsia_300" => Some(Color::from(FUCHSIA_300)),
        "fuchsia_400" => Some(Color::from(FUCHSIA_400)),
        "fuchsia_500" => Some(Color::from(FUCHSIA_500)),
        "fuchsia_600" => Some(Color::from(FUCHSIA_600)),
        "fuchsia_700" => Some(Color::from(FUCHSIA_700)),
        "fuchsia_800" => Some(Color::from(FUCHSIA_800)),
        "fuchsia_900" => Some(Color::from(FUCHSIA_900)),
        "fuchsia_950" => Some(Color::from(FUCHSIA_950)),
        "gray_50" => Some(Color::from(GRAY_50)),
        "gray_100" => Some(Color::from(GRAY_100)),
        "gray_200" => Some(Color::from(GRAY_200)),
        "gray_300" => Some(Color::from(GRAY_300)),
        "gray_400" => Some(Color::from(GRAY_400)),
        "gray_500" => Some(Color::from(GRAY_500)),
        "gray_600" => Some(Color::from(GRAY_600)),
        "gray_700" => Some(Color::from(GRAY_700)),
        "gray_800" => Some(Color::from(GRAY_800)),
        "gray_900" => Some(Color::from(GRAY_900)),
        "gray_950" => Some(Color::from(GRAY_950)),
        "green_50" => Some(Color::from(GREEN_50)),
        "green_100" => Some(Color::from(GREEN_100)),
        "green_200" => Some(Color::from(GREEN_200)),
        "green_300" => Some(Color::from(GREEN_300)),
        "green_400" => Some(Color::from(GREEN_400)),
        "green_500" => Some(Color::from(GREEN_500)),
        "green_600" => Some(Color::from(GREEN_600)),
        "green_700" => Some(Color::from(GREEN_700)),
        "green_800" => Some(Color::from(GREEN_800)),
        "green_900" => Some(Color::from(GREEN_900)),
        "green_950" => Some(Color::from(GREEN_950)),
        "indigo_50" => Some(Color::from(INDIGO_50)),
        "indigo_100" => Some(Color::from(INDIGO_100)),
        "indigo_200" => Some(Color::from(INDIGO_200)),
        "indigo_300" => Some(Color::from(INDIGO_300)),
        "indigo_400" => Some(Color::from(INDIGO_400)),
        "indigo_500" => Some(Color::from(INDIGO_500)),
        "indigo_600" => Some(Color::from(INDIGO_600)),
        "indigo_700" => Some(Color::from(INDIGO_700)),
        "indigo_800" => Some(Color::from(INDIGO_800)),
        "indigo_900" => Some(Color::from(INDIGO_900)),
        "indigo_950" => Some(Color::from(INDIGO_950)),
        "lime_50" => Some(Color::from(LIME_50)),
        "lime_100" => Some(Color::from(LIME_100)),
        "lime_200" => Some(Color::from(LIME_200)),
        "lime_300" => Some(Color::from(LIME_300)),
        "lime_400" => Some(Color::from(LIME_400)),
        "lime_500" => Some(Color::from(LIME_500)),
        "lime_600" => Some(Color::from(LIME_600)),
        "lime_700" => Some(Color::from(LIME_700)),
        "lime_800" => Some(Color::from(LIME_800)),
        "lime_900" => Some(Color::from(LIME_900)),
        "lime_950" => Some(Color::from(LIME_950)),
        "neutral_50" => Some(Color::from(NEUTRAL_50)),
        "neutral_100" => Some(Color::from(NEUTRAL_100)),
        "neutral_200" => Some(Color::from(NEUTRAL_200)),
        "neutral_300" => Some(Color::from(NEUTRAL_300)),
        "neutral_400" => Some(Color::from(NEUTRAL_400)),
        "neutral_500" => Some(Color::from(NEUTRAL_500)),
        "neutral_600" => Some(Color::from(NEUTRAL_600)),
        "neutral_700" => Some(Color::from(NEUTRAL_700)),
        "neutral_800" => Some(Color::from(NEUTRAL_800)),
        "neutral_900" => Some(Color::from(NEUTRAL_900)),
        "neutral_950" => Some(Color::from(NEUTRAL_950)),
        "orange_50" => Some(Color::from(ORANGE_50)),
        "orange_100" => Some(Color::from(ORANGE_100)),
        "orange_200" => Some(Color::from(ORANGE_200)),
        "orange_300" => Some(Color::from(ORANGE_300)),
        "orange_400" => Some(Color::from(ORANGE_400)),
        "orange_500" => Some(Color::from(ORANGE_500)),
        "orange_600" => Some(Color::from(ORANGE_600)),
        "orange_700" => Some(Color::from(ORANGE_700)),
        "orange_800" => Some(Color::from(ORANGE_800)),
        "orange_900" => Some(Color::from(ORANGE_900)),
        "orange_950" => Some(Color::from(ORANGE_950)),
        "pink_50" => Some(Color::from(PINK_50)),
        "pink_100" => Some(Color::from(PINK_100)),
        "pink_200" => Some(Color::from(PINK_200)),
        "pink_300" => Some(Color::from(PINK_300)),
        "pink_400" => Some(Color::from(PINK_400)),
        "pink_500" => Some(Color::from(PINK_500)),
        "pink_600" => Some(Color::from(PINK_600)),
        "pink_700" => Some(Color::from(PINK_700)),
        "pink_800" => Some(Color::from(PINK_800)),
        "pink_900" => Some(Color::from(PINK_900)),
        "pink_950" => Some(Color::from(PINK_950)),
        "purple_50" => Some(Color::from(PURPLE_50)),
        "purple_100" => Some(Color::from(PURPLE_100)),
        "purple_200" => Some(Color::from(PURPLE_200)),
        "purple_300" => Some(Color::from(PURPLE_300)),
        "purple_400" => Some(Color::from(PURPLE_400)),
        "purple_500" => Some(Color::from(PURPLE_500)),
        "purple_600" => Some(Color::from(PURPLE_600)),
        "purple_700" => Some(Color::from(PURPLE_700)),
        "purple_800" => Some(Color::from(PURPLE_800)),
        "purple_900" => Some(Color::from(PURPLE_900)),
        "purple_950" => Some(Color::from(PURPLE_950)),
        "red_50" => Some(Color::from(RED_50)),
        "red_100" => Some(Color::from(RED_100)),
        "red_200" => Some(Color::from(RED_200)),
        "red_300" => Some(Color::from(RED_300)),
        "red_400" => Some(Color::from(RED_400)),
        "red_500" => Some(Color::from(RED_500)),
        "red_600" => Some(Color::from(RED_600)),
        "red_700" => Some(Color::from(RED_700)),
        "red_800" => Some(Color::from(RED_800)),
        "red_900" => Some(Color::from(RED_900)),
        "red_950" => Some(Color::from(RED_950)),
        "rose_50" => Some(Color::from(ROSE_50)),
        "rose_100" => Some(Color::from(ROSE_100)),
        "rose_200" => Some(Color::from(ROSE_200)),
        "rose_300" => Some(Color::from(ROSE_300)),
        "rose_400" => Some(Color::from(ROSE_400)),
        "rose_500" => Some(Color::from(ROSE_500)),
        "rose_600" => Some(Color::from(ROSE_600)),
        "rose_700" => Some(Color::from(ROSE_700)),
        "rose_800" => Some(Color::from(ROSE_800)),
        "rose_900" => Some(Color::from(ROSE_900)),
        "rose_950" => Some(Color::from(ROSE_950)),
        "sky_50" => Some(Color::from(SKY_50)),
        "sky_100" => Some(Color::from(SKY_100)),
        "sky_200" => Some(Color::from(SKY_200)),
        "sky_300" => Some(Color::from(SKY_300)),
        "sky_400" => Some(Color::from(SKY_400)),
        "sky_500" => Some(Color::from(SKY_500)),
        "sky_600" => Some(Color::from(SKY_600)),
        "sky_700" => Some(Color::from(SKY_700)),
        "sky_800" => Some(Color::from(SKY_800)),
        "sky_900" => Some(Color::from(SKY_900)),
        "sky_950" => Some(Color::from(SKY_950)),
        "slate_50" => Some(Color::from(SLATE_50)),
        "slate_100" => Some(Color::from(SLATE_100)),
        "slate_200" => Some(Color::from(SLATE_200)),
        "slate_300" => Some(Color::from(SLATE_300)),
        "slate_400" => Some(Color::from(SLATE_400)),
        "slate_500" => Some(Color::from(SLATE_500)),
        "slate_600" => Some(Color::from(SLATE_600)),
        "slate_700" => Some(Color::from(SLATE_700)),
        "slate_800" => Some(Color::from(SLATE_800)),
        "slate_900" => Some(Color::from(SLATE_900)),
        "slate_950" => Some(Color::from(SLATE_950)),
        "stone_50" => Some(Color::from(STONE_50)),
        "stone_100" => Some(Color::from(STONE_100)),
        "stone_200" => Some(Color::from(STONE_200)),
        "stone_300" => Some(Color::from(STONE_300)),
        "stone_400" => Some(Color::from(STONE_400)),
        "stone_500" => Some(Color::from(STONE_500)),
        "stone_600" => Some(Color::from(STONE_600)),
        "stone_700" => Some(Color::from(STONE_700)),
        "stone_800" => Some(Color::from(STONE_800)),
        "stone_900" => Some(Color::from(STONE_900)),
        "stone_950" => Some(Color::from(STONE_950)),
        "teal_50" => Some(Color::from(TEAL_50)),
        "teal_100" => Some(Color::from(TEAL_100)),
        "teal_200" => Some(Color::from(TEAL_200)),
        "teal_300" => Some(Color::from(TEAL_300)),
        "teal_400" => Some(Color::from(TEAL_400)),
        "teal_500" => Some(Color::from(TEAL_500)),
        "teal_600" => Some(Color::from(TEAL_600)),
        "teal_700" => Some(Color::from(TEAL_700)),
        "teal_800" => Some(Color::from(TEAL_800)),
        "teal_900" => Some(Color::from(TEAL_900)),
        "teal_950" => Some(Color::from(TEAL_950)),
        "violet_50" => Some(Color::from(VIOLET_50)),
        "violet_100" => Some(Color::from(VIOLET_100)),
        "violet_200" => Some(Color::from(VIOLET_200)),
        "violet_300" => Some(Color::from(VIOLET_300)),
        "violet_400" => Some(Color::from(VIOLET_400)),
        "violet_500" => Some(Color::from(VIOLET_500)),
        "violet_600" => Some(Color::from(VIOLET_600)),
        "violet_700" => Some(Color::from(VIOLET_700)),
        "violet_800" => Some(Color::from(VIOLET_800)),
        "violet_900" => Some(Color::from(VIOLET_900)),
        "violet_950" => Some(Color::from(VIOLET_950)),
        "yellow_50" => Some(Color::from(YELLOW_50)),
        "yellow_100" => Some(Color::from(YELLOW_100)),
        "yellow_200" => Some(Color::from(YELLOW_200)),
        "yellow_300" => Some(Color::from(YELLOW_300)),
        "yellow_400" => Some(Color::from(YELLOW_400)),
        "yellow_500" => Some(Color::from(YELLOW_500)),
        "yellow_600" => Some(Color::from(YELLOW_600)),
        "yellow_700" => Some(Color::from(YELLOW_700)),
        "yellow_800" => Some(Color::from(YELLOW_800)),
        "yellow_900" => Some(Color::from(YELLOW_900)),
        "yellow_950" => Some(Color::from(YELLOW_950)),
        "zinc_50" => Some(Color::from(ZINC_50)),
        "zinc_100" => Some(Color::from(ZINC_100)),
        "zinc_200" => Some(Color::from(ZINC_200)),
        "zinc_300" => Some(Color::from(ZINC_300)),
        "zinc_400" => Some(Color::from(ZINC_400)),
        "zinc_500" => Some(Color::from(ZINC_500)),
        "zinc_600" => Some(Color::from(ZINC_600)),
        "zinc_700" => Some(Color::from(ZINC_700)),
        "zinc_800" => Some(Color::from(ZINC_800)),
        "zinc_900" => Some(Color::from(ZINC_900)),
        "zinc_950" => Some(Color::from(ZINC_950)),
        _ => hex_color_parser(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_parser_invalid() {
        assert_eq!(None, hex_color_parser("#xyzzyx"));
        assert_eq!(None, hex_color_parser("testhexcolorparser"));
    }

    #[test]
    fn test_hex_color_parser() {
        let color_one = Color::srgba(0.8, 0.8, 0.8, 1.0);
        let color_two = Color::srgba(0.8, 0.4, 0.4, 1.0);

        assert_eq!(color_one, hex_color_parser("#cccccc").unwrap());
        assert_eq!(color_two, hex_color_parser("#cc6666").unwrap());
        assert_eq!(color_two, hex_color_parser("#cc6666ff").unwrap());
    }
}
