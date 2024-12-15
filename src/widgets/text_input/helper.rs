use crate::widgets::text_input::*;
use crate::widgets::FaWidgetBundle;
use bevy::prelude::*;

fn set_text_input_default_size(
    size: &Option<TextInputSize>,
    // text_bundle: &TextBundle,
    text_input_bundle: &mut FaWidgetBundle,
) {
    let width = Val::Percent(100.0);
    let size_small_height = Val::Px(28.0);
    let size_normal_height = Val::Px(34.0);
    let size_large_height = Val::Px(40.0);

    let height = match size {
        Some(TextInputSize::Small) => size_small_height,
        Some(TextInputSize::Normal) => size_normal_height,
        Some(TextInputSize::Large) => size_large_height,
        None => size_normal_height,
    };

    text_input_bundle.style.height = height;
    text_input_bundle.style.width = width;
}

fn get_default_text_input_style(border_width: UiRect) -> Style {
    Style {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        border: border_width,
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            top: Val::Px(1.0),
            bottom: Val::Px(1.0),
        },
        ..default()
    }
}

pub fn default_text_input_bundle(
    border_width: UiRect,
    border_radius: BorderRadius,
    size: &Option<TextInputSize>,
) -> FaWidgetBundle {
    let mut input_bundle = FaWidgetBundle {
        style: get_default_text_input_style(border_width),
        border_radius,
        border_color: BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922)),
        ..default()
    };
    set_text_input_default_size(size, &mut input_bundle);
    input_bundle
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
    BorderRadius::all(Val::Px(5.0))
}

pub fn underlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}
