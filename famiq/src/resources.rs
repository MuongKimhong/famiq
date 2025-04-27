//! Famiq's global resources, used by all modules.

use bevy::platform::collections::HashMap;
use bevy::reflect::TypePath;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;
use cosmic_text::{FontSystem, SwashCache};
use crate::widgets::*;
use crate::utils::*;

/// Resource for detecting style changes in json file
#[derive(Resource, Default)]
pub(crate) struct StylesKeyValueResource {
    pub values: HashMap<String, WidgetStyle>, // key-value of "#widget-id"/".class-name" and all its styles in styles.json
    pub changed_keys: Vec<String>
}

impl StylesKeyValueResource {
    pub fn get_style_by_id(&self, widget_id: &str) -> Option<&WidgetStyle> {
        self.values.get(widget_id)
    }

    pub fn get_style_by_class_name(&self, class_name: &str) -> Option<&WidgetStyle> {
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
    pub root_node_entity: Option<Entity>,

    pub(crate) widget_focus_state: HashMap<Entity, bool>,
    pub(crate) external_style_applied: bool,
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

// /// Store children entity for containable widgets
// #[derive(Resource, Debug, Default)]
// pub struct ContainableChildren {
//     pub data: HashMap<Entity, Vec<Entity>>
// }

// impl ContainableChildren {
//     /// Insert or update data
//     pub fn insert(&mut self, entity: Entity, children: Vec<Entity>) {
//         self.data.insert(entity, children);
//     }

//     /// Replace child entity with provided one
//     pub fn update_child(&mut self, parent: Entity, new_child: Entity, old_child: Entity) {
//         if let Some(children_list) = self.data.get_mut(&parent) {
//             for child in children_list.iter_mut() {
//                 if *child == old_child {
//                     *child = new_child;
//                     return;
//                 }
//             }
//         }
//     }

//     /// Insert new containable, remove old one.
//     pub fn update_containable(&mut self, old: Entity, new: Entity) {
//         if let Some(children_list) = self.data.remove(&old) {
//             self.data.insert(new, children_list);
//         }
//     }

//     pub fn get_children(&self, containable: Entity) -> Option<Vec<Entity>> {
//         if let Some(children_list) = self.data.get(&containable) {
//             return Some(children_list.to_owned());
//         }
//         None
//     }
// }

// reference: https://bevyengine.org/examples/assets/custom-asset/
#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct JsonStyleAsset(pub HashMap<String, WidgetStyle>);

#[derive(Default)]
pub struct JsonStyleAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum JsonStyleAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for JsonStyleAssetLoader {
    type Asset = JsonStyleAsset;
    type Settings = ();
    type Error = JsonStyleAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let json_asset = serde_json::from_slice::<JsonStyleAsset>(&bytes).unwrap();
        Ok(json_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Resource, Debug, Default)]
pub struct JsonStyleAssetState {
    pub initial_loaded: bool,
    pub fully_loaded: bool,
    pub style_handle: Handle<JsonStyleAsset>,
    pub font_handle: Handle<Font>
}
