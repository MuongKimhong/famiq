use bevy::prelude::*;

pub fn default_container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart, // Align children at the top
        height: Val::Auto,
        border: UiRect::all(Val::Px(10.)),
        ..default()
    }
}
