use bevy::prelude::*;
use crate::widgets::text_input::TextInputSize;

pub const PLACEHOLDER_COLOR: Color = Color::srgba(0.651, 0.651, 0.651, 0.6);
pub const TEXT_INPUT_VALUE_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.922);

pub fn default_input_node() -> Node {
    Node {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(15.0),
            right: Val::Px(15.0),
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
        },
        margin: UiRect {
            top: Val::Px(5.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(5.0),
        },
        height: Val::Auto,
        width: Val::Percent(100.0),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

pub fn outlined_border_width() -> UiRect {
    UiRect::all(Val::Px(2.0))
}

pub fn underlined_border_width() -> UiRect {
    UiRect {
        left: Val::Px(0.0),
        right: Val::Px(0.0),
        top: Val::Px(0.0),
        bottom: Val::Px(2.0),
    }
}

pub fn outlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(6.0))
}

pub fn underlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn round_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Percent(50.0))
}

pub fn rectangle_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn get_text_size(size: &TextInputSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        TextInputSize::Small => size_small,
        TextInputSize::Normal => size_normal,
        TextInputSize::Large => size_large,
    };
    text_size
}
