use crate::utils;
use crate::widgets::{
    style_parse::*, DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetBuilderResource,
    FamiqWidgetId, StylesKeyValueResource, WidgetStyle,
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct ExternalStylesApplyState(pub bool);

type WidgetStyleQuery<'a, 'w, 's> = Query<
    'w,
    's,
    (
        &'a FamiqWidgetId,
        &'a mut Node,
        &'a mut BackgroundColor,
        &'a mut BorderColor,
        &'a mut BorderRadius,
        &'a mut ZIndex,
        &'a mut Visibility,
        &'a DefaultWidgetEntity,
    ),
>;

pub fn read_styles_from_file_system(
    mut styles: ResMut<StylesKeyValueResource>,
    builder_resource: ResMut<FamiqWidgetBuilderResource>,
    apply_state: Res<ExternalStylesApplyState>,
) {
    if builder_resource.hot_reload_styles || !apply_state.0 {
        if let Ok(s) = utils::read_styles_json_file(&builder_resource.style_path) {
            styles.0 = s;
        }
    }
}

pub fn finish_style_applying_system(mut apply_state: ResMut<ExternalStylesApplyState>) {
    apply_state.0 = true;
}

pub fn apply_widgets_styles_system(
    styles: Res<StylesKeyValueResource>,
    apply_state: Res<ExternalStylesApplyState>,
    builder_resource: ResMut<FamiqWidgetBuilderResource>,
    mut query: WidgetStyleQuery,
) {
    if builder_resource.hot_reload_styles || !apply_state.0 {
        for (
            widget_id,
            mut node,
            mut bg_color,
            mut border_color,
            mut border_radius,
            mut z_index,
            mut visibility,
            default_widget_entity,
        ) in query.iter_mut()
        {
            // assign default first before applying external style
            *bg_color = default_widget_entity.background_color.clone();
            *border_color = default_widget_entity.border_color.clone();
            *border_radius = default_widget_entity.border_radius.clone();
            *visibility = default_widget_entity.visibility.clone();
            *z_index = default_widget_entity.z_index.clone();
            *node = default_widget_entity.node.clone();

            if let Some(widget_style) = styles.get_style_by_id(&widget_id.0) {
                apply_styles_from_external_json(
                    &mut bg_color,
                    &mut border_color,
                    &mut border_radius,
                    &mut visibility,
                    &mut z_index,
                    &mut node,
                    &widget_style,
                );
            }
        }
    }
}

pub fn apply_styles_from_external_json(
    bg_color: &mut BackgroundColor,
    border_color: &mut BorderColor,
    border_radius: &mut BorderRadius,
    visibility: &mut Visibility,
    z_index: &mut ZIndex,
    node: &mut Node,
    widget_style: &WidgetStyle,
) {
    if let Some(bg_color_value) = &widget_style.background_color {
        if let Some(v) = parse_background_color(&bg_color_value) {
            *bg_color = v;
        }
    }

    if let Some(border_color_value) = &widget_style.border_color {
        if let Some(v) = parse_border_color(&border_color_value) {
            *border_color = v;
        }
    }

    if let Some(border_radius_value) = &widget_style.border_radius {
        if let Some(v) = parse_border_radius(&border_radius_value) {
            *border_radius = v;
        }
    }

    if let Some(border_radius_top_left) = &widget_style.border_radius_top_left {
        if let Some(v) = parse_val(border_radius_top_left) {
            border_radius.top_left = v;
        }
    }

    if let Some(border_radius_top_right) = &widget_style.border_radius_top_right {
        if let Some(v) = parse_val(border_radius_top_right) {
            border_radius.top_right = v;
        }
    }

    if let Some(border_radius_bottom_left) = &widget_style.border_radius_bottom_left {
        if let Some(v) = parse_val(border_radius_bottom_left) {
            border_radius.bottom_left = v;
        }
    }

    if let Some(border_radius_bottom_right) = &widget_style.border_radius_bottom_right {
        if let Some(v) = parse_val(border_radius_bottom_right) {
            border_radius.bottom_right = v;
        }
    }

    if let Some(visibility_value) = &widget_style.visibility {
        if let Some(v) = parse_visibility(&visibility_value) {
            *visibility = v;
        }
    }

    if let Some(z_index_value) = &widget_style.z_index {
        if let Some(v) = parse_z_index(&z_index_value) {
            *z_index = v;
        }
    }

    if let Some(display_value) = &widget_style.display {
        if let Some(v) = parse_display(&display_value) {
            node.display = v;
        }
    }

    if let Some(position_type_value) = &widget_style.position_type {
        if let Some(v) = parse_position_type(&position_type_value) {
            node.position_type = v;
        }
    }

    if let Some(overflow_x_value) = &widget_style.overflow_x {
        if let Some(v) = parse_overflow_x(&overflow_x_value) {
            node.overflow.x = v;
        }
    }

    if let Some(overflow_y_value) = &widget_style.overflow_y {
        if let Some(v) = parse_overflow_y(&overflow_y_value) {
            node.overflow.y = v;
        }
    }

    if let Some(left_value) = &widget_style.left {
        if let Some(v) = parse_val(&left_value) {
            node.left = v;
        }
    }

    if let Some(right_value) = &widget_style.right {
        if let Some(v) = parse_val(&right_value) {
            node.right = v;
        }
    }

    if let Some(top_value) = &widget_style.top {
        if let Some(v) = parse_val(&top_value) {
            node.top = v;
        }
    }

    if let Some(bottom_value) = &widget_style.bottom {
        if let Some(v) = parse_val(&bottom_value) {
            node.bottom = v;
        }
    }

    if let Some(width_value) = &widget_style.width {
        if let Some(v) = parse_val(&width_value) {
            node.width = v;
        }
    }

    if let Some(height_value) = &widget_style.height {
        if let Some(v) = parse_val(&height_value) {
            node.height = v;
        }
    }

    if let Some(min_width_value) = &widget_style.min_width {
        if let Some(v) = parse_val(&min_width_value) {
            node.min_width = v;
        }
    }

    if let Some(min_height_value) = &widget_style.min_height {
        if let Some(v) = parse_val(&min_height_value) {
            node.min_height = v;
        }
    }

    if let Some(max_width_value) = &widget_style.max_width {
        if let Some(v) = parse_val(&max_width_value) {
            node.max_width = v;
        }
    }

    if let Some(max_height_value) = &widget_style.max_height {
        if let Some(v) = parse_val(&max_height_value) {
            node.max_height = v;
        }
    }

    if let Some(align_items) = &widget_style.align_items {
        if let Some(v) = parse_align_items(&align_items) {
            node.align_items = v;
        }
    }

    if let Some(align_self) = &widget_style.align_self {
        if let Some(v) = parse_align_self(&align_self) {
            node.align_self = v;
        }
    }

    if let Some(justify_items) = &widget_style.justify_items {
        if let Some(v) = parse_justify_items(&justify_items) {
            node.justify_items = v;
        }
    }

    if let Some(justify_content) = &widget_style.justify_content {
        if let Some(v) = parse_justify_content(&justify_content) {
            node.justify_content = v;
        }
    }

    if let Some(padding) = &widget_style.padding {
        if let Some(v) = parse_ui_rect(&padding) {
            node.padding = v;
        }
    }

    if let Some(margin) = &widget_style.margin {
        if let Some(v) = parse_ui_rect(&margin) {
            node.margin = v;
        }
    }

    if let Some(border) = &widget_style.border {
        if let Some(v) = parse_ui_rect(&border) {
            node.border = v;
        }
    }

    if let Some(flex_direction) = &widget_style.flex_direction {
        if let Some(v) = parse_flex_direction(flex_direction) {
            node.flex_direction = v;
        }
    }

    if let Some(flex_wrap) = &widget_style.flex_wrap {
        if let Some(v) = parse_flex_wrap(flex_wrap) {
            node.flex_wrap = v;
        }
    }

    if let Some(flex_grow) = &widget_style.flex_grow {
        if let Ok(parsed_value) = flex_grow.trim().parse::<f32>() {
            node.flex_grow = parsed_value;
        }
    }

    if let Some(flex_shrink) = &widget_style.flex_shrink {
        if let Ok(parsed_value) = flex_shrink.trim().parse::<f32>() {
            node.flex_shrink = parsed_value;
        }
    }

    if let Some(flex_basis) = &widget_style.flex_basis {
        if let Some(v) = parse_val(flex_basis) {
            node.flex_basis = v;
        }
    }

    if let Some(row_gap) = &widget_style.row_gap {
        if let Some(v) = parse_val(row_gap) {
            node.row_gap = v;
        }
    }

    if let Some(column_gap) = &widget_style.column_gap {
        if let Some(v) = parse_val(column_gap) {
            node.column_gap = v;
        }
    }

    if let Some(grid_auto_flow) = &widget_style.grid_auto_flow {
        if let Some(v) = parse_grid_auto_flow(grid_auto_flow) {
            node.grid_auto_flow = v;
        }
    }

    if let Some(margin_left) = &widget_style.margin_left {
        if let Some(v) = parse_val(margin_left) {
            node.margin.left = v;
        }
    }

    if let Some(margin_right) = &widget_style.margin_right {
        if let Some(v) = parse_val(margin_right) {
            node.margin.right = v;
        }
    }

    if let Some(margin_top) = &widget_style.margin_top {
        if let Some(v) = parse_val(margin_top) {
            node.margin.top = v;
        }
    }

    if let Some(margin_bottom) = &widget_style.margin_bottom {
        if let Some(v) = parse_val(margin_bottom) {
            node.margin.bottom = v;
        }
    }

    if let Some(padding_left) = &widget_style.padding_left {
        if let Some(v) = parse_val(padding_left) {
            node.padding.left = v;
        }
    }

    if let Some(padding_right) = &widget_style.padding_right {
        if let Some(v) = parse_val(padding_right) {
            node.padding.right = v;
        }
    }

    if let Some(padding_top) = &widget_style.padding_top {
        if let Some(v) = parse_val(padding_top) {
            node.padding.top = v;
        }
    }

    if let Some(padding_bottom) = &widget_style.padding_bottom {
        if let Some(v) = parse_val(padding_bottom) {
            node.padding.bottom = v;
        }
    }

    if let Some(border_left) = &widget_style.border_left {
        if let Some(v) = parse_val(border_left) {
            node.border.left = v;
        }
    }

    if let Some(border_right) = &widget_style.border_right {
        if let Some(v) = parse_val(border_right) {
            node.border.right = v;
        }
    }

    if let Some(border_top) = &widget_style.border_top {
        if let Some(v) = parse_val(border_top) {
            node.border.top = v;
        }
    }

    if let Some(border_bottom) = &widget_style.border_bottom {
        if let Some(v) = parse_val(border_bottom) {
            node.border.bottom = v;
        }
    }
}

// for fa_text & Text only
pub fn apply_text_style_system(
    styles: Res<StylesKeyValueResource>,
    apply_state: Res<ExternalStylesApplyState>,
    builder_resource: ResMut<FamiqWidgetBuilderResource>,
    mut text_q: Query<(
        &mut TextFont,
        &mut TextColor,
        &FamiqWidgetId,
        &DefaultTextEntity
    )>,
) {
    if builder_resource.hot_reload_styles || !apply_state.0 {
        for (mut text_font, mut text_color, widget_id, default_text) in text_q.iter_mut() {
            // assign default values first
            *text_font = default_text.text_font.clone();
            *text_color = default_text.text_color.clone();

            if let Some(text_style) = styles.get_style_by_id(&widget_id.0) {
                // font size
                if let Some(font_size) = &text_style.font_size {
                    if let Ok(parsed_value) = font_size.trim().parse::<f32>() {
                        text_font.font_size = parsed_value;
                    }
                }

                // color
                if let Some(color) = &text_style.color {
                    if let Some(v) = parse_color(color) {
                        text_color.0 = v;
                    }
                }
            }
        }
    }
}
