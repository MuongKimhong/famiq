pub mod button;
pub mod color;
pub mod container;
pub mod list_view;
pub mod selection;
pub mod style;
pub mod style_parse;
pub mod text;
pub mod text_input;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// key-value of "#widget-id" and all its styles in styles.json
pub type StyleKeyValue = HashMap<String, WidgetStyle>;
pub type StylesKeyValue = Vec<StyleKeyValue>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WidgetType {
    Button,
    Container,
    Text,
    TextInput,
    ListView,
    ListViewItem,
    Selection,
    SelectionItem,
}

#[derive(Resource)]
pub struct StylesKeyValueResource(pub StylesKeyValue);

#[derive(Component, Deref)]
pub struct FamiqWidgetId(pub String);

#[derive(Component)]
// use to store default bundle of each widget created by user
pub struct DefaultWidgetBundle(pub FaWidgetBundle);

#[derive(Component)]
// use to store default text bundle of widget created by user
pub struct DefaultTextBundle(pub TextBundle);

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

impl StylesKeyValueResource {
    pub fn get_style_by_id(&self, btn_id: &str) -> Option<&WidgetStyle> {
        self.0.iter().flat_map(|map| map.get(btn_id)).next()
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct FaWidgetBundle {
    pub node: Node,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub background_color: BackgroundColor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub inherited_visibility: InheritedVisibility,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub z_index: ZIndex,
}

impl Default for FaWidgetBundle {
    fn default() -> Self {
        Self {
            node: Default::default(),
            style: Default::default(),
            interaction: Default::default(),
            focus_policy: FocusPolicy::Block,
            border_color: Default::default(),
            border_radius: Default::default(),
            background_color: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            inherited_visibility: Default::default(),
            visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct IsFaWidgetRoot;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetStyle {
    pub color: Option<String>,     // for TextBundle's text color only
    pub font_size: Option<String>, // for TextBundle's text font_size only
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
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            },
            FamiqWidgetId("#fa_root".to_string()),
            IsFaWidgetRoot,
        ))
    }

    pub fn fa_container(&mut self, id: &str, children: &Vec<Entity>) -> Entity {
        container::FaContainer::new(id, &mut self.ui_root_node, children, None)
    }

    pub fn fa_button(
        &mut self,
        id: &str,
        text: &str,
        variant: Option<button::BtnVariant>,
        size: Option<button::BtnSize>,
    ) -> Entity {
        button::FaButton::normal_btn(
            id,
            text,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            variant,
            size,
        )
    }

    pub fn fa_text(&mut self, id: &str, value: &str) -> Entity {
        text::fa_text(
            id,
            value,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            None,
            None,
        )
    }

    pub fn fa_text_input(
        &mut self,
        id: &str,
        placeholder: &str,
        size: Option<text_input::TextInputSize>,
        variant: Option<text_input::TextInputVariant>,
    ) -> Entity {
        let mut use_variant = text_input::TextInputVariant::Default;

        if let Some(v) = variant {
            use_variant = v;
        }
        text_input::FaTextInput::fa_text_input(
            id,
            placeholder,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            size,
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
        items: &Vec<String>,
        label: Option<&str>,
        size: Option<selection::SelectionSize>,
        variant: Option<selection::SelectorVariant>,
    ) -> Entity {
        let mut use_variant = selection::SelectorVariant::Default;

        match variant {
            Some(v) => use_variant = v,
            None => (),
        }
        selection::FaSelection::build_selection(
            id,
            placeholder,
            label,
            &mut self.ui_root_node,
            self.asset_server,
            &self.font_path,
            size,
            items,
            use_variant,
        )
    }
}
