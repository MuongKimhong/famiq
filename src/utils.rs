use bevy::ecs::system::EntityCommands;
use bevy::asset::{io::AssetSourceId, AssetPath};
use bevy::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::errors::StylesFileError;
use crate::widgets::{StyleKeyValue, StylesKeyValue};

pub fn read_styles_json_file(path: &str) -> Result<StylesKeyValue, StylesFileError> {
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

    let maps = get_widget_styles_maps(&contents).unwrap();
    Ok(maps)
}

// get all style key-value pairs as Vector
pub fn get_widget_styles_maps(
    widget_styles_str: &String,
) -> Result<StylesKeyValue, StylesFileError> {
    let widget_styles: StyleKeyValue = match serde_json::from_str(&widget_styles_str) {
        Ok(v) => v,
        Err(_) => {
            return Err(StylesFileError::ReadStylesFromFileToStructFail);
        }
    };
    let maps: StylesKeyValue = widget_styles
        .into_iter()
        .map(|(key, value)| {
            let mut map = StyleKeyValue::new();
            map.insert(key, value);
            map
        })
        .collect();
    Ok(maps)
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

// by default, bevy's AssetServer expects assets to be inside
// assets folder, and we can load assets without needing to
// include assets/ prefix.
//
// if user provide assets/ prefix, remove it
pub fn strip_assets_prefix(path: &String) -> Option<String> {
    if let Some(normalized) = path.strip_prefix("assets/") {
        Some(normalized.to_string())
    } else {
        None
    }
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
