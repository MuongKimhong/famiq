#![cfg(test)]

use bevy::prelude::*;
use super::*;

#[test]
fn test_widget_style_from_external() {
    let mut local_style = WidgetStyle {
        color: Some("red".to_string()),
        ..default()
    };

    let external_style = WidgetStyle {
        color: Some("blue".to_string()),
        background_color: Some("yellow".to_string()),
        ..default()
    };

    // Update the local style with the external style
    local_style.from_external(&external_style);

    assert_eq!(
        local_style.color,
        Some("blue".to_string())
    );
    assert_eq!(
        local_style.background_color,
        Some("yellow".to_string()),
    );
}

#[test]
fn test_widget_style_update_from() {
    let mut local_style = WidgetStyle {
        color: Some("red".to_string()),
        font_size: None,
        background_color: Some("white".to_string()),
        ..default()
    };

    let external_style = WidgetStyle {
        color: Some("blue".to_string()),
        font_size: Some("16px".to_string()),
        background_color: None,
        ..default()
    };

    // Update the local style with the external style
    local_style.update_from(&external_style);

    assert_eq!(
        local_style.color,
        Some("blue".to_string())
    );
    assert_eq!(
        local_style.font_size,
        Some("16px".to_string())
    );
    assert_eq!(
        local_style.background_color,
        None,
    );
}
