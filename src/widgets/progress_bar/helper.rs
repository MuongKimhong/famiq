use bevy::prelude::*;
use crate::widgets::color::*;
use super::{ProgressBarColor, ProgressBarSize};

fn _get_progress_bar_size(size: &ProgressBarSize) -> f32 {
    let size_small = 5.0;
    let size_normal = 10.0;
    let size_large = 15.0;

    match size {
        ProgressBarSize::Small => size_small,
        ProgressBarSize::Normal => size_normal,
        ProgressBarSize::Large => size_large,
    }
}

pub fn default_progress_bar_node(size: &ProgressBarSize) -> Node {
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

pub fn get_progress_value_color(color: &ProgressBarColor) -> Color {
    let bg_color = match color {
        ProgressBarColor::Default => BUTTON_DEFAULT_COLOR,
        ProgressBarColor::Primary => PRIMARY_COLOR,
        ProgressBarColor::PrimaryDark => PRIMARY_DARK_COLOR,
        ProgressBarColor::Secondary => SECONDARY_COLOR,
        ProgressBarColor::Success => SUCCESS_COLOR,
        ProgressBarColor::SuccessDark => SUCCESS_DARK_COLOR,
        ProgressBarColor::Danger => DANGER_COLOR,
        ProgressBarColor::DangerDark => DANGER_DARK_COLOR,
        ProgressBarColor::Warning => WARNING_COLOR,
        ProgressBarColor::WarningDark => WARNING_DARK_COLOR,
        ProgressBarColor::Info => INFO_COLOR,
        ProgressBarColor::InfoDark => INFO_DARK_COLOR,
    };
    bg_color
}
