use bevy::prelude::*;

pub fn default_input_node() -> Node {
    Node {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            top: Val::Px(4.0),
            bottom: Val::Px(4.0),
        },
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        height: Val::Auto,
        width: Val::Percent(100.0),
        border: UiRect::all(Val::Px(2.0)),
        overflow: Overflow::scroll_x(),
        ..default()
    }
}
