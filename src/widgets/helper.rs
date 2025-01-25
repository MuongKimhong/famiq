use bevy::prelude::*;
use super::style_parse::*;
use super::{WidgetStyle, DefaultWidgetEntity};

pub fn handle_apply_margin(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetEntity,
    node: &mut Node
) {
    if let Some(margin) = &widget_style.margin {
        // If the full margin is defined, set it and skip individual sub-properties
        if let Some(v) = parse_ui_rect(&margin) {
            node.margin = v;
        }
    } else {
        // If the full margin is not defined, check sub-properties individually
        if let Some(margin_left) = &widget_style.margin_left {
            if let Some(v) = parse_val(margin_left) {
                node.margin.left = v;
            }
        } else {
            node.margin.left = default_widget_entity.node.margin.left.clone();
        }

        if let Some(margin_right) = &widget_style.margin_right {
            if let Some(v) = parse_val(margin_right) {
                node.margin.right = v;
            }
        } else {
            node.margin.right = default_widget_entity.node.margin.right.clone();
        }

        if let Some(margin_top) = &widget_style.margin_top {
            if let Some(v) = parse_val(margin_top) {
                node.margin.top = v;
            }
        } else {
            node.margin.top = default_widget_entity.node.margin.top.clone();
        }

        if let Some(margin_bottom) = &widget_style.margin_bottom {
            if let Some(v) = parse_val(margin_bottom) {
                node.margin.bottom = v;
            }
        } else {
            node.margin.bottom = default_widget_entity.node.margin.bottom.clone();
        }
    }
}

pub fn handle_apply_padding(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetEntity,
    node: &mut Node
) {
    if let Some(padding) = &widget_style.padding {
        // If the full padding is defined, set it and skip individual sub-properties
        if let Some(v) = parse_ui_rect(&padding) {
            node.padding = v;
        }
    } else {
        // If the full padding is not defined, check sub-properties individually
        if let Some(padding_left) = &widget_style.padding_left {
            if let Some(v) = parse_val(padding_left) {
                node.padding.left = v;
            }
        } else {
            node.padding.left = default_widget_entity.node.padding.left.clone();
        }

        if let Some(padding_right) = &widget_style.padding_right {
            if let Some(v) = parse_val(padding_right) {
                node.padding.right = v;
            }
        } else {
            node.padding.right = default_widget_entity.node.padding.right.clone();
        }

        if let Some(padding_top) = &widget_style.padding_top {
            if let Some(v) = parse_val(padding_top) {
                node.padding.top = v;
            }
        } else {
            node.padding.top = default_widget_entity.node.padding.top.clone();
        }

        if let Some(padding_bottom) = &widget_style.padding_bottom {
            if let Some(v) = parse_val(padding_bottom) {
                node.padding.bottom = v;
            }
        } else {
            node.padding.bottom = default_widget_entity.node.padding.bottom.clone();
        }
    }
}

pub fn handle_apply_border(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetEntity,
    node: &mut Node
) {
    if let Some(border) = &widget_style.border {
        // If the full border is defined, set it and skip individual sub-properties
        if let Some(v) = parse_ui_rect(&border) {
            node.border = v;
        }
    } else {
        // If the full border is not defined, check sub-properties individually
        if let Some(border_left) = &widget_style.border_left {
            if let Some(v) = parse_val(border_left) {
                node.border.left = v;
            }
        } else {
            node.border.left = default_widget_entity.node.border.left.clone();
        }

        if let Some(border_right) = &widget_style.border_right {
            if let Some(v) = parse_val(border_right) {
                node.border.right = v;
            }
        } else {
            node.border.right = default_widget_entity.node.border.right.clone();
        }

        if let Some(border_top) = &widget_style.border_top {
            if let Some(v) = parse_val(border_top) {
                node.border.top = v;
            }
        } else {
            node.border.top = default_widget_entity.node.border.top.clone();
        }

        if let Some(border_bottom) = &widget_style.border_bottom {
            if let Some(v) = parse_val(border_bottom) {
                node.border.bottom = v;
            }
        } else {
            node.border.bottom = default_widget_entity.node.border.bottom.clone();
        }
    }
}

pub fn handle_apply_border_radius(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetEntity,
    border_radius: &mut BorderRadius
) {
    // check for full border radius
    if let Some(border_radius_value) = &widget_style.border_radius {
        if let Some(v) = parse_border_radius(&border_radius_value) {
            *border_radius = v;
        }
    } else {
        // If the full border radius is not defined, check sub-properties individually
        if let Some(border_radius_top_left) = &widget_style.border_radius_top_left {
            if let Some(v) = parse_val(border_radius_top_left) {
                border_radius.top_left = v;
            }
        } else {
            border_radius.top_left = default_widget_entity.border_radius.top_left.clone();
        }

        if let Some(border_radius_top_right) = &widget_style.border_radius_top_right {
            if let Some(v) = parse_val(border_radius_top_right) {
                border_radius.top_right = v;
            }
        } else {
            border_radius.top_right = default_widget_entity.border_radius.top_right.clone();
        }

        if let Some(border_radius_bottom_left) = &widget_style.border_radius_bottom_left {
            if let Some(v) = parse_val(border_radius_bottom_left) {
                border_radius.bottom_left = v;
            }
        } else {
            border_radius.bottom_left = default_widget_entity.border_radius.bottom_left.clone();
        }

        if let Some(border_radius_bottom_right) = &widget_style.border_radius_bottom_right {
            if let Some(v) = parse_val(border_radius_bottom_right) {
                border_radius.bottom_right = v;
            }
        } else {
            border_radius.bottom_right = default_widget_entity.border_radius.bottom_right.clone();
        }
    }
}
