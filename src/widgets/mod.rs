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

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use button::{BtnSize, BtnVariant, FaButton};
use circular::{CircularVariant, CircularSize, FaCircular};
use image::FaImage;
use modal::FaModal;
use selection::{SelectionSize, SelectorVariant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use text_input::{TextInputSize, TextInputVariant};

// key-value of "#widget-id" and all its styles in styles.json
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
}

#[derive(Component, Deref)]
pub struct FamiqWidgetId(pub String);

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
pub struct FamiqWidgetBuilderResource {
    // font path relative to project root
    pub font_path: String,

    // user external style (json) file path relative to project root
    pub style_path: String,

    // read external style (json) file and apply styles to widget every single frame
    pub hot_reload_styles: bool,
}

impl Default for FamiqWidgetBuilderResource {
    fn default() -> Self {
        Self {
            font_path: String::new(),
            style_path: String::new(),
            hot_reload_styles: false,
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
}

pub struct FamiqWidgetBuilder<'a> {
    pub asset_server: &'a ResMut<'a, AssetServer>,
    pub ui_root_node: EntityCommands<'a>,
    pub font_path: String,
    pub style_path: String,
}

impl<'a> FamiqWidgetBuilder<'a> {
    fn _update_builder_resource(
        font_path: &str,
        style_path: &str,
        hot_reload_styles: bool,
        builder_resource: &mut ResMut<FamiqWidgetBuilderResource>,
    ) {
        builder_resource.font_path = font_path.to_string();
        builder_resource.style_path = style_path.to_string();
        builder_resource.hot_reload_styles = hot_reload_styles;
    }

    pub fn new(
        commands: &'a mut Commands,
        asset_server: &'a ResMut<'a, AssetServer>,
        builder_resource: &mut ResMut<FamiqWidgetBuilderResource>,
        font_path: &str,  // font path relative to project root
        style_path: &str, // style path relative to project root
        hot_reload_styles: bool,
    ) -> Self {
        Self::_update_builder_resource(font_path, style_path, hot_reload_styles, builder_resource);

        Self {
            font_path: font_path.to_string(),
            style_path: style_path.to_string(),
            asset_server,
            ui_root_node: Self::create_ui_root_node(commands),
        }
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

    pub fn fa_container(&mut self, id: &str, children: &Vec<Entity>) -> Entity {
        container::FaContainer::new(id, &mut self.ui_root_node, children)
    }

    pub fn fa_button(&mut self, id: &str, text: &str, variant: &str, size: &str) -> Entity {
        let use_variant;
        let use_size;

        match variant.trim().to_lowercase().as_str() {
            "primary" => use_variant = BtnVariant::Primary,
            "secondary" => use_variant = BtnVariant::Secondary,
            "danger" => use_variant = BtnVariant::Danger,
            "success" => use_variant = BtnVariant::Success,
            "warning" => use_variant = BtnVariant::Warning,
            "info" => use_variant = BtnVariant::Info,
            _ => use_variant = BtnVariant::Default,
        }

        match size.trim().to_lowercase().as_str() {
            "small" => use_size = BtnSize::Small,
            "large" => use_size = BtnSize::Large,
            _ => use_size = BtnSize::Normal,
        }

        FaButton::new(
            id,
            text,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            use_variant,
            use_size,
        )
    }

    pub fn fa_text(&mut self, id: &str, value: &str) -> Entity {
        text::fa_text(
            id,
            value,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
        )
    }

    pub fn fa_fps(&mut self, id: &str, change_color: bool) -> Entity {
        fps::FaFpsText::new(
            id,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            change_color
        )
    }

    pub fn fa_text_input(
        &mut self,
        id: &str,
        placeholder: &str,
        variant: &str,
        size: &str,
    ) -> Entity {
        let use_variant;
        let use_size;

        match variant.trim().to_lowercase().as_str() {
            "underlined" => use_variant = TextInputVariant::Underlined,
            "outlined" => use_variant = TextInputVariant::Outlined,
            _ => use_variant = TextInputVariant::Default,
        }
        match size.trim().to_lowercase().as_str() {
            "small" => use_size = TextInputSize::Small,
            "large" => use_size = TextInputSize::Large,
            _ => use_size = TextInputSize::Normal,
        }

        text_input::FaTextInput::new(
            id,
            placeholder,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            use_size,
            use_variant,
        )
    }

    pub fn fa_list_view(&mut self, id: &str, items: &Vec<Entity>) -> Entity {
        list_view::FaListView::new(id, &mut self.ui_root_node, items)
    }

    pub fn fa_selection(
        &mut self,
        id: &str,
        placeholder: &str,
        choices: &Vec<String>,
        label: &str,
        variant: &str,
        size: &str,
    ) -> Entity {
        let use_variant;
        let use_size;
        let use_label;

        match variant.trim().to_lowercase().as_str() {
            "underlined" => use_variant = SelectorVariant::Underlined,
            "outlined" => use_variant = SelectorVariant::Outlined,
            _ => use_variant = SelectorVariant::Default
        }
        match size.trim().to_lowercase().as_str() {
            "small" => use_size = SelectionSize::Small,
            "large" => use_size = SelectionSize::Large,
            _ => use_size = SelectionSize::Normal
        }
        if label.trim().is_empty() {
            use_label = None;
        }
        else {
            use_label = Some(label);
        }

        selection::FaSelection::new(
            id,
            placeholder,
            use_label,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            use_variant,
            use_size,
            choices,
        )
    }

    pub fn fa_circular(
        &mut self,
        id: &str,
        variant: &str,
        size: &str
    ) -> Entity {
        let use_variant;
        let use_size;

        match variant.trim().to_lowercase().as_str() {
            "primary" => use_variant = CircularVariant::Primary,
            "secondary" => use_variant = CircularVariant::Secondary,
            "danger" => use_variant = CircularVariant::Danger,
            "success" => use_variant = CircularVariant::Success,
            "warning" => use_variant = CircularVariant::Warning,
            "info" => use_variant = CircularVariant::Info,
            _ => use_variant = CircularVariant::Default,
        }
        match size.trim().to_lowercase().as_str() {
            "small" => use_size = CircularSize::Small,
            "large" => use_size = CircularSize::Large,
            _ => use_size = CircularSize::Normal
        }

        FaCircular::new(id, &mut self.ui_root_node, use_variant, use_size)
    }

    pub fn fa_image(&mut self, id: &str, path: &str) -> Entity {
        FaImage::new(id, path, &mut self.ui_root_node, self.asset_server)
    }

    // doesn't need to return Entity
    pub fn fa_modal(&mut self, id: &str, items: &Vec<Entity>) {
        FaModal::new(id, items, &mut self.ui_root_node);
    }
}
