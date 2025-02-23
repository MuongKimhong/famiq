use bevy::prelude::*;
use crate::widgets::color::*;
use super::*;

fn _get_progress_bar_size(size: &WidgetSize) -> f32 {
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

pub fn default_progress_bar_node(size: &WidgetSize) -> Node {
    Node {
        padding: UiRect::all(Val::Px(0.0)),
        margin: UiRect {
            top: Val::Px(2.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(2.0),
        },
        height: Val::Px(_get_progress_bar_size(size)),
        width: Val::Percent(100.0),
        border: UiRect::all(Val::Px(0.0)),
        ..default()
    }
}

pub fn default_progress_value_node(percentage: Option<f32>) -> Node {
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

pub fn get_progress_value_color(color: &WidgetColor) -> Color {
    let bg_color = match color {
        WidgetColor::Primary => PRIMARY_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_DARK_COLOR,
        WidgetColor::Secondary => SECONDARY_COLOR,
        WidgetColor::Success => SUCCESS_COLOR,
        WidgetColor::SuccessDark => SUCCESS_DARK_COLOR,
        WidgetColor::Danger => DANGER_COLOR,
        WidgetColor::DangerDark => DANGER_DARK_COLOR,
        WidgetColor::Warning => WARNING_COLOR,
        WidgetColor::WarningDark => WARNING_DARK_COLOR,
        WidgetColor::Info => INFO_COLOR,
        WidgetColor::InfoDark => INFO_DARK_COLOR,
        WidgetColor::Custom(color) => {
            if let Some(parsed_color) = built_in_color_parser(color) {
                parsed_color
            } else {
                DEFAULT_COLOR
            }
        },
        _ => DEFAULT_COLOR
    };
    bg_color
}
