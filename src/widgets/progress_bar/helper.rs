use bevy::prelude::*;
use crate::widgets::color::*;
use super::{ProgressBarColor, ProgressBarSize, INDETERMINATE_WIDTH};

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
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

pub fn default_progress_value_node(percentage: Option<f32>) -> Node {
    let mut node = Node {
        width: Val::Percent(INDETERMINATE_WIDTH),
        height: Val::Percent(100.0),
        left: Val::Px(0.0),
        border: UiRect::all(Val::Px(0.0)),
        margin: UiRect::all(Val::Px(0.0)),
        ..default()
    };

    if let Some(percentage) = percentage {
        node.width = Val::Percent(percentage);
    }
    node
}

pub fn get_progress_value_background_color(color: &ProgressBarColor) -> BackgroundColor {
    let bg_color: BackgroundColor = match color {
        ProgressBarColor::Default => BUTTON_DEFAULT_COLOR.into(),
        ProgressBarColor::Primary => PRIMARY_COLOR.into(),
        ProgressBarColor::PrimaryDark => PRIMARY_DARK_COLOR.into(),
        ProgressBarColor::Secondary => SECONDARY_COLOR.into(),
        ProgressBarColor::Success => SUCCESS_COLOR.into(),
        ProgressBarColor::SuccessDark => SUCCESS_DARK_COLOR.into(),
        ProgressBarColor::Danger => DANGER_COLOR.into(),
        ProgressBarColor::DangerDark => DANGER_DARK_COLOR.into(),
        ProgressBarColor::Warning => WARNING_COLOR.into(),
        ProgressBarColor::WarningDark => WARNING_DARK_COLOR.into(),
        ProgressBarColor::Info => INFO_COLOR.into(),
        ProgressBarColor::InfoDark => INFO_DARK_COLOR.into(),
    };
    bg_color
}

pub fn get_progress_value_border_color(color: &ProgressBarColor) -> BorderColor {
    let bd_color: BorderColor = match color {
        ProgressBarColor::Default => BUTTON_DEFAULT_COLOR.into(),
        ProgressBarColor::Primary => PRIMARY_COLOR.into(),
        ProgressBarColor::PrimaryDark => PRIMARY_DARK_COLOR.into(),
        ProgressBarColor::Secondary => SECONDARY_COLOR.into(),
        ProgressBarColor::Success => SUCCESS_COLOR.into(),
        ProgressBarColor::SuccessDark => SUCCESS_DARK_COLOR.into(),
        ProgressBarColor::Danger => DANGER_COLOR.into(),
        ProgressBarColor::DangerDark => DANGER_DARK_COLOR.into(),
        ProgressBarColor::Warning => WARNING_COLOR.into(),
        ProgressBarColor::WarningDark => WARNING_DARK_COLOR.into(),
        ProgressBarColor::Info => INFO_COLOR.into(),
        ProgressBarColor::InfoDark => INFO_DARK_COLOR.into(),
    };
    bd_color
}
