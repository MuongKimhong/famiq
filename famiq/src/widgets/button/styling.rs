use bevy::prelude::*;

pub fn default_button_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(0.0)),
        padding: UiRect {
            left: Val::Px(8.0),
            right: Val::Px(8.0),
            top: Val::Px(5.0),
            bottom: Val::Px(5.0)
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
