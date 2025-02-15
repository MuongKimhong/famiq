pub mod components;
pub mod helper;
pub mod tests;

pub use components::*;
use helper::*;

use crate::utils::{self, entity_add_children};
use super::{
    DefaultTextEntity, DefaultWidgetEntity, ExternalStyleHasChanged, FamiqToolTipText, FamiqWidgetClasses, FamiqWidgetId, FamiqWidgetResource, WidgetStyle
};
use super::tooltip::FaToolTipResource;
use crate::event_writer::FaInteractionEvent;
use bevy::prelude::*;

pub struct FaButton;

// Needs container
impl FaButton {
    fn _build_text(
        id: &Option<String>,
        class: &Option<String>,
        text: &str,
        commands: &mut Commands,
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

        let entity = commands
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

        utils::insert_id_and_class(commands, entity, id, class);
        entity
    }

    fn _build_overlay(commands: &mut Commands, shape: &BtnShape) -> Entity {
        let mut border_radius =  BorderRadius::all(Val::Px(6.0));

        match shape {
            BtnShape::Round => border_radius = BorderRadius::all(Val::Percent(50.0)),
            BtnShape::Rectangle => border_radius = BorderRadius::all(Val::Px(0.0)),
            _ => ()
        }

        commands
            .spawn((
                default_button_overlay_node(),
                BorderColor::default(),
                BackgroundColor::default(),
                border_radius,
                ZIndex(2),
                Visibility::Inherited,
                Interaction::default(),
                IsFamiqButtonOverlay
            ))
            .id()
    }

    fn _update_overlay(
        overlay_q: &mut Query<
            (&mut Node, &mut BackgroundColor, &mut BorderColor, &mut BorderRadius),
            (With<IsFamiqButtonOverlay>, Without<IsFamiqButton>)
        >,
        button_border_radius: &BorderRadius,
        button_node: &Node,
        button_computed_node: &ComputedNode,
        overlay_entity: Entity,
        update_to: &str
    ) {
        if let Ok((mut node, mut bg_color, mut bd_color, mut bd_radius)) = overlay_q.get_mut(overlay_entity) {
            node.border = button_node.border;
            node.width = Val::Px(button_computed_node.size().x);
            node.height = Val::Px(button_computed_node.size().y);
            *bd_radius = button_border_radius.clone();


            match update_to {
                "hover" => {
                    bg_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.3);
                    bd_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.3);
                },
                "press" => {
                    bg_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.6);
                    bd_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.6);
                },
                "none" => {
                    *bg_color = BackgroundColor::default();
                    *bd_color = BorderColor::default();
                }
                _ => {}
            }
        }
    }

    /// Internal system to handle `fa_button` interaction events.
    pub fn handle_button_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut builder_res: ResMut<FamiqWidgetResource>,
        mut button_q: Query<
            (
            &Node,
            &ComputedNode,
            &GlobalTransform,
            &ButtonOverlayEntity,
            &BorderRadius,
            Option<&FamiqToolTipText>
            ),
            With<IsFamiqButton>
        >,
        mut tooltip_res: ResMut<FaToolTipResource>,
        mut overlay_q: Query<
            (&mut Node, &mut BackgroundColor, &mut BorderColor, &mut BorderRadius),
            (With<IsFamiqButtonOverlay>, Without<IsFamiqButton>)
        >
    ) {
        for e in events.read() {
            if let Ok((
                node,
                computed,
                transform,
                overlay_entity,
                border_radius,
                tooltip_text
            )) = button_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        if let Some(text) = tooltip_text {
                            tooltip_res.show(text.0.clone(), computed.size(), transform.translation());
                        }
                        FaButton::_update_overlay(&mut overlay_q, border_radius, node, computed, overlay_entity.0, "hover");
                    },
                    Interaction::Pressed => {
                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                        FaButton::_update_overlay(&mut overlay_q, border_radius, node, computed, overlay_entity.0, "press");
                    },
                    Interaction::None => {
                        if tooltip_text.is_some() {
                            tooltip_res.hide();
                        }
                        FaButton::_update_overlay(&mut overlay_q, border_radius, node, computed, overlay_entity.0, "none");
                    },
                }
            }
        }
    }

    pub fn _detect_fa_button_creation_system(
        mut commands: Commands,
        famiq_res: Res<FamiqWidgetResource>,
        asset_server: Res<AssetServer>,
        button_q: Query<
            (Entity, Option<&FamiqWidgetId>, Option<&FamiqWidgetClasses>, &FaButtonText, &BtnColor, &BtnShape, &BtnSize),
            Added<IsFamiqButton>
        >
    ) {
        for (entity, id, class, text, color, shape, size) in button_q.iter() {
            let font_handle = asset_server.load(&famiq_res.font_path);
            let id_ref = id.map(|s| s.0.clone());
            let class_ref = class.map(|s| s.0.clone());
            let txt_entity = FaButton::_build_text(&id_ref, &class_ref, &text.0, &mut commands, font_handle, &color, &size);
            let overlay_entity = FaButton::_build_overlay(&mut commands, &shape);

            let mut node = default_button_node();
            utils::process_spacing_built_in_class(&mut node, &class_ref);

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

            commands
                .entity(entity)
                .insert((
                    node.clone(),
                    border_color.clone(),
                    bg_color.clone(),
                    border_radius.clone(),
                    z_index.clone(),
                    visibility.clone(),
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
                    ExternalStyleHasChanged(false),
                    ButtonOverlayEntity(overlay_entity)
                ));

            entity_add_children(&mut commands, &vec![overlay_entity, txt_entity], entity);
        }
    }
}

/// Builder for creating `fa_button`.
pub struct FaButtonBuilder<'w, 's> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub text: String,
    pub commands: Commands<'w, 's>,
    pub has_tooltip: bool,
    pub tooltip_text: String
}

impl<'w, 's> FaButtonBuilder<'w, 's> {
    pub fn new(text: &str, commands: Commands<'w, 's>) -> Self {
        Self {
            id: None,
            class: None,
            text: text.to_string(),
            commands,
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
        let entity = self.commands.spawn((
            IsFamiqButton,
            FaButtonText(self.text.clone()),
            color,
            size,
            shape
        ))
        .id();

        utils::insert_id_and_class(&mut self.commands, entity, &self.id, &self.class);
        entity
    }
}

/// API to create a `FaButtonBuilder`.
pub fn fa_button<'w, 's>(commands: &'w mut Commands, text: &str) -> FaButtonBuilder<'w, 's>
where
    'w: 's
{
    FaButtonBuilder::new(text, commands.reborrow())
}

/// Checks if the button internal system(s) can run.
///
/// `True` only if there is a button widget created.
pub fn can_run_button_systems(button_q: Query<&IsFamiqButton>) -> bool {
    button_q.iter().count() > 0
}
