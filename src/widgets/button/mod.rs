pub mod components;
pub mod helper;
pub mod tests;

pub use components::*;
use helper::*;

use crate::utils;
use super::{
    DefaultTextEntity, DefaultWidgetEntity,
    FamiqWidgetResource, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged, FamiqToolTipText
};
use super::tooltip::FaToolTipResource;
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

/// Built-in button color options for `fa_button`.
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

/// Built-in button size options for `fa_button`.
pub enum BtnSize {
    Small,
    Normal,
    Large,
}
/// Built-in shape options for `fa_button`.
pub enum BtnShape {
    Default,
    Round,
    Rectangle
}

pub struct FaButton;

// Needs container
impl<'a> FaButton {
    fn _build_text(
        id: &Option<String>,
        class: &Option<String>,
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
                IsFamiqButtonText,
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        utils::insert_id_and_class(root_node, entity, id, class);
        entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        text: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        color: BtnColor,
        size: BtnSize,
        shape: BtnShape,
        has_tooltip: bool,
        tooltip_text: Option<String>
    ) -> Entity {
        let txt_entity = Self::_build_text(&id, &class, text, root_node, font_handle, &color, &size);

        let mut node = default_button_node();
        utils::process_spacing_built_in_class(&mut node, &class);

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

        if has_tooltip {
            root_node.commands().entity(btn_entity).insert(FamiqToolTipText(tooltip_text.unwrap()));
        }
        utils::insert_id_and_class(root_node, btn_entity, &id, &class);
        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }

    /// Internal system to handle `fa_button` interaction events.
    pub fn handle_button_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut builder_res: ResMut<FamiqWidgetResource>,
        mut button_q: Query<(
            &DefaultWidgetEntity,
            &mut BackgroundColor,
            &mut BorderColor,
            &ComputedNode,
            &GlobalTransform,
            Option<&FamiqToolTipText>
        ), With<IsFamiqButton>>,
        mut tooltip_res: ResMut<FaToolTipResource>
    ) {
        for e in events.read() {
            if let Ok((default_style, mut bg_color, mut bd_color, computed, transform, tooltip_text)) = button_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        if let Some(text) = tooltip_text {
                            tooltip_res.show(text.0.clone(), computed.size(), transform.translation());
                        }
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
                        if tooltip_text.is_some() {
                            tooltip_res.hide();
                        }
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                    },
                }
            }
        }
    }
}

/// Builder for creating `fa_button`.
pub struct FaButtonBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub text: String,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>,
    pub has_tooltip: bool,
    pub tooltip_text: String
}

impl<'a> FaButtonBuilder<'a> {
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
            has_tooltip: false,
            tooltip_text: String::new()
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

    /// Method to add class to button entity.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to button entity.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Method to add tooltip to button.
    pub fn tooltip(mut self, text: &str) -> Self {
        self.has_tooltip = true;
        self.tooltip_text = text.to_string();
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
            shape,
            self.has_tooltip,
            Some(self.tooltip_text.clone())
        )
    }
}

/// API to create a `FaButtonBuilder`.
pub fn fa_button<'a>(builder: &'a mut FamiqWidgetBuilder, text: &str) -> FaButtonBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaButtonBuilder::new(
        text.to_string(),
        font_handle,
        builder.ui_root_node.reborrow(),
    )
}

/// Checks if the button internal system(s) can run.
///
/// `True` only if there is a button widget created.
pub fn can_run_button_systems(button_q: Query<&IsFamiqButton>) -> bool {
    button_q.iter().count() > 0
}
