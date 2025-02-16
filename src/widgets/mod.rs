pub mod button;
pub mod color;
pub mod container;
pub mod fps;
pub mod list_view;
pub mod selection;
pub mod style;
pub mod style_parse;
pub mod text;
pub mod text_input;
pub mod circular;
pub mod modal;
pub mod image;
pub mod bg_image;
pub mod helper;
pub mod tooltip;
pub mod progress_bar;

pub use button::fa_button;
pub use circular::fa_circular;
pub use container::fa_container;
pub use fps::fa_fps;
pub use image::fa_image;
pub use list_view::fa_listview;
pub use modal::fa_modal;
pub use text::fa_text;
pub use text_input::fa_text_input;
pub use selection::fa_selection;
pub use bg_image::fa_bg_image;
pub use progress_bar::fa_progress_bar;
use tooltip::FaToolTip;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::get_embedded_asset_path;

/// ResourceMap trait for `fa_text_input` and `fa_selection`
pub trait ResourceMap {
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

// key-value of "#widget-id"/".class-name" and all its styles in styles.json
pub type StyleKeyValue = HashMap<String, WidgetStyle>;
pub type StylesKeyValue = Vec<StyleKeyValue>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WidgetType {
    Root, // globalzindex 1
    Button,
    Container,
    Text,
    FpsText, // globalzindex 6
    TextInput,
    TextInputTogglePasswordIcon,
    ListView,
    ListViewItem,
    Selection,
    SelectionChoice, // choicepanel globalzindex 2
    Circular,
    Modal, // globalzindex 5
    Image
}

#[derive(Resource)]
pub struct StylesKeyValueResource(pub StylesKeyValue);

impl StylesKeyValueResource {
    pub fn get_style_by_id(&self, widget_id: &str) -> Option<&WidgetStyle> {
        self.0.iter().flat_map(|map| map.get(widget_id)).next()
    }

    pub fn get_style_by_class_name(&self, class_name: &str) -> Option<&WidgetStyle> {
        let classname = format!(".{class_name}");
        self.0.iter().flat_map(|map| map.get(&classname)).next()
    }
}

#[derive(Component)]
pub struct FamiqToolTipText(pub String);

#[derive(Component, Deref)]
pub struct FamiqWidgetId(pub String);

#[derive(Component, Deref)]
pub struct FamiqWidgetClasses(pub String);

#[derive(Component)]
pub struct DefaultWidgetEntity {
    pub node: Node,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub background_color: BackgroundColor,
    pub z_index: ZIndex,
    pub visibility: Visibility,
}

impl DefaultWidgetEntity {
    pub fn new(
        node: Node,
        border_color: BorderColor,
        border_radius: BorderRadius,
        background_color: BackgroundColor,
        z_index: ZIndex,
        visibility: Visibility,
    ) -> Self {
        Self {
            node,
            border_color,
            border_radius,
            background_color,
            z_index,
            visibility,
        }
    }
}

#[derive(Component)]
pub struct DefaultTextEntity {
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}

impl DefaultTextEntity {
    pub fn new(
        text: Text,
        text_font: TextFont,
        text_color: TextColor,
        text_layout: TextLayout,
    ) -> Self {
        Self {
            text,
            text_font,
            text_color,
            text_layout,
        }
    }
}

#[derive(Component)]
pub struct DefaultTextSpanEntity {
    pub text: TextSpan,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

impl DefaultTextSpanEntity {
    pub fn new(
        text: TextSpan,
        text_font: TextFont,
        text_color: TextColor
    ) -> Self {
        Self {
            text,
            text_font,
            text_color,
        }
    }
}

#[derive(Component)]
pub struct ExternalStyleHasChanged(pub bool);

#[derive(Resource)]
pub struct FamiqResource {
    // font path relative to project root
    pub font_path: String,

    // user external style (json) file path relative to project root
    pub style_path: String,

    // read external style (json) file and apply styles to widget every single frame
    pub hot_reload_styles: bool,

    pub widget_focus_state: HashMap<Entity, bool>,

    pub external_style_applied: bool,

    pub root_node_entity: Option<Entity>,

    pub tooltip_registered: bool
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

#[derive(Component)]
pub struct IsFaWidgetRoot;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Component)]
pub struct WidgetStyle {
    pub color: Option<String>,     // for fa_text, fa_fps, Text color only
    pub font_size: Option<String>, // for fa_text, fa_fps, Text font_size only
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_radius: Option<String>,
    pub visibility: Option<String>,
    pub z_index: Option<String>,
    pub display: Option<String>,
    pub position_type: Option<String>,
    pub overflow_x: Option<String>,
    pub overflow_y: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
    pub top: Option<String>,
    pub bottom: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub min_width: Option<String>,
    pub min_height: Option<String>,
    pub max_width: Option<String>,
    pub max_height: Option<String>,
    pub align_items: Option<String>,
    pub justify_items: Option<String>,
    pub align_self: Option<String>,
    pub justify_content: Option<String>,
    pub margin: Option<String>,
    pub padding: Option<String>,
    pub border: Option<String>,
    pub flex_direction: Option<String>,
    pub flex_wrap: Option<String>,
    pub flex_grow: Option<String>,
    pub flex_shrink: Option<String>,
    pub flex_basis: Option<String>,
    pub row_gap: Option<String>,
    pub column_gap: Option<String>,
    pub grid_auto_flow: Option<String>,
    pub margin_left: Option<String>,
    pub margin_right: Option<String>,
    pub margin_top: Option<String>,
    pub margin_bottom: Option<String>,
    pub padding_left: Option<String>,
    pub padding_right: Option<String>,
    pub padding_top: Option<String>,
    pub padding_bottom: Option<String>,
    pub border_left: Option<String>,
    pub border_right: Option<String>,
    pub border_top: Option<String>,
    pub border_bottom: Option<String>,
    pub border_radius_top_left: Option<String>,
    pub border_radius_top_right: Option<String>,
    pub border_radius_bottom_left: Option<String>,
    pub border_radius_bottom_right: Option<String>
}

impl WidgetStyle {
    // assign external to self no matter what
    pub fn from_external(&mut self, external: &WidgetStyle) {
        *self = external.clone();
    }

    // merge external into & overwrite fields in self if
    // - field in self is "null"
    // - field in both self & external are not "null"
    pub fn merge_external(&mut self, external: &WidgetStyle) -> bool {
        let mut has_changed = false;

        let mut self_map = serde_json::to_value(&mut *self).unwrap();
        let external_map = serde_json::to_value(external).unwrap();

        let merged_map = self_map.as_object_mut().unwrap();
        for (key, value) in external_map.as_object().unwrap() {

            let self_field = merged_map.get(key).unwrap();
            let external_field = external_map.get(key).unwrap();

            if (self_field.is_null() && !external_field.is_null()) ||
               (!self_field.is_null() && !external_field.is_null()) {
                    merged_map.insert(key.clone(), value.clone());
                    has_changed = true;
               }
        }

        *self = serde_json::from_value(serde_json::Value::Object(merged_map.clone())).unwrap();
        has_changed
    }

    // override fields self that are different from external fields
    pub fn update_from(&mut self, external: &WidgetStyle) -> bool {
        let mut has_changed = false;

        let self_json = serde_json::to_value(&mut *self).unwrap();
        let external_json = serde_json::to_value(external).unwrap();

        if let serde_json::Value::Object(mut self_map) = self_json {
            if let serde_json::Value::Object(external_map) = external_json {
                for (key, external_value) in external_map {
                    if self_map.get(&key) != Some(&external_value) {
                        // Update only if different
                        self_map.insert(key, external_value);
                        has_changed = true;
                    }
                }
            }
            *self = serde_json::from_value(serde_json::Value::Object(self_map)).unwrap();
        }

        has_changed
    }
}


pub struct FamiqBuilder<'a> {
    pub asset_server: &'a ResMut<'a, AssetServer>,
    pub ui_root_node: EntityCommands<'a>,
    pub resource: Mut<'a, FamiqResource>
}

impl<'a> FamiqBuilder<'a> {
    fn _reset_builder_resource(builder_resource: &mut ResMut<FamiqResource>) {
        builder_resource.external_style_applied = false;
    }

    pub fn new(
        commands: &'a mut Commands,
        builder_resource: &'a mut ResMut<FamiqResource>,
        asset_server: &'a ResMut<'a, AssetServer>,
    ) -> Self {
        Self::_reset_builder_resource(builder_resource);
        Self {
            asset_server,
            ui_root_node: commands.entity(builder_resource.root_node_entity.unwrap()),
            resource: builder_resource.reborrow()
        }
    }

    /// Method to use custom font
    ///
    /// # Arguments
    ///
    /// * `font_path` - A path to the font, relative to Bevy's `assets/` folder.
    ///
    /// # Examples
    ///
    /// ## Normal Project Structure
    ///
    /// ```text
    /// my_project/
    /// ├── assets/
    /// │   ├── fonts/
    /// │   │   ├── Some-font.ttf
    /// ├── src/
    /// ```
    ///
    /// ```text
    /// builder.use_font_path("fonts/Some-font.ttf");
    /// ```
    ///
    /// ## Multi-Crate / Workspace Structure
    ///
    /// In a multi-crate workspace, the custom font path is read from the subcrate/member's `assets/` folder:
    ///
    /// ```text
    /// my_project/
    /// ├── sub_crate_1/
    /// │   ├── assets/
    /// │   │   ├── fonts/
    /// │   │   │   ├── Some-font.ttf
    /// │   ├── src/
    /// ├── sub_crate_2/
    /// │   ├── assets/
    /// │   ├── src/
    /// ```
    ///
    /// ```text
    /// // Inside subcrate 1
    /// builder.use_font_path("fonts/Some-font.ttf");
    /// ```
    pub fn use_font_path(mut self, font_path: &str) -> Self {
        self.resource.font_path = font_path.to_string();
        self
    }

    /// Method to use custom style file path.
    ///
    /// # Argument
    /// * style_path: Full path to the json file, relative to root directory.
    pub fn use_style_path(mut self, style_path: &str) -> Self {
        self.resource.style_path = style_path.to_string();
        self
    }

    /// Method to enable hot-reload.
    pub fn hot_reload(mut self) -> Self {
        self.resource.hot_reload_styles = true;
        self
    }

    /// Registers a tooltip option for widgets.
    ///
    /// If `use_font_path` is called, `register_tooltip` must be called **after** `use_font_path`
    /// to ensure that the custom font is applied to the tooltip.
    pub fn register_tooltip(mut self) -> Self {
        if !self.resource.tooltip_registered {
            let font_handle = self.asset_server.load(&self.resource.font_path);
            FaToolTip::new(
                &mut self.ui_root_node,
                font_handle
            );
            self.resource.tooltip_registered = true;
        }
        self
    }

    pub fn insert_component<T: Bundle>(&mut self, entity: Entity, components: T) {
        self.ui_root_node.commands().entity(entity).insert(components);
    }

    pub fn remove_component<T: Bundle>(&mut self, entity: Entity) {
        self.ui_root_node.commands().entity(entity).remove::<T>();
    }

    pub fn get_entity(&mut self) -> Entity {
        self.ui_root_node.id()
    }

    pub fn clean(&mut self) {
        let root_entity = self.get_entity();
        self.ui_root_node.commands().entity(root_entity).despawn_recursive();
    }
}

pub fn hot_reload_is_enabled(builder_res: Res<FamiqResource>) -> bool {
    builder_res.hot_reload_styles
}

pub fn hot_reload_is_disabled(builder_res: Res<FamiqResource>) -> bool {
    !builder_res.hot_reload_styles && !builder_res.external_style_applied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_style_from_external() {
        let mut local_style = WidgetStyle {
            color: Some("red".to_string()),
            ..default()
        };

        let external_style = WidgetStyle {
            color: Some("blue".to_string()),
            background_color: Some("yellow".to_string()),
            ..default()
        };

        // Update the local style with the external style
        local_style.from_external(&external_style);

        assert_eq!(
            local_style.color,
            Some("blue".to_string())
        );
        assert_eq!(
            local_style.background_color,
            Some("yellow".to_string()),
        );
    }

    #[test]
    fn test_widget_style_update_from() {
        let mut local_style = WidgetStyle {
            color: Some("red".to_string()),
            font_size: None,
            background_color: Some("white".to_string()),
            ..default()
        };

        let external_style = WidgetStyle {
            color: Some("blue".to_string()),
            font_size: Some("16px".to_string()),
            background_color: None,
            ..default()
        };

        // Update the local style with the external style
        local_style.update_from(&external_style);

        assert_eq!(
            local_style.color,
            Some("blue".to_string())
        );
        assert_eq!(
            local_style.font_size,
            Some("16px".to_string())
        );
        assert_eq!(
            local_style.background_color,
            None,
        );
    }
}
