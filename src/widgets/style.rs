use crate::utils;
use crate::widgets::{
    style_parse::*, DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetResource,
    FamiqWidgetId, FamiqWidgetClasses, StylesKeyValueResource, WidgetStyle,
    ExternalStyleHasChanged
};
use bevy::prelude::*;

type WidgetStyleQuery<'a, 'w, 's> = Query<
    'w,
    's,
    (
        Option<&'a FamiqWidgetId>,
        Option<&'a FamiqWidgetClasses>,
        &'a mut Node,
        &'a mut BackgroundColor,
        &'a mut BorderColor,
        &'a mut BorderRadius,
        &'a mut ZIndex,
        &'a mut Visibility,
        &'a WidgetStyle,
        &'a ExternalStyleHasChanged,
        &'a DefaultWidgetEntity,
    ),
>;

pub fn read_styles_from_file_system(
    mut styles: ResMut<StylesKeyValueResource>,
    builder_res: ResMut<FamiqWidgetResource>,
) {
    if builder_res.hot_reload_styles || !builder_res.external_style_applied {
        if let Ok(s) = utils::read_styles_json_file(&builder_res.style_path) {
            styles.0 = s;
        }
    }
}

pub fn detect_external_style_changes(
    styles: Res<StylesKeyValueResource>,
    mut query: Query<(
        Option<&FamiqWidgetId>,
        Option<&FamiqWidgetClasses>,
        &mut ExternalStyleHasChanged,
        &mut WidgetStyle
    )>
) {
    for (id, class, mut has_changed, mut widget_style) in query.iter_mut() {
        if let Some(id) = id {
            if let Some(external_style) = styles.get_style_by_id(&id.0) {
                has_changed.0 = widget_style.update_from(external_style);
            }
        }

        if let Some(classes) = class {
            let classes_split: Vec<&str> = classes.0.split_whitespace().collect();
            for class_name in classes_split {
                if let Some(external_style) = styles.get_style_by_class_name(class_name) {
                    has_changed.0 = widget_style.update_from(external_style);
                }
            }
        }
    }
}

pub fn inject_external_style(
    styles: Res<StylesKeyValueResource>,
    mut query: Query<(
        Option<&FamiqWidgetId>,
        Option<&FamiqWidgetClasses>,
        &mut WidgetStyle
    )>
) {
    for (id, class, mut widget_style) in query.iter_mut() {
        if let Some(id) = id {
            if let Some(external_style) = styles.get_style_by_id(&id.0) {
                widget_style.from_external(external_style);
            }
        }

        if let Some(classes) = class {
            let classes_split: Vec<&str> = classes.0.split_whitespace().collect();
            for class_name in classes_split {
                if let Some(external_style) = styles.get_style_by_class_name(class_name) {
                    widget_style.from_external(external_style);
                }
            }
        }
    }
}

pub fn finish_style_applying_system(mut builder_res: ResMut<FamiqWidgetResource>) {
    builder_res.external_style_applied = true;
}

pub fn apply_widgets_styles_system(
    builder_res: Res<FamiqWidgetResource>,
    mut query: WidgetStyleQuery,
) {
    for (
        widget_id,
        widget_classes,
        mut node,
        mut bg_color,
        mut border_color,
        mut border_radius,
        mut z_index,
        mut visibility,
        local_widget_style,
        has_external_changed,
        default_widget_entity,
    ) in query.iter_mut()
    {
        if builder_res.hot_reload_styles {
            if has_external_changed.0 && (widget_id.is_some() || widget_classes.is_some()) {
                apply_styles_from_external_json(
                    &mut bg_color,
                    &mut border_color,
                    &mut border_radius,
                    &mut visibility,
                    &mut z_index,
                    &mut node,
                    local_widget_style,
                    default_widget_entity
                );
            }
        }

        if !builder_res.hot_reload_styles && !builder_res.external_style_applied {
            if widget_id.is_some() || widget_classes.is_some() {
                apply_styles_from_external_json(
                    &mut bg_color,
                    &mut border_color,
                    &mut border_radius,
                    &mut visibility,
                    &mut z_index,
                    &mut node,
                    local_widget_style,
                    default_widget_entity
                );
            }
        }
    }
}

pub fn apply_text_styles_from_external_json(
    local_style: &WidgetStyle,
    default_text_entity: &DefaultTextEntity,
    text_font: &mut TextFont,
    text_color: &mut TextColor,
) {
    if let Some(font_size) = &local_style.font_size {
        if let Ok(parsed_value) = font_size.trim().parse::<f32>() {
            text_font.font_size = parsed_value;
        }
    } else {
        text_font.font_size = default_text_entity.text_font.font_size.clone();
    }

    if let Some(color) = &local_style.color {
        if let Some(v) = parse_color(color) {
            text_color.0 = v;
        }
    } else {
        text_color.0 = default_text_entity.text_color.0.clone();
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
    default_widget_entity: &DefaultWidgetEntity
) {
    if let Some(bg_color_value) = &widget_style.background_color {
        if let Some(v) = parse_background_color(&bg_color_value) {
            *bg_color = v;
        }
    } else {
        *bg_color = default_widget_entity.background_color.clone();
    }


    if let Some(border_color_value) = &widget_style.border_color {
        if let Some(v) = parse_border_color(&border_color_value) {
            *border_color = v;
        }
    } else {
        *border_color = default_widget_entity.border_color.clone();
    }

    if let Some(border_radius_value) = &widget_style.border_radius {
        if let Some(v) = parse_border_radius(&border_radius_value) {
            *border_radius = v;
        }
    } else {
        *border_radius = default_widget_entity.border_radius.clone();
    }

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

    if let Some(visibility_value) = &widget_style.visibility {
        if let Some(v) = parse_visibility(&visibility_value) {
            *visibility = v;
        }
    } else {
        *visibility = default_widget_entity.visibility.clone();
    }

    if let Some(z_index_value) = &widget_style.z_index {
        if let Some(v) = parse_z_index(&z_index_value) {
            *z_index = v;
        }
    } else {
        *z_index = default_widget_entity.z_index.clone();
    }

    if let Some(display_value) = &widget_style.display {
        if let Some(v) = parse_display(&display_value) {
            node.display = v;
        }
    } else {
        node.display = default_widget_entity.node.display.clone();
    }

    if let Some(position_type_value) = &widget_style.position_type {
        if let Some(v) = parse_position_type(&position_type_value) {
            node.position_type = v;
        }
    } else {
        node.position_type = default_widget_entity.node.position_type.clone();
    }

    if let Some(overflow_x_value) = &widget_style.overflow_x {
        if let Some(v) = parse_overflow_x(&overflow_x_value) {
            node.overflow.x = v;
        }
    } else {
        node.overflow.x = default_widget_entity.node.overflow.x.clone();
    }

    if let Some(overflow_y_value) = &widget_style.overflow_y {
        if let Some(v) = parse_overflow_y(&overflow_y_value) {
            node.overflow.y = v;
        }
    } else {
        node.overflow.y = default_widget_entity.node.overflow.y.clone();
    }

    if let Some(left_value) = &widget_style.left {
        if let Some(v) = parse_val(&left_value) {
            node.left = v;
        }
    } else {
        node.left = default_widget_entity.node.left.clone();
    }

    if let Some(right_value) = &widget_style.right {
        if let Some(v) = parse_val(&right_value) {
            node.right = v;
        }
    } else {
        node.right = default_widget_entity.node.right.clone();
    }

    if let Some(top_value) = &widget_style.top {
        if let Some(v) = parse_val(&top_value) {
            node.top = v;
        }
    } else {
        node.top = default_widget_entity.node.top.clone();
    }

    if let Some(bottom_value) = &widget_style.bottom {
        if let Some(v) = parse_val(&bottom_value) {
            node.bottom = v;
        }
    } else {
        node.bottom = default_widget_entity.node.bottom.clone();
    }

    if let Some(width_value) = &widget_style.width {
        if let Some(v) = parse_val(&width_value) {
            node.width = v;
        }
    } else {
        node.width = default_widget_entity.node.width.clone();
    }

    if let Some(height_value) = &widget_style.height {
        if let Some(v) = parse_val(&height_value) {
            node.height = v;
        }
    } else {
        node.height = default_widget_entity.node.height.clone();
    }

    if let Some(min_width_value) = &widget_style.min_width {
        if let Some(v) = parse_val(&min_width_value) {
            node.min_width = v;
        }
    } else {
        node.min_width = default_widget_entity.node.min_width.clone();
    }

    if let Some(min_height_value) = &widget_style.min_height {
        if let Some(v) = parse_val(&min_height_value) {
            node.min_height = v;
        }
    } else {
        node.min_height = default_widget_entity.node.min_height.clone();
    }

    if let Some(max_width_value) = &widget_style.max_width {
        if let Some(v) = parse_val(&max_width_value) {
            node.max_width = v;
        }
    } else {
        node.max_width = default_widget_entity.node.max_width.clone();
    }

    if let Some(max_height_value) = &widget_style.max_height {
        if let Some(v) = parse_val(&max_height_value) {
            node.max_height = v;
        }
    } else {
        node.max_height = default_widget_entity.node.max_height.clone();
    }

    if let Some(align_items) = &widget_style.align_items {
        if let Some(v) = parse_align_items(&align_items) {
            node.align_items = v;
        }
    } else {
        node.align_items = default_widget_entity.node.align_items.clone();
    }

    if let Some(align_self) = &widget_style.align_self {
        if let Some(v) = parse_align_self(&align_self) {
            node.align_self = v;
        }
    } else {
        node.align_self = default_widget_entity.node.align_self.clone();
    }

    if let Some(justify_items) = &widget_style.justify_items {
        if let Some(v) = parse_justify_items(&justify_items) {
            node.justify_items = v;
        }
    } else {
        node.justify_items = default_widget_entity.node.justify_items.clone();
    }

    if let Some(justify_content) = &widget_style.justify_content {
        if let Some(v) = parse_justify_content(&justify_content) {
            node.justify_content = v;
        }
    } else {
        node.justify_content = default_widget_entity.node.justify_content.clone();
    }

    if let Some(padding) = &widget_style.padding {
        if let Some(v) = parse_ui_rect(&padding) {
            node.padding = v;
        }
    } else {
        node.padding = default_widget_entity.node.padding.clone();
    }

    if let Some(margin) = &widget_style.margin {
        if let Some(v) = parse_ui_rect(&margin) {
            node.margin = v;
        }
    } else {
        node.margin = default_widget_entity.node.margin.clone();
    }

    if let Some(border) = &widget_style.border {
        if let Some(v) = parse_ui_rect(&border) {
            node.border = v;
        }
    } else {
        node.border = default_widget_entity.node.border.clone();
    }

    if let Some(flex_direction) = &widget_style.flex_direction {
        if let Some(v) = parse_flex_direction(flex_direction) {
            node.flex_direction = v;
        }
    } else {
        node.flex_direction = default_widget_entity.node.flex_direction.clone();
    }

    if let Some(flex_wrap) = &widget_style.flex_wrap {
        if let Some(v) = parse_flex_wrap(flex_wrap) {
            node.flex_wrap = v;
        }
    } else {
        node.flex_wrap = default_widget_entity.node.flex_wrap.clone();
    }

    if let Some(flex_grow) = &widget_style.flex_grow {
        if let Ok(parsed_value) = flex_grow.trim().parse::<f32>() {
            node.flex_grow = parsed_value;
        }
    } else {
        node.flex_grow = default_widget_entity.node.flex_grow.clone();
    }

    if let Some(flex_shrink) = &widget_style.flex_shrink {
        if let Ok(parsed_value) = flex_shrink.trim().parse::<f32>() {
            node.flex_shrink = parsed_value;
        }
    } else {
        node.flex_shrink = default_widget_entity.node.flex_shrink.clone();
    }

    if let Some(flex_basis) = &widget_style.flex_basis {
        if let Some(v) = parse_val(flex_basis) {
            node.flex_basis = v;
        }
    } else {
        node.flex_basis = default_widget_entity.node.flex_basis.clone();
    }

    if let Some(row_gap) = &widget_style.row_gap {
        if let Some(v) = parse_val(row_gap) {
            node.row_gap = v;
        }
    } else {
        node.row_gap = default_widget_entity.node.row_gap.clone();
    }

    if let Some(column_gap) = &widget_style.column_gap {
        if let Some(v) = parse_val(column_gap) {
            node.column_gap = v;
        }
    } else {
        node.column_gap = default_widget_entity.node.column_gap.clone();
    }

    if let Some(grid_auto_flow) = &widget_style.grid_auto_flow {
        if let Some(v) = parse_grid_auto_flow(grid_auto_flow) {
            node.grid_auto_flow = v;
        }
    } else {
        node.grid_auto_flow = default_widget_entity.node.grid_auto_flow.clone();
    }

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

// for fa_text & Text only
pub fn apply_text_style_system(
    builder_res: Res<FamiqWidgetResource>,
    mut text_q: Query<(
        &mut TextFont,
        &mut TextColor,
        Option<&FamiqWidgetId>,
        Option<&FamiqWidgetClasses>,
        &WidgetStyle,
        &ExternalStyleHasChanged,
        &DefaultTextEntity
    )>,
) {
    for (
        mut text_font,
        mut text_color,
        widget_id,
        widget_classes,
        local_widget_style,
        has_external_changed,
        default_text_entity
    )
    in text_q.iter_mut() {

        if builder_res.hot_reload_styles {
            if has_external_changed.0 && (widget_id.is_some() || widget_classes.is_some()) {
                // if let Some(font_size) = &local_widget_style.font_size {
                //     if let Ok(parsed_value) = font_size.trim().parse::<f32>() {
                //         text_font.font_size = parsed_value;
                //     }
                // } else {
                //     text_font.font_size = default_text_entity.text_font.font_size.clone();
                // }

                // if let Some(color) = &local_widget_style.color {
                //     if let Some(v) = parse_color(color) {
                //         text_color.0 = v;
                //     }
                // } else {
                //     text_color.0 = default_text_entity.text_color.0.clone();
                // }
                apply_text_styles_from_external_json(
                    local_widget_style,
                    default_text_entity,
                    &mut text_font,
                    &mut text_color
                );
            }
        }

        if !builder_res.hot_reload_styles && !builder_res.external_style_applied {
            if widget_id.is_some() || widget_classes.is_some() {
                apply_text_styles_from_external_json(
                    local_widget_style,
                    default_text_entity,
                    &mut text_font,
                    &mut text_color
                );
            }
        }
    }
}
