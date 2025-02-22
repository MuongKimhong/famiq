use bevy::ecs::system::EntityCommands;
use bevy::utils::HashMap;
use bevy::asset::{io::AssetSourceId, AssetPath, AssetPlugin};
use bevy::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::plugin::{CursorIcons, CursorType};
use crate::widgets::style_parse::*;
use crate::widgets::{WidgetStyle, DefaultWidgetEntity};
use crate::errors::StylesFileError;
use crate::widgets::{FamiqWidgetId, FamiqWidgetClasses};

pub fn read_styles_json_file(path: &str) -> Result<HashMap<String, WidgetStyle>, StylesFileError> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(StylesFileError::StylesFileDoesNotExist),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| StylesFileError::ReadStylesFileFail)?;

    contents = contents
        .replace("\u{a0}", " ")
        .replace("\t", "    ")
        .replace("\\\"", "\"");

    let styles: HashMap<String, WidgetStyle> = serde_json::from_str(&contents)
        .map_err(|_| StylesFileError::ReadStylesFromFileToStructFail)?;

    Ok(styles)
}

// extract bevy Val enum value
pub fn extract_val(val: Val) -> Option<f32> {
    match val {
        Val::Px(value) => Some(value),
        Val::Percent(value) => Some(value),
        Val::Vw(value) => Some(value),
        Val::Vh(value) => Some(value),
        _ => None,
    }
}

// add an entity as child to another entity
pub fn entity_add_child<'a>(root_node: &'a mut EntityCommands, child: Entity, parent: Entity) {
    root_node.commands().entity(parent).add_child(child);
}

// add multiple entities as children to another entity
pub fn entity_add_children<'a>(
    root_node: &'a mut EntityCommands,
    children: &Vec<Entity>,
    parent: Entity,
) {
    root_node.commands().entity(parent).add_children(children);
}


pub fn lighten_color(percentage: f32, color: &Color) -> Option<Color> {
    let multiplier = percentage / 100.0;

    if let Color::Srgba(mut value) = color {
        value.red = (value.red + (1.0 - value.red) * multiplier).clamp(0.0, 1.0);
        value.green = (value.green + (1.0 - value.green) * multiplier).clamp(0.0, 1.0);
        value.blue = (value.blue + (1.0 - value.blue) * multiplier).clamp(0.0, 1.0);
        return Some(Color::Srgba(value));
    }
    if let Color::LinearRgba(mut value) = color {
        value.red = (value.red + (1.0 - value.red) * multiplier).clamp(0.0, 1.0);
        value.green = (value.green + (1.0 - value.green) * multiplier).clamp(0.0, 1.0);
        value.blue = (value.blue + (1.0 - value.blue) * multiplier).clamp(0.0, 1.0);
        return Some(Color::LinearRgba(value));
    }
    if let Color::Hsla(mut value) = color {
        value.lightness = (value.lightness + (1.0 - value.lightness) * multiplier).clamp(0.0, 1.0);
        return Some(Color::Hsla(value));
    }
    None
}

pub fn darken_color(percentage: f32, color: &Color) -> Option<Color> {
    let multiplier = 1.0 - (percentage / 100.0);

    if let Color::Srgba(mut value) = color {
        value.red = (value.red * multiplier).clamp(0.0, 1.0);
        value.green = (value.green * multiplier).clamp(0.0, 1.0);
        value.blue = (value.blue * multiplier).clamp(0.0, 1.0);
        return Some(Color::Srgba(value));
    }
    if let Color::LinearRgba(mut value) = color {
        value.red = (value.red * multiplier).clamp(0.0, 1.0);
        value.green = (value.green * multiplier).clamp(0.0, 1.0);
        value.blue = (value.blue * multiplier).clamp(0.0, 1.0);
        return Some(Color::LinearRgba(value));
    }
    if let Color::Hsla(mut value) = color {
        value.lightness = (value.lightness * multiplier).clamp(0.0, 1.0);
        return Some(Color::Hsla(value));
    }
    None
}

pub fn get_embedded_asset_path(file_path: &str) -> AssetPath {
    // path: relative to embedded_assets dir

    let path = Path::new("famiq").join(file_path);
    let source = AssetSourceId::from("embedded");
    AssetPath::from_path(&path).with_source(source).into_owned()
}

pub fn create_test_app() -> App {
    let mut app = App::new();
    // Note the use of `MinimalPlugins` instead of `DefaultPlugins`, as described above.
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.init_resource::<Assets<Shader>>();
    app.init_asset::<Font>();
    app.init_asset::<Image>();
    // Spawning a fake window allows testing systems that require a window.
    app.world_mut().spawn(Window::default());
    app
}

pub fn process_spacing_built_in_class(node: &mut Node, class: &Option<String>) {
    if let Some(class) = class {
        for class_name in class.split_whitespace() {
            if let Some((prefix, value)) = class_name.split_once('-') {

                let spacing_value = if value == "auto" {
                    Val::Auto
                } else if let Ok(num) = value.parse::<f32>() {
                    Val::Px(num * 5.0)
                } else {
                    continue;
                };

                match prefix {
                    // Margin classes
                    "mt" => node.margin.top = spacing_value,
                    "mb" => node.margin.bottom = spacing_value,
                    "ml" => node.margin.left = spacing_value,
                    "mr" => node.margin.right = spacing_value,
                    "my" => {
                        node.margin.top = spacing_value;
                        node.margin.bottom = spacing_value;
                    }
                    "mx" => {
                        node.margin.left = spacing_value;
                        node.margin.right = spacing_value;
                    }

                    // Padding classes
                    "pt" => node.padding.top = spacing_value,
                    "pb" => node.padding.bottom = spacing_value,
                    "pl" => node.padding.left = spacing_value,
                    "pr" => node.padding.right = spacing_value,
                    "py" => {
                        node.padding.top = spacing_value;
                        node.padding.bottom = spacing_value;
                    }
                    "px" => {
                        node.padding.left = spacing_value;
                        node.padding.right = spacing_value;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn mask_string(input: &str) -> String {
    "*".repeat(input.len())
}

pub fn insert_id_and_class<'a>(
    root_node: &'a mut EntityCommands,
    entity: Entity,
    id: &Option<String>,
    class: &Option<String>
) {
    if let Some(id) = id {
        root_node.commands().entity(entity).insert(FamiqWidgetId(id.to_owned()));
    }
    if let Some(class) = class {
        root_node.commands().entity(entity).insert(FamiqWidgetClasses(class.to_owned()));
    }
}

pub fn _handle_apply_margin(
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

pub fn _handle_apply_padding(
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

pub fn _handle_apply_border(
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

pub fn _handle_apply_box_shadow(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetEntity,
    box_shadow: &mut BoxShadow
) {
    if let Some(shadow_color_value) = &widget_style.shadow_color {
        if let Some(v) = parse_color(&shadow_color_value) {
            box_shadow.color = v;
        }
    } else {
        box_shadow.color = default_widget_entity.box_shadow.color;
    }

    if let Some(shadow_spread_value) = &widget_style.shadow_spread {
        if let Some(v) = parse_val(&shadow_spread_value) {
            box_shadow.spread_radius = v;
        }
    } else {
        box_shadow.spread_radius = default_widget_entity.box_shadow.spread_radius;
    }

    if let Some(shadow_blur_value) = &widget_style.shadow_blur {
        if let Some(v) = parse_val(&shadow_blur_value) {
            box_shadow.blur_radius = v;
        }
    } else {
        box_shadow.blur_radius = default_widget_entity.box_shadow.blur_radius;
    }

    if let Some(shadow_x_value) = &widget_style.shadow_x_offset {
        if let Some(v) = parse_val(&shadow_x_value) {
            box_shadow.x_offset = v;
        }
    } else {
        box_shadow.x_offset = default_widget_entity.box_shadow.x_offset;
    }

    if let Some(shadow_y_value) = &widget_style.shadow_y_offset {
        if let Some(v) = parse_val(&shadow_y_value) {
            box_shadow.y_offset = v;
        }
    } else {
        box_shadow.y_offset = default_widget_entity.box_shadow.y_offset;
    }
}

pub fn _handle_apply_border_radius(
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

pub fn _change_cursor_icon(
    commands: &mut Commands,
    res: &Res<CursorIcons>,
    window_entity: Entity,
    _type: CursorType
) {
    commands.entity(window_entity).insert(match _type {
        CursorType::Pointer => res.pointer.clone(),
        CursorType::Text => res.text.clone(),
        _ => res.normal.clone(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_built_in_class() {
        let mut node = Node {
            margin: UiRect::all(Val::Px(0.0)),
            padding: UiRect::all(Val::Px(0.0)),
            ..default()
        };

        let test_class =  Some(String::from("mx-auto my-2 pb-2 pr-3"));
        process_spacing_built_in_class(&mut node, &test_class);

        assert_eq!(Val::Auto, node.margin.left);
        assert_eq!(Val::Auto, node.margin.right);
        assert_eq!(Val::Px(10.0), node.margin.top);
        assert_eq!(Val::Px(10.0), node.margin.bottom);

        assert_eq!(Val::Px(10.0), node.padding.bottom);
        assert_eq!(Val::Px(15.0), node.padding.right);
    }
}
