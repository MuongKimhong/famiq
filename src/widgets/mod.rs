//! Famiq's built-in widgets.

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
pub mod progress_bar;
pub mod checkbox;
pub mod tests;
pub mod base_components;

use bevy::utils::hashbrown::HashMap;
pub use button::fa_button_builder;
pub use circular::fa_circular_builder;
pub use container::*;
pub use fps::fa_fps_builder;
pub use image::fa_image_builder;
pub use list_view::{fa_listview_builder, ListViewMovePanelEntity};
pub use modal::{fa_modal_builder, FaModalState, FaModalContainerEntity, IsFamiqModalBackground};
pub use text::fa_text_builder;
pub use text_input::fa_text_input_builder;
pub use selection::fa_selection_builder;
pub use bg_image::fa_bg_image_builder;
pub use progress_bar::fa_progress_bar_builder;
pub use checkbox::fa_checkbox_builder;
pub use base_components::*;
pub use style::*;
use crate::resources::*;
use crate::utils::get_text_size;
use crate::widgets::style_parse::*;
use bevy::ecs::system::{EntityCommands, SystemParam};
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub enum WidgetColor {
    #[default]
    Default, // White or Light
    Dark,
    Primary,
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark,
    Custom(String)
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum WidgetSize {
    #[default]
    Default,
    Small,
    Large,
    Custom(f32)
}

#[derive(Default, Clone)]
pub struct WidgetAttributes {
    pub id: Option<String>,
    pub class: Option<String>,
    pub node: Node,
    pub color: WidgetColor,
    pub size: WidgetSize,
    pub font_handle: Option<Handle<Font>>,
    pub image_handle: Option<Handle<Image>>,
    pub has_tooltip: bool,
    pub tooltip_text: String,
    pub bind_keys: Vec<String>,
    default_display_changed: bool,
    default_display: Display
}

pub trait SetWidgetAttributes: Sized {
    fn attributes(&mut self) -> &mut WidgetAttributes;

    fn bind(mut self, keys: &[&'static str]) -> Self {
        let keys: Vec<String> = keys.iter().map(|k| k.to_string()).collect();
        self.attributes().bind_keys = keys;
        self
    }

    fn id(mut self, id: &str) -> Self {
        self.attributes().id = Some(id.to_string());
        self
    }

    fn class(mut self, class: &str) -> Self {
        self.attributes().class = Some(class.to_string());
        self
    }

    fn color(mut self, color: &str) -> Self {
        self.attributes().color = WidgetColor::Custom(color.to_string());
        self
    }

    fn size(mut self, size: f32) -> Self {
        self.attributes().size = WidgetSize::Custom(size);
        self
    }

    fn tooltip(mut self, text: &str) -> Self {
        self.attributes().has_tooltip = true;
        self.attributes().tooltip_text = text.to_string();
        self
    }

    fn display(mut self, display: &str) -> Self {
        if let Some(parsed_display) = parse_display(display) {
            self.attributes().node.display = parsed_display;
            self.attributes().default_display_changed = true;
            self.attributes().default_display = parsed_display;
        }
        self
    }

    fn _node(&mut self);

    fn _process_built_in_color_class(&mut self) {
        if self.attributes().color != WidgetColor::Default {
            return;
        }
        let mut use_color = WidgetColor::Default;
        if let Some(class) = self.attributes().class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-dark" => use_color = WidgetColor::Dark,
                    "is-primary" => use_color = WidgetColor::Primary,
                    "is-primary-dark" => use_color = WidgetColor::PrimaryDark,
                    "is-secondary" => use_color = WidgetColor::Secondary,
                    "is-danger" => use_color = WidgetColor::Danger,
                    "is-danger-dark" => use_color = WidgetColor::DangerDark,
                    "is-success" => use_color = WidgetColor::Success,
                    "is-success-dark" => use_color= WidgetColor::SuccessDark,
                    "is-warning" => use_color = WidgetColor::Warning,
                    "is-warning-dark" => use_color = WidgetColor::WarningDark,
                    "is-info" => use_color = WidgetColor::Info,
                    "is-info-dark" => use_color = WidgetColor::InfoDark,
                    _ => {}
                }
            }
        }
        self.attributes().color = use_color;
    }

    fn _process_built_in_size_class(&mut self) {
        if self.attributes().size != WidgetSize::Default {
            return;
        }
        let mut use_size = WidgetSize::Default;
        if let Some(class) = self.attributes().class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-small" => use_size = WidgetSize::Small,
                    "is-large" => use_size = WidgetSize::Large,
                    _ => {}
                }
            }
        }
        self.attributes().size = use_size;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WidgetType {
    Root, // globalzindex 1
    Button,
    // Container,
    Text,
    FpsText, // globalzindex 6
    TextInput,
    ListView,
    Selection,
    Circular,
    ProgressBar,
    ToolTip, // globalzindex 4
    Modal, // globalzindex 5
    Image
}

pub struct FamiqBuilder<'a> {
    pub asset_server: &'a Res<'a, AssetServer>,
    pub ui_root_node: EntityCommands<'a>,
    pub resource: Mut<'a, FamiqResource>
}

impl<'a> FamiqBuilder<'a> {
    pub fn new(fa_query: &'a mut FaQuery, famiq_resource: &'a mut ResMut<FamiqResource>) -> Self {
        Self {
            asset_server: &fa_query.asset_server,
            ui_root_node: fa_query.commands.entity(famiq_resource.root_node_entity.unwrap()),
            resource: famiq_resource.reborrow()
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

pub fn hot_reload_is_enabled(famiq_res: Res<FamiqResource>) -> bool {
    famiq_res.hot_reload_styles
}

pub fn hot_reload_is_disabled(famiq_res: Res<FamiqResource>) -> bool {
    !famiq_res.hot_reload_styles && !famiq_res.external_style_applied
}

pub(crate) fn build_tooltip_node<'a>(
    attributes: &WidgetAttributes,
    root_node: &'a mut EntityCommands,
    widget_entity: Entity
) -> Entity {
    let txt_font = TextFont {
        font: attributes.font_handle.clone().unwrap(),
        font_size: get_text_size(&attributes.size),
        ..default()
    };
    let tooltip_entity = root_node
        .commands()
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(-28.0),
                width: Val::Auto,
                height: Val::Auto,
                display: Display::None,
                max_width: Val::Px(200.),
                padding: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    ..default()
                },
                margin: UiRect{
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            },
            GlobalZIndex(4),
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
            BorderRadius::all(Val::Px(5.0)),
            Transform::default(),
            Text::new(&attributes.tooltip_text),
            txt_font,
            TextColor(color::BLACK_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            IsFamiqTooltip
        ))
        .id();

    root_node
        .commands()
        .entity(widget_entity)
        .add_child(tooltip_entity)
        .insert(FamiqTooltipEntity(tooltip_entity));

    tooltip_entity
}

pub enum WidgetSelector<'a> {
    ID(&'a str),
    ENTITY(Entity)
}

/// Widget query
#[derive(QueryData)]
#[query_data(mutable)]
pub struct StyleQuery {
    background_color: &'static mut BackgroundColor,
    border_color: &'static mut BorderColor,
    border_radius: &'static mut BorderRadius,
    z_index: &'static mut ZIndex,
    visibility: &'static mut Visibility,
    box_shadow: &'static mut BoxShadow,
    node: &'static mut Node,
    id: Option<&'static FamiqWidgetId>,
}

/// Text query
#[derive(QueryData)]
#[query_data(mutable)]
pub struct TextStyleQuery {
    text_color: &'static mut TextColor,
    text_font: &'static mut TextFont,
    id: Option<&'static FamiqWidgetId>,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ContainableQuery {
    entity: Entity,
    listview_panel: Option<&'static ListViewMovePanelEntity>,
    modal_container: Option<&'static FaModalContainerEntity>,
    id: Option<&'static FamiqWidgetId>
}

#[derive(QueryData)]
pub struct ModalQuery {
    entity: Entity,
    id: Option<&'static FamiqWidgetId>
}

/// Famiq query
#[derive(SystemParam)]
pub struct FaQuery<'w, 's> {
    pub style_query: Query<'w, 's, StyleQuery>,
    pub text_style_query: Query<'w, 's, TextStyleQuery>,
    pub containable_query: Query<'w, 's, ContainableQuery, With<IsFamiqContainableWidget>>,
    pub modal_query: Query<'w, 's, ModalQuery, With<IsFamiqModalBackground>>,
    pub modal_state: ResMut<'w, FaModalState>,
    pub reactive_data: ResMut<'w, RData>,
    pub commands: Commands<'w, 's>,
    pub asset_server: Res<'w, AssetServer>
}

impl<'w, 's> FaQuery<'w, 's> {
    /// Get `StyleQueryItem` based on `WidgetSelector`
    pub fn get_style_mut(&mut self, selector: WidgetSelector) -> Option<StyleQueryItem<'_>> {
        match selector {
            WidgetSelector::ID(id) => self
                .style_query
                .iter_mut()
                .find_map(|result| {
                    result.id
                        .filter(|w_id| w_id.0 == id)
                        .map(|_| result)
                }),

            WidgetSelector::ENTITY(entity) => self.style_query.get_mut(entity).ok(),
        }
    }

    /// Get `TextStyleQueryItem` based on `WidgetSelector`
    pub fn get_text_style_mut(&mut self, selector: WidgetSelector) -> Option<TextStyleQueryItem<'_>> {
        match selector {
            WidgetSelector::ID(id) => self
                .text_style_query
                .iter_mut()
                .find_map(|result| {
                    result.id
                        .filter(|w_id| w_id.0 == id)
                        .map(|_| result)
                }),

            WidgetSelector::ENTITY(entity) => self.text_style_query.get_mut(entity).ok(),
        }
    }

    /// Finds a `ContainableQueryReadOnlyItem` based on `WidgetSelector`
    pub fn get_containable_item(&self, selector: WidgetSelector) -> Option<ContainableQueryReadOnlyItem<'_>> {
        match selector {
            WidgetSelector::ID(id) => self
                .containable_query
                .iter()
                .find_map(|result| {
                    result.id
                        .filter(|w_id| w_id.0 == id)
                        .map(|_| result)
                }),

            WidgetSelector::ENTITY(entity) => self.containable_query.get(entity).ok(),
        }
    }

    /// Finds a `ModalQueryItem` based on `WidgetSelector`
    pub fn get_modal_item(&self, selector: WidgetSelector) -> Option<ModalQueryItem<'_>> {
        match selector {
            WidgetSelector::ID(id) => self
                .modal_query
                .iter()
                .find_map(|result| {
                    result.id
                        .filter(|w_id| w_id.0 == id)
                        .map(|_| result)
                }),

            WidgetSelector::ENTITY(entity) => self.modal_query.get(entity).ok(),
        }
    }

    /// Add child/children to containable widget
    pub fn add_children(&mut self, selector: WidgetSelector, children: &[Entity]) {
        if let Some(item) = self.get_containable_item(selector) {
            if let Some(listview_panel_entity) = item.listview_panel {
                self.commands
                    .entity(listview_panel_entity.0)
                    .add_children(children);
                return;
            }

            if let Some(modal_container_entity) = item.modal_container {
                self.commands
                    .entity(modal_container_entity.0)
                    .add_children(children);
                return;
            }

            self.commands.entity(item.entity).add_children(children);
        }
    }

    /// Insert child/children to containable widget at given index
    pub fn insert_children(&mut self, selector: WidgetSelector, index: usize, children: &[Entity]) {
        if let Some(item) = self.get_containable_item(selector) {
            if let Some(listview_panel_entity) = item.listview_panel {
                self.commands
                    .entity(listview_panel_entity.0)
                    .insert_children(index, children);
                return;
            }

            if let Some(modal_container_entity) = item.modal_container {
                self.commands
                    .entity(modal_container_entity.0)
                    .insert_children(index, children);
                return;
            }

            self.commands.entity(item.entity).insert_children(index, children);
        }
    }

    /// Remove (despawn) children
    pub fn remove_children(&mut self, children: &[Entity]) {
        for child in children {
            self.commands.entity(*child).despawn();
        }
    }

    /// Show modal
    pub fn show_modal(&mut self, selector: WidgetSelector) {
        let mut entity: Option<Entity> = None;
        if let Some(item) = self.get_modal_item(selector) {
            entity = Some(item.entity);
        }
        if let Some(entity) = entity {
            self.modal_state.show_by_entity(entity);
        }
    }

    /// Hide modal
    pub fn hide_modal(&mut self, selector: WidgetSelector) {
        let mut entity: Option<Entity> = None;
        if let Some(item) = self.get_modal_item(selector) {
            entity = Some(item.entity);
        }
        if let Some(entity) = entity {
            self.modal_state.hide_by_entity(entity);
        }
    }

    /// Overwrite existing reactive data with new data.
    pub fn bind_new_data(&mut self, new_data: HashMap<String, RVal>) {
        self.reactive_data.data = new_data;
    }

    /// Insert new key-value into reactive data.
    pub fn insert_data(&mut self, key: &str, value: RVal) {
        self.reactive_data.data.insert(key.to_string(), value);
    }

    /// Explicitly mutates specific key.
    pub fn mutate_data(&mut self, key: &str, new_val: RVal) {
        let old_val = self.reactive_data.data.get(key);
        if old_val.is_none() {
            panic!("\n[FamiqError]: mutate_data, key {:?} not found\n", key);
        }
        let old_val = old_val.unwrap();
        if !RData::type_match(old_val, &new_val) {
            panic!("\n[FamiqError]: mutate_data, expected {:?} found {:?}\n", old_val, new_val);
        }
        if !self.reactive_data.changed_keys.contains(&key.to_string()) {
            self.reactive_data.changed_keys.push(key.to_string());
        }
        self.reactive_data.data.insert(key.to_string(), new_val);
    }

    /// Get value of provided key.
    pub fn get_data(&self, key: &str) -> Option<&RVal> {
        if let Some(val) = self.reactive_data.data.get(key) {
            return Some(val);
        }
        None
    }

    /// Get mutable value of provided key.
    pub fn get_data_mut(&mut self, key: &str) -> Option<&mut RVal> {
        if self.get_data(key).is_none() {
            return None;
        }
        if !self.reactive_data.changed_keys.contains(&key.to_string()) {
            self.reactive_data.changed_keys.push(key.to_string());
        }
        self.reactive_data.data.get_mut(key)
    }
}

#[macro_export]
macro_rules! children {
    ( $( $child:expr ),* $(,)? ) => {
        {
            let mut vec = Vec::new();
            $(
                let child = $child;
                vec.push(child);
            )*
            vec
        }
    };
}
