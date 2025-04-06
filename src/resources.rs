//! Famiq's global resources, used by all modules.

use bevy::utils::hashbrown::HashMap;
use bevy::prelude::*;
use cosmic_text::{FontSystem, SwashCache};
use std::marker::PhantomData;

use crate::widgets::*;
use crate::utils::*;

/// Generic resource for `FaTextInputResource` and `FaSelectionResource`
#[derive(Resource, Default, Debug)]
pub struct InputResource<T> {
    pub values_id: HashMap<String, String>,   // id - value
    _marker: PhantomData<T>
}

/// trait for `fa_text_input` and `fa_selection`
pub trait InputResourceMap {
    /// internal method to insert a value by id
    fn _insert(&mut self, id: String, value: String);

    /// Get a value by id
    fn get_value(&self, id: &str) -> String;

    /// Check if an id exists
    fn exists(&self, id: &str) -> bool;
}

/// Generic methods for InputResource<T>
impl<T> InputResourceMap for InputResource<T> {
    fn _insert(&mut self, id: String, value: String) {
        self.values_id.insert(id, value);
    }

    fn get_value(&self, id: &str) -> String {
        self.values_id.get(id).map_or_else(
            || String::from(""),
            |v| if v == "-/-" { String::from("") } else { v.clone() },
        )
    }

    fn exists(&self, id: &str) -> bool {
        self.values_id.contains_key(id)
    }
}

/// Resource for detecting style changes in json file
#[derive(Resource, Default)]
pub(crate) struct StylesKeyValueResource {
    pub(crate) values: HashMap<String, WidgetStyle>, // key-value of "#widget-id"/".class-name" and all its styles in styles.json
    pub(crate) changed_keys: Vec<String>
}

impl StylesKeyValueResource {
    pub(crate) fn get_style_by_id(&self, widget_id: &str) -> Option<&WidgetStyle> {
        self.values.get(widget_id)
    }

    pub(crate) fn get_style_by_class_name(&self, class_name: &str) -> Option<&WidgetStyle> {
        self.values.get(class_name)
    }
}

#[derive(Resource)]
pub struct FamiqResource {
    /// font path relative to project root
    pub font_path: String,

    /// user external style (json) file path relative to project root
    pub style_path: String,

    /// read external style (json) file and apply styles to widget every single frame
    pub hot_reload_styles: bool,

    /// copied text from text input
    pub copied_text: String,

    pub(crate) widget_focus_state: HashMap<Entity, bool>,
    pub(crate) external_style_applied: bool,
    pub(crate) root_node_entity: Option<Entity>,
}

#[derive(Resource)]
pub struct CosmicFontSystem(pub FontSystem);

#[derive(Resource)]
pub struct CosmicSwashCache(pub SwashCache);

impl FamiqResource {
    pub fn update_or_insert_focus_state(&mut self, entity: Entity, state: bool) {
        if let Some(old_value) = self.widget_focus_state.get_mut(&entity) {
            *old_value = state;
        } else {
            self.widget_focus_state.insert(entity, state);
        }
    }

    pub fn update_all_focus_states(&mut self, new_state: bool) {
        for (_, state) in self.widget_focus_state.iter_mut() {
            *state = new_state;
        }
    }

    pub fn get_widget_focus_state(&self, entity: &Entity) -> Option<bool> {
        if let Some(&state) = self.widget_focus_state.get(entity) {
            return Some(state);
        }
        None
    }

    pub fn new() -> Self {
        Self {
            font_path: get_embedded_asset_path("embedded_assets/fonts/fira-mono-medium.ttf").to_string(),
            style_path: "assets/styles.json".to_string(),
            hot_reload_styles: false,
            widget_focus_state: HashMap::new(),
            external_style_applied: false,
            root_node_entity: None,
            copied_text: String::new()
        }
    }
}

#[derive(Debug)]
pub enum RVal {
    Num(i32),
    FNum(f32),
    Str(&'static str)
}

#[derive(Resource, Debug, Default)]
pub struct RData {
    pub data: HashMap<String, RVal>,
    pub changed_keys: Vec<String>,
    pub has_changed: bool
}

impl RData {
    pub fn type_match(old_val: &RVal, new_val: &RVal) -> bool {
        match (old_val, new_val) {
            (RVal::Num(_), RVal::Num(_)) => true,
            (RVal::FNum(_), RVal::FNum(_)) => true,
            (RVal::Str(_), RVal::Str(_)) => true,
            _ => false
        }
    }
}

pub fn detect_reactive_data_change(
    mut r_data: ResMut<RData>,
    mut keys_q: Query<(&mut Text, &ReactiveKeys)>
) {
    if !r_data.is_changed() && !r_data.is_added() {
        return;
    }
    println!("is changed");
    for (mut r_text, r_keys) in keys_q.iter_mut() {
        println!("r_text {:?}, r_keys: {:?}", r_text, r_keys);
        let mut text = r_text.0.clone();

        for (i, key) in r_keys.0.iter().enumerate() {
            if let Some(value) = r_data.data.get(key) {
                let placeholder = format!("$[{}]", i);

                match value {
                    RVal::Num(v) => text = text.replace(&placeholder, &v.to_string()),
                    RVal::FNum(v) => text = text.replace(&placeholder, &v.to_string()),
                    RVal::Str(v) => text = text.replace(&placeholder, *v),
                }
            }
        }
        r_text.0 = text;
    }
    r_data.changed_keys.clear();
    r_data.has_changed = false;
}
