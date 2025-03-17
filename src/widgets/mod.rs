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
pub use checkbox::fa_checkbox;
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

/// Famiq query
#[derive(SystemParam)]
pub struct FaQuery<'w, 's> {
    pub style_query: Query<'w, 's, StyleQuery>,
    pub text_style_query: Query<'w, 's, TextStyleQuery>,
    pub commands: Commands<'w, 's>,
    pub asset_server: Res<'w, AssetServer>
}

impl<'w, 's> FaQuery<'w, 's> {
    /// Finds a `StyleQueryItem` based on `WidgetSelector`
    pub fn get_style_item(&mut self, selector: WidgetSelector) -> Option<StyleQueryItem<'_>> {
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

    /// Finds a `TextStyleQueryItem` based on `WidgetSelector`
    pub fn get_style_item_with_text(&mut self, selector: WidgetSelector) -> Option<TextStyleQueryItem<'_>> {
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

    /// Directly set z-index to widget
    pub fn set_z_index(&mut self, selector: WidgetSelector, z_index: i32) {
        if let Some(mut item) = self.get_style_item(selector) {
            *item.z_index = ZIndex(z_index);
        }
    }

    /// Directly set visibility to widget
    pub fn set_visibility(&mut self, selector: WidgetSelector, vis: Visibility) {
        if let Some(mut item) = self.get_style_item(selector) {
            *item.visibility = vis;
        }
    }

    /// Directly set background color to widget
    pub fn set_background_color(&mut self, selector: WidgetSelector, color: Color) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.background_color.0 = color;
        }
    }

    /// Directly set shadow color to widget
    pub fn set_shadow_color(&mut self, selector: WidgetSelector, color: Color) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.box_shadow.color = color;
        }
    }

    /// Directly set shadow blur radius
    pub fn set_shadow_blur(&mut self, selector: WidgetSelector, radius: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.box_shadow.blur_radius = radius;
        }
    }

    /// Directly set shadow spread radius
    pub fn set_shadow_spread(&mut self, selector: WidgetSelector, radius: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.box_shadow.spread_radius = radius;
        }
    }

    /// Directly set shadow x offset
    pub fn set_shadow_x_offset(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.box_shadow.x_offset = val;
        }
    }

    /// Directly set shadow y offset
    pub fn set_shadow_y_offset(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.box_shadow.y_offset = val;
        }
    }

    /// Directly set shadow offset for both x and y
    pub fn set_shadow_offset(&mut self, selector: WidgetSelector, offset: (Val, Val)) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.box_shadow.x_offset = offset.0;
            item.box_shadow.y_offset = offset.1;
        }
    }

    /// Directly set border color to widget
    pub fn set_border_color(&mut self, selector: WidgetSelector, color: Color) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.border_color.0 = color;
        }
    }

    /// Directly set margin to widget
    pub fn set_margin(&mut self, selector: WidgetSelector, margin: UiRect) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.margin = margin;
        }
    }

    /// Directly set margin-left to widget
    pub fn set_margin_left(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.margin.left = val;
        }
    }

    /// Directly set margin-right to widget
    pub fn set_margin_right(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.margin.right = val;
        }
    }

    /// Directly set margin-top to widget
    pub fn set_margin_top(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.margin.top = val;
        }
    }
    /// Directly set margin-bottom to widget
    pub fn set_margin_bottom(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.margin.bottom = val;
        }
    }

    /// Directly set padding to widget
    pub fn set_padding(&mut self, selector: WidgetSelector, padding: UiRect) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.padding = padding;
        }
    }

    /// Directly set padding-left to widget
    pub fn set_padding_left(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.padding.left = val;
        }
    }

    /// Directly set padding-right to widget
    pub fn set_padding_right(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.padding.right = val;
        }
    }

    /// Directly set padding-top to widget
    pub fn set_padding_top(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.padding.top = val;
        }
    }

    /// Directly set border to widget
    pub fn set_border(&mut self, selector: WidgetSelector, border: UiRect) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.border = border;
        }
    }

    /// Directly set border-left to widget
    pub fn set_border_left(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.border.left = val;
        }
    }

    /// Directly set border-right to widget
    pub fn set_border_right(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.border.right = val;
        }
    }

    /// Directly set border-top to widget
    pub fn set_border_top(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.border.top = val;
        }
    }

    /// Directly set border-bottom to widget
    pub fn set_border_bottom(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.border.bottom = val;
        }
    }

    /// Directly set display to widget
    pub fn set_display(&mut self, selector: WidgetSelector, display: Display) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.display = display;
        }
    }

    /// Directly set border radius to widget (top-left, top-right, bottom-left, bottom-right)
    pub fn set_border_radius(&mut self, selector: WidgetSelector, radius: (Val, Val, Val, Val)) {
        if let Some(mut item) = self.get_style_item(selector) {
            *item.border_radius = BorderRadius {
                top_left: radius.0,
                top_right: radius.1,
                bottom_left: radius.2,
                bottom_right: radius.3,
            };
        }
    }

    /// Directly set top-left border radius to widget
    pub fn set_border_radius_top_left(&mut self, selector: WidgetSelector, radius: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.border_radius.top_left = radius;
        }
    }

    /// Directly set top-right border radius to widget
    pub fn set_border_radius_top_right(&mut self, selector: WidgetSelector, radius: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.border_radius.top_right = radius;
        }
    }

    /// Directly set bottom-left border radius to widget
    pub fn set_border_radius_bottom_left(&mut self, selector: WidgetSelector, radius: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.border_radius.bottom_left = radius;
        }
    }

    /// Directly set bottom-right border radius to widget
    pub fn set_border_radius_bottom_right(&mut self, selector: WidgetSelector, radius: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.border_radius.bottom_right = radius;
        }
    }

    /// Directly set text's color
    pub fn set_color(&mut self, selector: WidgetSelector, color: Color) {
        if let Some(mut item) = self.get_style_item_with_text(selector) {
            item.text_color.0 = color;
        }
    }

    /// Directly set text's font size
    pub fn set_font_size(&mut self, selector: WidgetSelector, size: f32) {
        if let Some(mut item) = self.get_style_item_with_text(selector) {
            item.text_font.font_size = size;
        }
    }

    /// Directly set width to widget
    pub fn set_width(&mut self, selector: WidgetSelector, width: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.width = width;
        }
    }

    /// Directly set height to widget
    pub fn set_height(&mut self, selector: WidgetSelector, height: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.height = height;
        }
    }

    /// Directly set width and height to widget
    pub fn set_size(&mut self, selector: WidgetSelector, size: (Val, Val)) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.width = size.0;
            item.node.height = size.1;
        }
    }

    /// Directly set left position to widget
    pub fn set_left(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.left = val;
        }
    }

    /// Directly set right position to widget
    pub fn set_right(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.right = val;
        }
    }

    /// Directly set top position to widget
    pub fn set_top(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.top = val;
        }
    }

    /// Directly set bottom position to widget
    pub fn set_bottom(&mut self, selector: WidgetSelector, val: Val) {
        if let Some(mut item) = self.get_style_item(selector) {
            item.node.bottom = val;
        }
    }
}
