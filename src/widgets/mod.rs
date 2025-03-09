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
pub use base_components::*;
pub use style::*;
use crate::resources::*;
use crate::widgets::style_parse::*;
use bevy::ecs::system::EntityCommands;
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
    pub fn new(
        commands: &'a mut Commands,
        builder_resource: &'a mut ResMut<FamiqResource>,
        asset_server: &'a Res<'a, AssetServer>,
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
        font_size: 18.0,
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
