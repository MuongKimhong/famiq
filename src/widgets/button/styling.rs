use crate::widgets::button::*;
use bevy::prelude::*;

pub fn get_text_size(size: &WidgetSize) -> f32 {
    let size_small = 14.0;
    let size_normal = 18.0;
    let size_large = 22.0;

    match size {
        WidgetSize::Small => size_small,
        WidgetSize::Large => size_large,
        _ => size_normal
    }
}

pub fn default_button_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(0.0)),
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            top: Val::Px(2.0),
            bottom: Val::Px(2.0)
        },
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        ..default()
    }
}
