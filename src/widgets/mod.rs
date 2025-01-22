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

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::get_embedded_asset_path;

// key-value of "#widget-id"/".class-name" and all its styles in styles.json
pub type StyleKeyValue = HashMap<String, WidgetStyle>;
pub type StylesKeyValue = Vec<StyleKeyValue>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WidgetType {
    Button,
    Container,
    Text,
    FpsText,
    TextInput,
    ListView,
    ListViewItem,
    Selection,
    SelectionChoice,
    Circular,
    Modal,
    Image
}

#[derive(Resource)]
pub struct StylesKeyValueResource(pub StylesKeyValue);

impl StylesKeyValueResource {
    pub fn get_style_by_id(&self, widget_id: &str) -> Option<&WidgetStyle> {
        self.0.iter().flat_map(|map| map.get(widget_id)).next()
    }

    pub fn get_style_by_class_name(&self, class_name: &str) -> Option<&WidgetStyle> {
        self.0.iter().flat_map(|map| map.get(class_name)).next()
    }
}

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

#[derive(Resource)]
pub struct FamiqWidgetResource {
    // font path relative to project root
    pub font_path: String,

    // user external style (json) file path relative to project root
    pub style_path: String,

    // read external style (json) file and apply styles to widget every single frame
    pub hot_reload_styles: bool,

    pub widget_focus_state: HashMap<Entity, bool>,

    pub external_style_applied: bool
}

impl FamiqWidgetResource {
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
}

impl Default for FamiqWidgetResource {
    fn default() -> Self {
        Self {
            font_path: String::new(),
            style_path: String::new(),
            hot_reload_styles: false,
            widget_focus_state: HashMap::new(),
            external_style_applied: false
        }
    }
}

#[derive(Component)]
pub struct IsFaWidgetRoot;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub struct FamiqWidgetBuilder<'a> {
    pub asset_server: &'a ResMut<'a, AssetServer>,
    pub ui_root_node: EntityCommands<'a>,
    pub font_path: Option<String>,
    pub style_path: Option<String>
}

impl<'a> FamiqWidgetBuilder<'a> {
    fn _reset_builder_resource(builder_resource: &mut ResMut<FamiqWidgetResource>) {
        builder_resource.font_path = get_embedded_asset_path("embedded_assets/fonts/fira-mono-regular.ttf").to_string();
        builder_resource.style_path = "assets/styles.json".to_string();
        builder_resource.hot_reload_styles = false;
        builder_resource.external_style_applied = false;
    }

    pub fn new(
        commands: &'a mut Commands,
        builder_resource: &mut ResMut<FamiqWidgetResource>,
        asset_server: &'a ResMut<'a, AssetServer>,
    ) -> Self {
        Self::_reset_builder_resource(builder_resource);
        Self {
            asset_server,
            ui_root_node: Self::create_ui_root_node(commands),
            font_path: Some(get_embedded_asset_path("embedded_assets/fonts/fira-mono-regular.ttf").to_string()),
            style_path: Some("assets/styles.json".to_string())
        }
    }

    pub fn use_font_path(
        mut self,
        builder_resource: &mut ResMut<FamiqWidgetResource>,
        font_path: &str
    ) -> Self {
        self.font_path = Some(font_path.to_string());
        builder_resource.font_path = font_path.to_string();
        self
    }

    pub fn use_style_path(
        mut self,
        builder_resource: &mut ResMut<FamiqWidgetResource>,
        style_path: &str
    ) -> Self {
        let final_path = if style_path.starts_with("assets/") {
            style_path.to_string()
        } else {
            format!("assets/{}", style_path)
        };
        self.style_path = Some(final_path.clone());
        builder_resource.style_path = final_path;
        self
    }

    pub fn disable_hot_reload(self, builder_resource: &mut ResMut<FamiqWidgetResource>) -> Self {
        builder_resource.hot_reload_styles = false;
        self
    }

    pub fn enable_hot_reload(self, builder_resource: &mut ResMut<FamiqWidgetResource>) -> Self {
        builder_resource.hot_reload_styles = true;
        self
    }

    fn create_ui_root_node(commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            FamiqWidgetId("#fa_root".to_string()),
            IsFaWidgetRoot,
            GlobalZIndex(1)
        ))
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
