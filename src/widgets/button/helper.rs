use crate::widgets::{button::*, color::*, FaWidgetBundle};
use bevy::prelude::*;

const DEFAULT_BUTTON_BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(5.0));

pub fn set_button_background_color(
    variant: &Option<BtnVariant>,
    button_bundle: &mut FaWidgetBundle,
) {
    let bg_color: BackgroundColor = match variant {
        Some(BtnVariant::Default) => BackgroundColor(BUTTON_DEFAULT_COLOR),
        Some(BtnVariant::Primary) => BackgroundColor(PRIMARY_COLOR),
        Some(BtnVariant::Secondary) => BackgroundColor(SECONDARY_COLOR),
        Some(BtnVariant::Success) => BackgroundColor(SUCCESS_COLOR),
        Some(BtnVariant::Danger) => BackgroundColor(DANGER_COLOR),
        Some(BtnVariant::Warning) => BackgroundColor(WARNING_COLOR),
        Some(BtnVariant::Info) => BackgroundColor(INFO_COLOR),
        None => BackgroundColor(BUTTON_DEFAULT_COLOR),
    };
    button_bundle.background_color = bg_color;
}

pub fn set_button_border_color(variant: &Option<BtnVariant>, button_bundle: &mut FaWidgetBundle) {
    let border_color: BorderColor = match variant {
        Some(BtnVariant::Default) => BorderColor(BUTTON_DEFAULT_COLOR),
        Some(BtnVariant::Primary) => BorderColor(PRIMARY_COLOR),
        Some(BtnVariant::Secondary) => BorderColor(SECONDARY_COLOR),
        Some(BtnVariant::Success) => BorderColor(SUCCESS_COLOR),
        Some(BtnVariant::Danger) => BorderColor(DANGER_COLOR),
        Some(BtnVariant::Warning) => BorderColor(WARNING_COLOR),
        Some(BtnVariant::Info) => BorderColor(INFO_COLOR),
        None => BorderColor(BUTTON_DEFAULT_COLOR),
    };
    button_bundle.border_color = border_color;
}

pub fn set_button_size(
    size: &Option<BtnSize>,
    text_bundle: &TextBundle,
    button_bundle: &mut FaWidgetBundle,
) {
    let size_small = (Val::Px(24.0), Val::Px(2.0));
    let size_normal = (Val::Px(30.0), Val::Px(5.0));
    let size_large = (Val::Px(36.0), Val::Px(5.0));

    let (height, border_width) = match size {
        Some(BtnSize::Small) => size_small,
        Some(BtnSize::Normal) => size_normal,
        Some(BtnSize::Large) => size_large,
        None => size_normal,
    };

    button_bundle.style.height = height;
    button_bundle.style.width = text_bundle.style.width;
    button_bundle.style.border = UiRect::all(border_width);
}

fn get_default_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn create_default_button_bundle(
    variant: &Option<BtnVariant>,
    size: &Option<BtnSize>,
    text_bundle: &TextBundle,
) -> FaWidgetBundle {
    let mut button_bundle = FaWidgetBundle {
        style: get_default_button_style(),
        border_radius: DEFAULT_BUTTON_BORDER_RADIUS,
        ..default()
    };
    set_button_size(&size, &text_bundle, &mut button_bundle);
    set_button_border_color(&variant, &mut button_bundle);
    set_button_background_color(&variant, &mut button_bundle);
    button_bundle
}
