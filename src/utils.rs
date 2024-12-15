use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::fs::File;
use std::io::Read;

use crate::errors::StylesFileError;
use crate::widgets::{StyleKeyValue, StylesKeyValue, WidgetStyle};

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

// get one style key-value pair based on id
pub fn get_widget_styles_map(id: &str, all_styles_maps: &StylesKeyValue) -> Option<WidgetStyle> {
    for style_map in all_styles_maps {
        if let Some(style) = style_map.get(id) {
            return Some(style.clone());
        }
    }
    None
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
pub fn entity_push_children<'a>(
    root_node: &'a mut EntityCommands,
    children: &Vec<Entity>,
    parent: Entity,
) {
    root_node.commands().entity(parent).push_children(children);
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
