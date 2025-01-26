pub mod components;
pub mod helper;
pub mod tests;

use crate::utils;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity,
    FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetResource, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged
};
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub use components::*;
use helper::*;

/// Represents built-in button color options for a `FaButton`.
pub enum BtnColor {
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
    InfoDark
}

/// Represents built-in button size options for a `FaButton`.
pub enum BtnSize {
    Small,
    Normal,
    Large,
}
/// Represents a custom button shape for Famiq widgets.
pub enum BtnShape {
    Default,
    Round,
    Rectangle
}

pub struct FaButton;

// Needs container
impl<'a> FaButton {
    /// Builds the text entity for the button.
    ///
    /// # Parameters
    /// - `id`: Optional ID for the text entity.
    /// - `text`: The text to display on the button.
    /// - `root_node`: A mutable reference to the root node's `EntityCommands`.
    /// - `font_handle`: Handle to the font to use for the button text.
    /// - `color`: Button color configuration.
    /// - `size`: Button size configuration.
    ///
    /// # Returns
    /// - The `Entity` of the created text component.
    fn _build_text(
        id: &Option<String>,
        text: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        color: &BtnColor,
        size: &BtnSize,
    ) -> Entity {
        let txt = Text::new(text);
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(size),
            ..default()
        };
        let txt_color = TextColor(get_text_color(color));
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqButtonText
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(format!("{id}_btn_text")));
        }
        entity
    }

    /// Creates a new `FaButton` entity.
    ///
    /// # Parameters
    /// - `id`: Optional ID for the button.
    /// - `class`: Optional CSS-like class for styling.
    /// - `text`: The text to display on the button.
    /// - `root_node`: A mutable reference to the root node's `EntityCommands`.
    /// - `font_handle`: Handle to the font to use for the button text.
    /// - `color`: Button color configuration.
    /// - `size`: Button size configuration.
    /// - `shape`: Button shape configuration.
    ///
    /// # Returns
    /// - The `Entity` of the created button.
    pub fn new(
        id: Option<String>,
        class: Option<String>,
        text: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        color: BtnColor,
        size: BtnSize,
        shape: BtnShape
    ) -> Entity {
        let txt_entity = Self::_build_text(&id, text, root_node, font_handle, &color, &size);

        let node = default_button_node();
        let border_color = get_button_border_color(&color);
        let bg_color = get_button_background_color(&color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;
        let mut border_radius =  BorderRadius::all(Val::Px(6.0));

        match shape {
            BtnShape::Round => border_radius = BorderRadius::all(Val::Percent(50.0)),
            BtnShape::Rectangle => border_radius = BorderRadius::all(Val::Px(0.0)),
            _ => ()
        }
        let btn_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                bg_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqButton,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                ButtonTextEntity(txt_entity),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(btn_entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(btn_entity).insert(FamiqWidgetClasses(class));
        }
        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }

    /// System to handle internal button interaction events and apply styles accordingly.
    ///
    /// # Parameters
    /// - `events`: Event reader for `FaInteractionEvent`.
    /// - `button_q`: Query for buttons and their components.
    /// - `builder_res`: Mutable reference to `FamiqWidgetResource` for managing focus states.
    pub fn handle_button_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut button_q: Query<(&IsFamiqButton, &DefaultWidgetEntity, &mut BackgroundColor, &mut BorderColor)>,
        mut builder_res: ResMut<FamiqWidgetResource>
    ) {
        for e in events.read() {
            if let Ok((_, default_style, mut bg_color, mut bd_color)) = button_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        // darken by 10%
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                        darken_bg_and_bg_color(10.0, &mut bg_color, &mut bd_color);
                    },
                    Interaction::Pressed => {
                        // darken by 15%
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                        darken_bg_and_bg_color(15.0, &mut bg_color, &mut bd_color);

                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                    },
                    Interaction::None => {
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                    },
                }
            }
        }
    }
}

/// Builder for creating `FaButton` entities with customizable options.
pub struct FaButtonBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub text: String,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>,
}

impl<'a> FaButtonBuilder<'a> {
    /// Create a new FaButtonBuilder
    ///
    /// # Returns
    /// - A new instance of `FaButtonBuilder`.
    pub fn new(
        text: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>,
    ) -> Self {
        Self {
            id: None,
            class: None,
            text,
            font_handle,
            root_node,
        }
    }

    fn _process_built_in_classes(&self) -> (BtnColor, BtnSize, BtnShape) {
        let mut use_color = BtnColor::Default;
        let mut use_size = BtnSize::Normal;
        let mut use_shape = BtnShape::Default;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    // Check for colors
                    "is-primary" => use_color = BtnColor::Primary,
                    "is-primary-dark" => use_color = BtnColor::PrimaryDark,
                    "is-secondary" => use_color = BtnColor::Secondary,
                    "is-danger" => use_color = BtnColor::Danger,
                    "is-danger-dark" => use_color = BtnColor::DangerDark,
                    "is-success" => use_color = BtnColor::Success,
                    "is-success-dark" => use_color = BtnColor::SuccessDark,
                    "is-warning" => use_color = BtnColor::Warning,
                    "is-warning-dark" => use_color = BtnColor::WarningDark,
                    "is-info" => use_color = BtnColor::Info,
                    "is-info-dark" => use_color = BtnColor::InfoDark,

                    // Check for sizes
                    "is-small" => use_size = BtnSize::Small,
                    "is-large" => use_size = BtnSize::Large,
                    "is-normal" => use_size = BtnSize::Normal,

                    // check for shapes
                    "is-round" => use_shape = BtnShape::Round,
                    "is-rectangle" => use_shape = BtnShape::Rectangle,

                        _ => (),
                }
            }
        }
        (use_color, use_size, use_shape)
    }

    /// Method to add class to button entity
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to button entity
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Spawn the button to UI world.
    pub fn build(&mut self) -> Entity {
        let (color, size, shape) = self._process_built_in_classes();
        FaButton::new(
            self.id.clone(),
            self.class.clone(),
            self.text.as_str(),
            &mut self.root_node,
            self.font_handle.clone(),
            color,
            size,
            shape
        )
    }
}

/// API to create a `FaButtonBuilder`.
pub fn fa_button<'a>(builder: &'a mut FamiqWidgetBuilder, text: &str) -> FaButtonBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    builder.resource.can_run_systems.button = true;

    FaButtonBuilder::new(
        text.to_string(),
        font_handle,
        builder.ui_root_node.reborrow(),
    )
}

/// Checks if the button internal system(s) can run.
///
/// `True` only if there is a button widget created.
pub fn can_run_button_systems(builder_res: Res<FamiqWidgetResource>) -> bool {
    builder_res.can_run_systems.button
}
