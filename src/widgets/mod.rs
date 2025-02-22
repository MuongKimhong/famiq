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
pub mod tooltip;
pub mod progress_bar;
pub mod tests;
pub mod base_components;

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
pub use base_components::*;
use crate::widgets::style_parse::*;
use std::marker::PhantomData;
use tooltip::FaToolTip;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::utils::get_embedded_asset_path;

#[derive(Clone, Default)]
pub struct ContainableData {
    pub entity: Option<Entity>,
    pub children: Vec<Entity>
}

#[derive(PartialEq, Clone, Default)]
enum ContainableMethodCall {
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
    method_called: ContainableMethodCall,
    changed_container: Option<Entity>,
    to_use_children: Vec<Entity>,
    insert_index: usize,
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

#[derive(Clone, Default, PartialEq)]
pub enum WidgetColor {
    #[default]
    Default,
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
    default_display_changed: bool,
    default_display: Display
}

pub trait SetWidgetAttributes: Sized {
    fn attributes(&mut self) -> &mut WidgetAttributes;

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
    ProgressBar,
    ToolTip, // globalzindex 4
    Modal, // globalzindex 5
    Image
}

#[derive(Resource, Default)]
pub struct StylesKeyValueResource {
    pub values: HashMap<String, WidgetStyle>, // key-value of "#widget-id"/".class-name" and all its styles in styles.json
    changed_key: Vec<String>
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

pub struct FamiqBuilder<'a> {
    pub asset_server: &'a ResMut<'a, AssetServer>,
    pub ui_root_node: EntityCommands<'a>,
    pub resource: Mut<'a, FamiqResource>
}

impl<'a> FamiqBuilder<'a> {
    pub fn new(
        commands: &'a mut Commands,
        builder_resource: &'a mut ResMut<FamiqResource>,
        asset_server: &'a ResMut<'a, AssetServer>,
    ) -> Self {
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

pub fn hot_reload_is_enabled(famiq_res: Res<FamiqResource>) -> bool {
    famiq_res.hot_reload_styles
}

pub fn hot_reload_is_disabled(famiq_res: Res<FamiqResource>) -> bool {
    !famiq_res.hot_reload_styles && !famiq_res.external_style_applied
}
