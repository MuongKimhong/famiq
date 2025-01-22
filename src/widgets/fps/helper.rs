use bevy::prelude::*;

// top left corner
pub fn default_fps_text_container_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        width: Val::Auto,
        height: Val::Auto,
        left: Val::Px(6.0),
        top: Val::Px(6.0),
        ..default()
    }
}
