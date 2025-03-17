//! Famiq's global resources, used by all modules.

use bevy::utils::hashbrown::HashMap;
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::widgets::list_view::ListViewMovePanelEntity;
use crate::widgets::modal::FaModalContainerEntity;
use crate::widgets::*;
use crate::utils::*;

#[derive(Clone, Default)]
pub struct ContainableData {
    pub entity: Option<Entity>,
    pub children: Vec<Entity>
}

#[derive(PartialEq, Clone, Default)]
pub enum ContainableMethodCall {
    #[default]
    AddChildren,
    InsertChildren,
    RemoveChildren
}

/// Resource to add/insert/remove children for containable widgets
/// including `fa_container`, `fa_listview` and `fa_modal`.
#[derive(Resource, Default)]
pub struct FaContainableResource {
    pub containers: HashMap<String, ContainableData>, // id - ContainableData
    pub(crate) method_called: ContainableMethodCall,
    pub(crate) changed_container: Option<Entity>,
    pub(crate) to_use_children: Vec<Entity>,
    pub(crate) insert_index: usize
}

/// trait for `fa_container`, `fa_modal` and `fa_listview`
pub trait ContainableResourceAction {
    fn add_children(&mut self, id: &str, children: &[Entity]);

    fn insert_children(&mut self, id: &str, index: usize, children: &[Entity]);

    fn remove_children(&mut self, id: &str, children: &[Entity]);
}

impl ContainableResourceAction for FaContainableResource {
    fn add_children(&mut self, id: &str, children: &[Entity]) {
        if let Some(container) = self.containers.get_mut(id) {
            container.children.extend(children.iter().cloned());
            self.method_called = ContainableMethodCall::AddChildren;
            self.to_use_children.clear();
            self.to_use_children.extend_from_slice(children);
            self.changed_container = container.entity;
        }
    }

    fn insert_children(&mut self, id: &str, index: usize, children: &[Entity]) {
        if let Some(container) = self.containers.get_mut(id) {
            container.children.splice(index..index, children.iter().cloned());
            self.method_called = ContainableMethodCall::InsertChildren;
            self.to_use_children.clear();
            self.to_use_children.extend_from_slice(children);
            self.changed_container = container.entity;
            self.insert_index = index;
        }
    }

    fn remove_children(&mut self, id: &str, children: &[Entity]) {
        if let Some(container) = self.containers.get_mut(id) {
            let to_remove: std::collections::HashSet<Entity> = children.iter().cloned().collect();
            container.children.retain(|child| !to_remove.contains(child));
            self.method_called = ContainableMethodCall::RemoveChildren;
            self.to_use_children.clear();
            self.to_use_children.extend_from_slice(children);
            self.changed_container = container.entity;
        }
    }
}

pub(crate) fn detect_fa_containable_resource_change(
    mut commands: Commands,
    mut styles_res: ResMut<StylesKeyValueResource>,
    containable_res: Res<FaContainableResource>,
    fa_listview_q: Query<&ListViewMovePanelEntity>,
    fa_modal_q: Query<&FaModalContainerEntity>,
    mut children_q: Query<(
        &mut Node,
        &mut DefaultWidgetEntity,
        Option<&FamiqWidgetId>,
        Option<&FamiqWidgetClasses>,
    )>,
) {
    if !containable_res.is_changed() || containable_res.is_added() {
        return;
    }
    let Some(changed_container) = containable_res.changed_container else { return };

    // check the correct entity to use
    let to_use_entity = fa_listview_q
        .get(changed_container)
        .map(|panel_entity| panel_entity.0)
        .or_else(|_| fa_modal_q.get(changed_container).map(|sub_container| sub_container.0))
        .unwrap_or(changed_container);

    let mut entity_commands = commands.entity(to_use_entity);
    match containable_res.method_called {
        ContainableMethodCall::AddChildren => {
            entity_commands.add_children(&containable_res.to_use_children);
        }
        ContainableMethodCall::InsertChildren => {
            entity_commands.insert_children(containable_res.insert_index, &containable_res.to_use_children);
        }
        ContainableMethodCall::RemoveChildren => {
            for &child in &containable_res.to_use_children {
                commands.entity(child).despawn_recursive();
            }
        }
    }

    let mut changed_json_style_keys = Vec::with_capacity(containable_res.to_use_children.len());
    for &child in &containable_res.to_use_children {
        if let Ok((mut node, mut default_widget, id, class)) = children_q.get_mut(child) {
            if node.display == Display::None {
                node.display = Display::Flex;
                default_widget.node.display = Display::Flex;
            }
            if let Some(id) = id {
                changed_json_style_keys.push(id.0.clone());
            }
            if let Some(classes) = class {
                classes.0.split_whitespace().for_each(|class_name| {
                    let formatted = format!(".{class_name}");
                    if !changed_json_style_keys.contains(&formatted) {
                        changed_json_style_keys.push(formatted);
                    }
                });
            }
        }
    }
    if !changed_json_style_keys.is_empty() {
        styles_res.changed_keys = changed_json_style_keys;
    }
}


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
