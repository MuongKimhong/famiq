use bevy::prelude::*;

pub fn default_main_container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Auto,
        flex_wrap: FlexWrap::Wrap,
        flex_direction: FlexDirection::Row,
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        ..default()
    }
}

pub fn default_choice_box_node() -> Node {
    Node {
        width: Val::Px(13.0),
        height: Val::Px(13.0),
        border: UiRect::all(Val::Px(1.0)),
        margin: UiRect {
            right: Val::Px(4.0),
            top: Val::Px(3.0),
            ..default()
        },
        ..default()
    }
}

pub fn default_choice_container_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        flex_direction: FlexDirection::Row,
        margin: UiRect {
            top: Val::Px(0.0),
            right: Val::Px(4.0),
            left: Val::Px(4.0),
            bottom: Val::Px(0.0),
        },
        ..default()
    }
}
