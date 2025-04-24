use crate::utils;
use crate::resources::*;
use crate::widgets::{style_parse::*, *};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;

use super::text_input::CosmicTextData;
use super::DefaultTextSpanConfig;

pub type WidgetStyleQuery<'a, 'w, 's> = Query<
    'w,
    's,
    (
        Option<&'a WidgetId>,
        Option<&'a WidgetClasses>,
        &'a mut Node,
        &'a mut BackgroundColor,
        &'a mut BorderColor,
        &'a mut BorderRadius,
        &'a mut ZIndex,
        &'a mut Visibility,
        &'a mut BoxShadow,
        &'a DefaultWidgetConfig,
    ),
>;

pub(crate) fn read_styles_from_file_system(
    mut styles: ResMut<StylesKeyValueResource>,
    famiq_res: ResMut<FamiqResource>,
) {
    if let Ok(json_styles) = utils::read_styles_json_file(&famiq_res.style_path) {
        let mut changed_keys: Vec<String> = Vec::new();

        for (external_key, external_value) in json_styles.iter() {

            if styles.values.get(external_key).is_none() {
                styles.values.insert(external_key.to_owned(), external_value.to_owned());
                changed_keys.push(external_key.to_owned());
                continue;
            }

            if let Some(internal_value) = styles.values.get(external_key) {
                if internal_value == external_value {
                    continue;
                }
                styles.values.insert(external_key.to_owned(), external_value.to_owned());
                changed_keys.push(external_key.to_owned());
            }
        }

        // Find keys that exist in `styles.values` but are missing from `json_styles`
        let keys_to_remove: Vec<String> = styles
            .values
            .keys()
            .filter(|key| !json_styles.contains_key(*key))
            .cloned()
            .collect();

        // Remove missing keys
        for key in keys_to_remove.iter() {
            styles.values.remove(key);
        }
        changed_keys.extend(keys_to_remove);

        if !changed_keys.is_empty() {
            styles.changed_keys = changed_keys;
        }
    }
}

pub(crate) fn detect_widget_external_styles_change(
    styles: Res<StylesKeyValueResource>,
    mut style_q: Query<StyleQuery>,
) {
    if styles.is_changed() {
        use std::time::Instant;
        let now = Instant::now();

        let changed_keys = &styles.changed_keys;

        style_q.par_iter_mut().for_each(|mut style| {
            let mut formatted = String::with_capacity(64);

            // quick exit if there's no id or class
            if style.id.is_none() && style.class.is_none() {
                return;
            }

            let mut _class_split: HashSet<&str> = HashSet::new();
            let mut changed = false;
            let mut empty_style = WidgetStyle::default();

            // Check id
            let id_match = style.id.as_ref().map_or(false, |id| changed_keys.contains(&id.0));

            // Check class match
            let class_match = if let Some(classes) = style.class {
                _class_split = classes.0.split_whitespace().collect();
                _class_split.iter().any(|&class_name| {
                    formatted.clear();
                    formatted.push('.');
                    formatted.push_str(class_name);
                    changed_keys.contains(&formatted)
                })
            } else {
                false
            };

            if !(id_match || class_match) {
                return;
            }

            // Apply updated styles
            if let Some(id) = style.id {
                if let Some(ext) = styles.get_style_by_id(&id.0) {
                    changed |= empty_style.update_from(ext);
                } else {
                    changed = true;
                }
            }

            for class_name in _class_split {
                formatted.clear();
                formatted.push('.');
                formatted.push_str(class_name);

                if let Some(ext) = styles.get_style_by_class_name(&formatted) {
                    changed |= empty_style.merge_external(ext);
                } else {
                    changed = true;
                }
            }

            if changed {
                apply_styles_from_external_json(
                    &mut style.background_color,
                    &mut style.border_color,
                    &mut style.border_radius,
                    &mut style.visibility,
                    &mut style.z_index,
                    &mut style.node,
                    &mut style.box_shadow,
                    &empty_style,
                    &style.default_style,
                );
            }
        });

        let elapsed = now.elapsed();
        println!("apply style in : {:.2?}", elapsed);
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ExternalTextStyleQuery {
    pub text_font: Option<&'static mut TextFont>,
    pub text_color: Option<&'static mut TextColor>,
    pub cosmic_text_data: Option<&'static mut CosmicTextData>,
    pub id: Option<&'static WidgetId>,
    pub class: Option<&'static WidgetClasses>,
    pub default_text_config: Option<&'static DefaultTextConfig>,
    pub default_text_span_config: Option<&'static DefaultTextSpanConfig>,
    pub default_cosmic_entity: Option<&'static DefaultCosmicTextEntity>
}

pub(crate) fn detect_text_external_styles_change(
    mut styles: ResMut<StylesKeyValueResource>,
    mut text_query: Query<ExternalTextStyleQuery>
) {
    if styles.is_changed() {

        text_query.par_iter_mut().for_each(|style| {
            let mut _id = String::new();
            let mut _class_split: HashSet<&str> = HashSet::new();
            let mut changed = false;
            let mut empty_style = WidgetStyle::default();

            // true if id or one of the classes is inside styles.changed_key
            let mut has_external_change = false;

            if let Some(id) = style.id {
                _id = id.0.clone();

                if styles.changed_keys.contains(&_id) {
                    has_external_change = true;
                }
            }
            if let Some(classes) = style.class {
                _class_split = classes.0.split_whitespace().collect();

                for class_name in &_class_split {
                    let formatted = format!(".{class_name}");
                    if styles.changed_keys.contains(&formatted) {
                        has_external_change = true;
                    }
                }
            }

            if has_external_change {
                if let Some(external_style) = styles.get_style_by_id(&_id) {
                    changed = empty_style.update_from(external_style);
                }
                else {
                    // Style was removed from json, Reset to default
                    changed = true;
                }

                for class_name in _class_split {
                    let formatted = format!(".{class_name}");
                    if let Some(external_style) = styles.get_style_by_class_name(&formatted) {
                        changed = empty_style.merge_external(external_style);
                    }
                    else {
                        // Style was removed from json, Reset to default
                        changed = true;
                    }
                }
            }

            if changed {
                apply_text_styles_from_external_json(
                    &empty_style,
                    style.default_text_config,
                    style.default_text_span_config,
                    style.text_font,
                    style.text_color,
                );
                apply_text_styles_for_cosmic_text(
                    &empty_style,
                    style.default_cosmic_entity,
                    style.cosmic_text_data
                );
            }
        });
        styles.changed_keys.clear();
    }
}

pub fn finish_style_applying_system(mut builder_res: ResMut<FamiqResource>) {
    builder_res.external_style_applied = true;
}

pub(crate) fn apply_text_styles_for_cosmic_text(
    local_style: &WidgetStyle,
    default_cosmic_text_entity: Option<&DefaultCosmicTextEntity>,
    cosmic_text_data: Option<Mut<'_, CosmicTextData>>,
) {
    let mut set_new_color = false;
    let mut set_new_size = false;

    if let Some(mut _cosmic_text_data) = cosmic_text_data {
        if let Some(font_size) = &local_style.font_size {
            if let Ok(parsed_value) = font_size.trim().parse::<f32>() {
                _cosmic_text_data.size = parsed_value;
                set_new_size = true;

            }
        }
        if let Some(color) = &local_style.color {
            if let Some(v) = parse_color(color) {
                _cosmic_text_data.color = v;
                set_new_color = true;
            }
        }

        if let Some(default_cosmic_text) = default_cosmic_text_entity {
            if !set_new_size {
                _cosmic_text_data.size = default_cosmic_text.text_data.size;
            }
            if !set_new_color {
                _cosmic_text_data.color = default_cosmic_text.text_data.color;
            }
        }
    }
}

pub(crate) fn apply_text_styles_from_external_json(
    local_style: &WidgetStyle,
    default_text_entity: Option<&DefaultTextConfig>,
    default_text_span_entity: Option<&DefaultTextSpanConfig>,
    text_font: Option<Mut<'_, TextFont>>,
    text_color: Option<Mut<'_, TextColor>>,
) {
    if let Some(font_size) = &local_style.font_size {
        if let Ok(parsed_value) = font_size.trim().parse::<f32>() {
            if let Some(mut text_font_size) = text_font {
                text_font_size.font_size = parsed_value;
            }
        }
    } else {
        if let Some(default_text) = default_text_entity {
            if let Some(mut text_font_size) = text_font {
                text_font_size.font_size = default_text.text_font.font_size.clone();
            }
        }
        else if let Some(default_text_span) = default_text_span_entity {
            if let Some(mut text_font_size) = text_font {
                text_font_size.font_size = default_text_span.text_font.font_size.clone();
            }
        }
    }

    if let Some(color) = &local_style.color {
        if let Some(v) = parse_color(color) {
            if let Some(mut _text_color) = text_color {
                _text_color.0 = v;
            }
        }
    } else {
        if let Some(default_text) = default_text_entity {
            if let Some(mut _text_color) = text_color {
                _text_color.0 = default_text.text_color.0.clone();
            }
        }
        else if let Some(default_text_span) = default_text_span_entity {
            if let Some(mut _text_color) = text_color {
                _text_color.0 = default_text_span.text_color.0.clone();
            }
        }
    }
}

pub(crate) fn apply_styles_from_external_json(
    bg_color: &mut BackgroundColor,
    border_color: &mut BorderColor,
    border_radius: &mut BorderRadius,
    visibility: &mut Visibility,
    z_index: &mut ZIndex,
    node: &mut Node,
    box_shadow: &mut BoxShadow,
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetConfig
) {
    utils::_handle_apply_padding(widget_style, default_widget_entity, node);
    utils::_handle_apply_margin(widget_style, default_widget_entity, node);
    utils::_handle_apply_border(widget_style, default_widget_entity, node);
    utils::_handle_apply_box_shadow(widget_style, default_widget_entity, box_shadow);

    if let Some(bg_color_value) = &widget_style.background_color {
        if let Some(v) = parse_background_color(&bg_color_value) {
            *bg_color = v;
        }
    } else {
        *bg_color = default_widget_entity.background_color.clone();
    }

    if let Some(bd_radius_value) = &widget_style.border_radius {
        if let Some(v) = parse_border_radius(&bd_radius_value) {
            *border_radius = v;
        }
    }  else {
        *border_radius = default_widget_entity.border_radius.clone();
    }

    if let Some(border_color_value) = &widget_style.border_color {
        if let Some(v) = parse_border_color(&border_color_value) {
            *border_color = v;
        }
    } else {
        *border_color = default_widget_entity.border_color.clone();
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
}
