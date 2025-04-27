//! Famiq's built-in widgets.

pub mod scroll;
pub mod button;
pub mod color;
pub mod container;
pub mod fps;
pub mod selection;
pub mod style;
pub mod style_parse;
pub mod text;
pub mod text_input;
pub mod circular;
pub mod modal;
pub mod image;
pub mod progress_bar;
pub mod checkbox;
pub mod tests;
pub mod base_components;

pub(crate) use scroll::ScrollMovePanelEntity;
pub use base_components::*;

use crate::resources::*;
use crate::reactivity::*;
use crate::utils::*;

use bevy::ecs::system::{EntityCommands, SystemParam};
use bevy::platform::collections::HashMap;
use bevy::ecs::query::QueryData;
use std::cell::RefCell;
use bevy::prelude::*;

pub trait SetupWidget {
    /// get components required for widget.
    fn components(&mut self) -> impl Bundle;

    /// build/spawn the widget into UI world.
    fn build(
        &mut self,
        reactive_data: &HashMap<String, RVal>,
        commands: &mut Commands
    ) -> Entity;

    /// build/spawn the widget into UI world using world instead of commands.
    fn rebuild(
        &mut self,
        reactive_data: &HashMap<String, RVal>,
        old_entity: Entity,
        world: &mut World
    );
}

/// Built-in color variants. These colors can be set via class.
#[derive(Clone, Default, PartialEq, Debug)]
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
    Transparent,
    Custom(String),
    CustomSrgba((f32, f32, f32, f32))
}

/// Built-in size variants. These sizes can be set via class.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum WidgetSize {
    #[default]
    Default,
    Small,
    Large,
    Custom(f32)
}

#[derive(Default, Clone, Debug)]
pub struct WidgetAttributes {
    pub id: Option<String>,
    pub class: Option<String>,
    pub node: Node,
    pub color: WidgetColor,
    pub size: WidgetSize,
    pub width: Option<String>,
    pub height: Option<String>,
    pub display: Option<String>,
    pub font_handle: Option<Handle<Font>>,
    pub image_handle: Option<Handle<Image>>,
    pub has_tooltip: bool,
    pub tooltip_text: String,
    pub bind_keys: Vec<String>,
    pub model_key: Option<String>,
    pub(crate) default_visibility: Visibility,
    pub(crate) default_z_index: ZIndex,
    pub(crate) overrided_background_color: Option<Color>,
    pub(crate) overrided_border_color: Option<Color>,
    pub(crate) override_text_size: Option<f32>
}

pub trait SetWidgetAttributes: Sized {
    fn attributes(&mut self) -> &mut WidgetAttributes;

    fn cloned_attrs(&mut self) -> &mut WidgetAttributes;

    fn bind<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.attributes().bind_keys = values.into_iter().map(Into::into).collect();
        self
    }

    fn set_model(&mut self, model_key: &str) {
        self.attributes().model_key = Some(model_key.to_string());
    }

    fn set_id(&mut self, id: &str) {
        self.attributes().id = Some(id.to_string());
    }

    fn set_class(&mut self, class: &str) {
        self.attributes().class = Some(class.to_string());
    }

    fn set_color(&mut self, color: &str) {
        self.attributes().color = WidgetColor::Custom(color.to_string());
    }

    fn set_size(&mut self, size: f32) {
        self.attributes().size = WidgetSize::Custom(size);
    }

    fn set_width(&mut self, width: &str) {
        self.attributes().width = Some(width.to_string());
    }

    fn set_height(&mut self, height: &str) {
        self.attributes().height = Some(height.to_string());
    }

    fn set_display(&mut self, display: &str) {
        self.attributes().display = Some(display.to_string());
    }

    fn set_tooltip(&mut self, text: &str) {
        self.attributes().has_tooltip = true;
        self.attributes().tooltip_text = text.to_string();
    }

    fn _process_built_in_color_class(&mut self) {
        if self.cloned_attrs().color != WidgetColor::Default {
            return;
        }
        let mut use_color = WidgetColor::Default;
        if let Some(class) = self.cloned_attrs().class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "dark" => use_color = WidgetColor::Dark,
                    "primary" => use_color = WidgetColor::Primary,
                    "primary-dark" => use_color = WidgetColor::PrimaryDark,
                    "secondary" => use_color = WidgetColor::Secondary,
                    "danger" => use_color = WidgetColor::Danger,
                    "danger-dark" => use_color = WidgetColor::DangerDark,
                    "success" => use_color = WidgetColor::Success,
                    "success-dark" => use_color= WidgetColor::SuccessDark,
                    "warning" => use_color = WidgetColor::Warning,
                    "warning-dark" => use_color = WidgetColor::WarningDark,
                    "info" => use_color = WidgetColor::Info,
                    "info-dark" => use_color = WidgetColor::InfoDark,
                    _ => {}
                }
            }
        }
        self.cloned_attrs().color = use_color;
    }

    fn _process_built_in_size_class(&mut self) {
        if self.cloned_attrs().size != WidgetSize::Default {
            return;
        }
        let mut use_size = WidgetSize::Default;
        if let Some(class) = self.cloned_attrs().class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "small" => use_size = WidgetSize::Small,
                    "large" => use_size = WidgetSize::Large,
                    _ => {}
                }
            }
        }
        self.cloned_attrs().size = use_size;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub enum WidgetType {
    Root, // globalzindex 1
    Button,
    Container,
    Text,
    FpsText, // globalzindex 6
    TextInput,
    Scroll,
    Selection,
    Circular,
    ProgressBar,
    ToolTip, // globalzindex 4
    Modal, // globalzindex 5
    Image,
    BackgroudImage
}

/// Root builder, allows access to AssetServer, root_node, FamiqResource and RData.
pub struct FamiqBuilder<'a> {
    pub asset_server: &'a Res<'a, AssetServer>,
    pub ui_root_node: EntityCommands<'a>,
    pub resource: Mut<'a, FamiqResource>,
    pub reactive_data: Mut<'a, RData>,
    // pub containable_children: Mut<'a, ContainableChildren>
}

impl<'a> FamiqBuilder<'a> {
    /// Create new root builder.
    pub fn new(fa_query: &'a mut FaQuery, famiq_resource: &'a mut ResMut<FamiqResource>) -> Self {
        Self {
            asset_server: &fa_query.asset_server,
            ui_root_node: fa_query.commands.entity(famiq_resource.root_node_entity.unwrap()),
            resource: famiq_resource.reborrow(),
            reactive_data: fa_query.reactive_data.reborrow(),
            // containable_children: fa_query.containable_children.reborrow()
        }
    }

    /// Inject root builder for global access.
    pub fn inject(self) {
        let boxed = Box::new(self);
        let raw = Box::into_raw(boxed); // *mut FamiqBuilder<'a>
        inject_builder(raw as *mut ());
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
    /// # Argument for native build
    /// * style_path: Full path to the json file, relative to root directory.
    ///
    /// # Argument for wasm build
    /// * style_path: Full path to json file, relative to assets directory.
    pub fn use_style_path(mut self, style_path: &str) -> Self {
        self.resource.style_path = style_path.to_string();
        self
    }

    /// Method to enable hot-reload.
    /// Should be called only for native builds (exclude wasm).
    pub fn hot_reload(mut self) -> Self {
        self.resource.hot_reload_styles = true;
        self
    }

    /// Method to get font handle from provided font_path.
    pub fn get_font_handle(&self) -> Handle<Font> {
        self.asset_server.load(&self.resource.font_path)
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
        self.ui_root_node.commands().entity(root_entity).despawn();
    }
}

pub fn hot_reload_is_enabled(famiq_res: Res<FamiqResource>) -> bool {
    famiq_res.hot_reload_styles
}

pub fn hot_reload_is_disabled(famiq_res: Res<FamiqResource>) -> bool {
    !famiq_res.hot_reload_styles && !famiq_res.external_style_applied
}

pub(crate) fn build_tooltip_node(
    attributes: &WidgetAttributes,
    commands: &mut Commands,
    widget_entity: Entity
) -> Entity {
    let txt_font = TextFont {
        font: attributes.font_handle.clone().unwrap(),
        font_size: get_text_size(&attributes.size),
        ..default()
    };
    let tooltip_entity = commands
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

    commands
        .entity(widget_entity)
        .add_child(tooltip_entity)
        .insert(TooltipEntity(tooltip_entity));

    tooltip_entity
}

pub enum WidgetSelector<'a> {
    ID(&'a str),
    ENTITY(Entity)
}

/// Widget style query
#[derive(QueryData)]
#[query_data(mutable)]
pub struct StyleQuery {
    pub background_color: &'static mut BackgroundColor,
    pub border_color: &'static mut BorderColor,
    pub border_radius: &'static mut BorderRadius,
    pub z_index: &'static mut ZIndex,
    pub visibility: &'static mut Visibility,
    pub box_shadow: &'static mut BoxShadow,
    pub node: &'static mut Node,
    pub id: Option<&'static WidgetId>,
    pub class: Option<&'static WidgetClasses>,
    pub default_style: &'static DefaultWidgetConfig
}

/// Text style query
#[derive(QueryData)]
#[query_data(mutable)]
pub struct TextStyleQuery {
    pub text_color: &'static mut TextColor,
    pub text_font: &'static mut TextFont,
    pub id: Option<&'static WidgetId>,
    pub class: Option<&'static WidgetClasses>,
    pub default_text_style: Option<&'static DefaultTextConfig>,
    pub default_text_span_style: Option<&'static DefaultTextSpanConfig>,
}

/// Query for getting/updating widget's styles and text's styles.
#[derive(SystemParam)]
pub struct FaStyleQuery<'w, 's> {
    pub style_query: Query<'w, 's, StyleQuery>,
    pub text_style_query: Query<'w, 's, TextStyleQuery>,
}

impl<'w, 's> FaStyleQuery<'w, 's> {
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
}

/// Containable query for `container!`, `modal!` and `scroll!`.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct ContainableQuery {
    entity: Entity,
    scroll_panel: Option<&'static ScrollMovePanelEntity>,
    id: Option<&'static WidgetId>
}

/// Famiq main query
#[derive(SystemParam)]
pub struct FaQuery<'w, 's> {
    pub containable_query: Query<'w, 's, ContainableQuery, With<IsFamiqContainableWidget>>,
    pub reactive_data: ResMut<'w, RData>,
    pub commands: Commands<'w, 's>,
    pub asset_server: Res<'w, AssetServer>,
    pub reactive_subscriber: ResMut<'w, RSubscriber>,
    // pub containable_children: ResMut<'w, ContainableChildren>
}

impl<'w, 's> FaQuery<'w, 's> {
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

    /// Add child/children to containable widget
    pub fn add_children(&mut self, selector: WidgetSelector, children: &[Entity]) {
        if let Some(item) = self.get_containable_item(selector) {
            if let Some(panel_entity) = item.scroll_panel {
                self.commands
                    .entity(panel_entity.0)
                    .add_children(children);
                return;
            }
            self.commands.entity(item.entity).add_children(children);
        }
    }

    /// Insert child/children to containable widget at given index
    pub fn insert_children(&mut self, selector: WidgetSelector, index: usize, children: &[Entity]) {
        if let Some(item) = self.get_containable_item(selector) {
            if let Some(panel_entity) = item.scroll_panel {
                self.commands
                    .entity(panel_entity.0)
                    .insert_children(index, children);
                return;
            }
            self.commands.entity(item.entity).insert_children(index, children);
        }
    }

    /// Remove (despawn) children from containable widget.
    pub fn remove_children(&mut self, children: &[Entity]) {
        for child in children {
            self.commands.entity(*child).despawn();
        }
    }

    /// Insert new key-value into reactive data.
    pub fn insert_data(&mut self, key: &str, value: RVal) {
        self.reactive_data.data.insert(key.to_string(), value);
    }

    /// Insert into reactive data as RVal::Str
    pub fn insert_str(&mut self, key: &str, value: impl Into<String>) {
        self.insert_data(key, RVal::Str(value.into()));
    }

    /// Insert into reactive data as RVal::None
    pub fn insert_none(&mut self, key: &str) {
        self.insert_data(key, RVal::None);
    }

    /// Insert into reactive data as Rval::Num
    pub fn insert_num(&mut self, key: &str, value: i32) {
        self.insert_data(key, RVal::Num(value));
    }

    /// Insert into reactive data as Rval::Bool
    pub fn insert_bool(&mut self, key: &str, value: bool) {
        self.insert_data(key, RVal::Bool(value));
    }

    /// Insert into reactive data as Rval::FNum
    pub fn insert_fnum(&mut self, key: &str, value: f32) {
        self.insert_data(key, RVal::FNum(value));
    }

    /// Insert into reactive data as `Rval::List<String>`
    pub fn insert_str_list(&mut self, key: &str, value: Vec<String>) {
        self.insert_data(key, RVal::List(value));
    }

    /// Explicitly mutates specific key.
    pub fn mutate_data(&mut self, key: &str, new_val: RVal) {
        let old_val = self.reactive_data.data.get(key);
        if old_val.is_none() {
            panic!("\n[FamiqError]: mutate_data, key {:?} not found\n", key);
        }
        if !self.reactive_data.changed_keys.contains(&key.to_string()) {
            self.reactive_data.changed_keys.push(key.to_string());
        }
        self.reactive_data.data.insert(key.to_string(), new_val);
    }

    /// Explicitly mutates specific key as RVal::Str
    pub fn mutate_str(&mut self, key: &str, new_str: &str) {
        self.mutate_data(key, RVal::Str(new_str.into()));
    }

    /// Explicitly mutates specific key as RVal::Num
    pub fn mutate_num(&mut self, key: &str, new_num: i32) {
        self.mutate_data(key, RVal::Num(new_num));
    }

    /// Explicitly mutates specific key as RVal::FNum
    pub fn mutate_fnum(&mut self, key: &str, new_fnum: f32) {
        self.mutate_data(key, RVal::FNum(new_fnum));
    }

    /// Explicitly mutates specific key as RVal::Bool
    pub fn mutate_bool(&mut self, key: &str, new_bool: bool) {
        self.mutate_data(key, RVal::Bool(new_bool));
    }

    /// Explicitly mutates specific key as RVal::None
    pub fn mutate_none(&mut self, key: &str) {
        self.mutate_data(key, RVal::None);
    }

    /// Explicitly mutates specific key as RVal::List<String>
    pub fn mutate_str_list(&mut self, key: &str, new_list: Vec<String>) {
        self.mutate_data(key, RVal::List(new_list));
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

/// Macro to extract children's entities from children attributes.
#[macro_export]
macro_rules! extract_children {
    // For children: [ item1, item2, item3 ]
    ($vec:ident, children: [ $( $child:expr ),* $(,)? ] $(, $($rest:tt)*)?) => {{
        $(
            $vec.push($child);
        )*
        $(
            $crate::extract_children!($vec, $($rest)*);
        )?
    }};

    // For children: vec!
    ($vec:ident, children: $children_vec:expr $(, $($rest:tt)*)?) => {{
        $vec.extend($children_vec);
        $(
            $crate::extract_children!($vec, $($rest)*);
        )?
    }};

    // other keys
    ($vec:ident, $key:ident : $val:expr $(, $($rest:tt)*)?) => {{
        $(
            $crate::extract_children!($vec, $builder, $($rest)*);
        )?
    }};
    ($vec:ident,) => {{}};
}

/// Macro for setting common attributes to a widget.
#[macro_export]
macro_rules! common_attributes {
    ( $builder:ident, $key:ident : $value:expr ) => {{
        match stringify!($key) {
            "id" => $builder.set_id($value),
            "class" => $builder.set_class($value),
            "color" => $builder.set_color($value),
            "tooltip" => $builder.set_tooltip($value),
            "width" => $builder.set_width($value),
            "height" => $builder.set_height($value),
            "display" => $builder.set_display($value),
            _ => {}
        }
    }};
}

/// Represent different type of widget's builder.
#[derive(Clone, Debug)]
pub enum BuilderType {
    Text(text::TextBuilder),
    Button(button::ButtonBuilder),
    Checkbox(checkbox::CheckboxBuilder),
    Circular(circular::CircularBuilder),
    Container(container::ContainerBuilder),
    Fps(fps::FpsBuilder),
    Image(image::ImageBuilder),
    Modal(modal::ModalBuilder),
    ProgressBar(progress_bar::ProgressBarBuilder),
    Selection(selection::SelectionBuilder),
    Scroll(scroll::ScrollBuilder)
}

#[derive(Clone, Debug)]
pub struct WidgetBuilder {
    pub builder: BuilderType
}

thread_local! {
    static GLOBAL_BUILDER: RefCell<Option<*mut ()>> = RefCell::new(None);
}

/// Inject root builder for global access.
pub fn inject_builder(ptr: *mut ()) {
    GLOBAL_BUILDER.with(|cell| {
        *cell.borrow_mut() = Some(ptr);
    });
}

/// Access to mutable reference root builder.
pub fn builder_mut<'a>() -> &'a mut FamiqBuilder<'a> {
    GLOBAL_BUILDER.with(|cell| {
        let ptr = cell
            .borrow()
            .expect("Can't access global widget builder!") as *mut FamiqBuilder<'a>;
        unsafe { &mut *ptr }
    })
}
