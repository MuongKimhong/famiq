use bevy::prelude::*;
use super::*;

fn get_progress_bar_size(size: &WidgetSize) -> f32 {
    let size_small = 8.0;
    let size_normal = 12.0;
    let size_large = 15.0;

    match size {
        WidgetSize::Small => size_small,
        WidgetSize::Large => size_large,
        WidgetSize::Custom(v) => *v,
        _ => size_normal
    }
}

pub(crate) fn default_progress_bar_node(size: &WidgetSize) -> Node {
    Node {
        padding: UiRect::all(Val::Px(0.0)),
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        height: Val::Px(get_progress_bar_size(size)),
        width: Val::Percent(100.0),
        border: UiRect::all(Val::Px(0.0)),
        ..default()
    }
}

pub(crate) fn default_progress_value_node(percentage: Option<f32>) -> Node {
    let mut node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        border: UiRect::all(Val::Px(0.0)),
        margin: UiRect::all(Val::Px(0.0)),
        ..default()
    };

    if let Some(percentage) = percentage {
        node.width = Val::Percent(percentage);
    }
    node
}
