//! Famiq's global resources, used by all modules.

use bevy::utils::HashMap;
use bevy::prelude::*;
use std::marker::PhantomData;

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

/// Generic resource for containable widgets `FaContainerResource`,
/// `FaModalResource`, `FaListViewResource`.
#[derive(Resource, Default)]
pub struct ContainableResource<T> {
    pub containers: HashMap<String, ContainableData>, // id - ContainableData
    pub(crate) method_called: ContainableMethodCall,
    pub(crate) changed_container: Option<Entity>,
    pub(crate) to_use_children: Vec<Entity>,
    pub(crate) insert_index: usize,
    _marker: PhantomData<T>
}

/// trait for `fa_container`, `fa_modal` and `fa_listview`
pub trait ContainableResourceAction {
    fn add_children(&mut self, id: &str, children: &[Entity]);

    fn insert_children(&mut self, id: &str, index: usize, children: &[Entity]);

    fn remove_children(&mut self, id: &str, children: &[Entity]);
}

impl<T> ContainableResourceAction for ContainableResource<T> {
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

/// Generic resource for `FaTextInputResource` and `FaSelectionResource`
#[derive(Resource, Default, Debug)]
pub struct InputResource<T> {
    pub values_id: HashMap<String, String>,   // id - value
    pub values_entity: HashMap<Entity, String>, // entity - value
    _marker: PhantomData<T>
}

/// trait for `fa_text_input` and `fa_selection`
pub trait InputResourceMap {
    /// internal method to insert a value by id
    fn _insert_by_id(&mut self, id: String, value: String);

    /// internal method to insert a value by entity
    fn _insert_by_entity(&mut self, entity: Entity, value: String);

    /// Get a value by id
    fn get_value_by_id(&self, id: &str) -> String;

    /// Get a value by entity
    fn get_value_by_entity(&self, entity: Entity) -> String;

    /// Check if an id exists
    fn exists_by_id(&self, id: &str) -> bool;

    /// Check if an entity exists
    fn exists_by_entity(&self, entity: Entity) -> bool;
}

/// Generic methods for InputResource<T>
impl<T> InputResourceMap for InputResource<T> {
    fn _insert_by_id(&mut self, id: String, value: String) {
        self.values_id.insert(id, value);
    }

    fn _insert_by_entity(&mut self, entity: Entity, value: String) {
        self.values_entity.insert(entity, value);
    }

    fn get_value_by_id(&self, id: &str) -> String {
        self.values_id.get(id).map_or_else(
            || String::from(""),
            |v| if v == "-/-" { String::from("") } else { v.clone() },
        )
    }

    fn get_value_by_entity(&self, entity: Entity) -> String {
        self.values_entity.get(&entity).map_or_else(
            || String::from(""),
            |v| if v == "-/-" { String::from("") } else { v.clone() },
        )
    }

    fn exists_by_id(&self, id: &str) -> bool {
        self.values_id.contains_key(id)
    }

    fn exists_by_entity(&self, entity: Entity) -> bool {
        self.values_entity.contains_key(&entity)
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

/// Resource for detecting style changes in systems (user changes a widget's style directly within systems)
#[derive(Resource, Default)]
pub struct FaStyleResource {
    pub(crate) values: HashMap<String, WidgetStyle>,
    pub(crate) changed_keys: Vec<String>,
    pub(crate) internal_changed: bool
}

impl FaStyleResource {
    pub(crate) fn get_style_by_id(&self, id: &str) -> Option<&WidgetStyle> {
        self.values.get(id)
    }

    fn update_key(&mut self, id: &str) {
        let _id = id.to_owned();
        if !self.changed_keys.contains(&_id) {
            self.changed_keys.push(_id);
            self.internal_changed = true;
        }
    }

    fn update_style<F>(&mut self, id: &str, update_fn: F)
        where
            F: FnOnce(&mut WidgetStyle),
        {
            if let Some(style) = self.values.get_mut(id) {
                update_fn(style);
                self.update_key(id);
            }
        }

    /// Set widget background color.
    /// Example: `style_res.set_background_color("#id", "yellow");`
    pub fn set_background_color(&mut self, id: &str, color: impl Into<String>) {
        self.update_style(id, |style| style.background_color = Some(color.into()));
    }

    /// Set widget border color.
    /// Example: `style_res.set_border_color("#id", "yellow");`
    pub fn set_border_color(&mut self, id: &str, color: impl Into<String>) {
        self.update_style(id, |style| style.border_color = Some(color.into()));
    }

    /// Set widget border radius, where radius is "top-left top-right bottom-left bottom-right".
    /// Example: `style_res.set_border_radius("#id", "10px 10px 5px 5px");`
    pub fn set_border_radius(&mut self, id: &str, radius: impl Into<String>) {
        self.update_style(id, |style| style.border_radius = Some(radius.into()));
    }

    /// Set widget width.
    /// Example: `style_res.set_width("#id", "50%");`
    pub fn set_width(&mut self, id: &str, width: impl Into<String>) {
        self.update_style(id, |style| style.width = Some(width.into()));
    }

    /// Set widget height.
    /// Example: `style_res.set_height("#id", "50%");`
    pub fn set_height(&mut self, id: &str, height: impl Into<String>) {
        self.update_style(id, |style| style.height = Some(height.into()));
    }

    /// Set widget visibility.
    /// Example: `style_res.set_visibility("#id", "hidden");`
    pub fn set_visibility(&mut self, id: &str, visibility: impl Into<String>) {
        self.update_style(id, |style| style.visibility = Some(visibility.into()));
    }

    /// Set widget margin, where margin is "left right top bottom".
    /// Example: `style_res.set_margin("#id", "10px 10px 10px 10px");`
    pub fn set_margin(&mut self, id: &str, margin: impl Into<String>) {
        self.update_style(id, |style| style.margin = Some(margin.into()));
    }

    /// Set widget padding, where padding is "left right top bottom".
    /// Example: `style_res.set_padding("#id", "10px 10px 10px 10px");`
    pub fn set_padding(&mut self, id: &str, padding: impl Into<String>) {
        self.update_style(id, |style| style.padding = Some(padding.into()));
    }

    /// Set widget display.
    /// Example: `style_res.set_display("#id", "grid");`
    pub fn set_display(&mut self, id: &str, display: impl Into<String>) {
        self.update_style(id, |style| style.display = Some(display.into()));
    }

    /// Set widget text color
    /// Example: `style_res.set_color("#id", "green");`
    pub fn set_color(&mut self, id: &str, color: impl Into<String>) {
        self.update_style(id, |style| style.color = Some(color.into()));
    }

    /// Set widget text size.
    /// Example: `style_res.set_font_size("#id", "20");`
    pub fn set_font_size(&mut self, id: &str, size: impl Into<String>) {
        self.update_style(id, |style| style.font_size = Some(size.into()));
    }

    /// Set widget shadow color.
    /// Example: `style_res.set_shadow_color("#id", "srgba 0.4, 0.4, 0.3, 0.4");`
    pub fn set_shadow_color(&mut self, id: &str, color: impl Into<String>) {
        self.update_style(id, |style| style.shadow_color = Some(color.into()));
    }

    /// Detect when a widget with id is created
    pub(crate) fn detect_new_widget_with_id(
        mut style_res: ResMut<FaStyleResource>,
        widget_q: Query<&FamiqWidgetId, Added<FamiqWidgetId>>
    ) {
        for id in widget_q.iter() {
            if style_res.values.get(&id.0).is_none() {
                style_res.values.insert(id.0.clone(), WidgetStyle::default());
            }
        }
    }

    pub(crate) fn detect_internal_widget_style_change(
        style_res: Res<FaStyleResource>,
        mut widget_query: WidgetStyleQuery
    ) {
        if style_res.internal_changed {
            for (
                id,
                _,
                mut node,
                mut bg_color,
                mut bd_color,
                mut bd_radius,
                mut z_index,
                mut visibility,
                mut box_shadow,
                default_widget,
            ) in widget_query.iter_mut() {
                let mut changed = false;
                let mut empty_style = WidgetStyle::default();

                if let Some(id) = id {
                    if style_res.changed_keys.contains(&id.0) {
                        if let Some(external_style) = style_res.get_style_by_id(&id.0) {
                            changed = empty_style.merge_external(external_style);
                        }
                    }
                }
                if changed {
                    apply_styles_from_external_json(
                        &mut bg_color,
                        &mut bd_color,
                        &mut bd_radius,
                        &mut visibility,
                        &mut z_index,
                        &mut node,
                        &mut box_shadow,
                        &empty_style,
                        default_widget
                    );
                }
            }
        }
    }

    pub(crate) fn detect_internal_text_style_change(
        mut style_res: ResMut<FaStyleResource>,
        mut text_query: Query<(
            &mut TextFont,
            &mut TextColor,
            Option<&FamiqWidgetId>,
            Option<&DefaultTextEntity>,
            Option<&DefaultTextSpanEntity>
        )>
    ) {
        if style_res.internal_changed {
            for (
                mut text_font,
                mut text_color,
                id,
                default_text_entity,
                default_text_span_entity
            ) in text_query.iter_mut() {
                let mut changed = false;
                let mut empty_style = WidgetStyle::default();

                if let Some(id) = id {
                    if style_res.changed_keys.contains(&id.0) {
                        if let Some(external_style) = style_res.get_style_by_id(&id.0) {
                            changed = empty_style.merge_external(external_style);
                        }
                    }
                }
                if changed {
                    apply_text_styles_from_external_json(
                        &empty_style,
                        default_text_entity,
                        default_text_span_entity,
                        &mut text_font,
                        &mut text_color
                    );
                }
            }
            style_res.changed_keys.clear();
            style_res.internal_changed = false;
        }
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

    pub(crate) widget_focus_state: HashMap<Entity, bool>,
    pub(crate) external_style_applied: bool,
    pub(crate) root_node_entity: Option<Entity>,
    pub(crate) tooltip_registered: bool
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
            font_path: get_embedded_asset_path("embedded_assets/fonts/fira-mono-regular.ttf").to_string(),
            style_path: "assets/styles.json".to_string(),
            hot_reload_styles: false,
            widget_focus_state: HashMap::new(),
            external_style_applied: false,
            root_node_entity: None,
            tooltip_registered: false
        }
    }
}
