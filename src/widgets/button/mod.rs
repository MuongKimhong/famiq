pub mod components;
pub mod helper;
pub mod tests;

pub use components::*;
use helper::*;

use crate::plugin::{CursorIcons, CursorType};
use crate::utils;
use crate::widgets::*;
use crate::widgets::tooltip::{FaToolTip, FaToolTipResource, IsFamiqToolTipText};
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct FaButton;

// Needs container
impl<'a> FaButton {
    fn _build_text(
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let txt = Text::new(text);
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };
        let txt_color = TextColor(get_text_color(&attributes.color));
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

        utils::insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    fn _build_overlay(root_node: &'a mut EntityCommands) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_button_overlay_node();
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));
        style_components.z_index = ZIndex(2);

        root_node
            .commands()
            .spawn((style_components, IsFamiqButtonOverlay))
            .id()
    }

    pub fn new(
        attributes: WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        has_tooltip: bool,
        tooltip_text: Option<String>
    ) -> Entity {
        let txt_entity = Self::_build_text(&attributes, text, root_node);
        let overlay_entity = Self::_build_overlay(root_node);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node;
        style_components.border_color = get_button_border_color(&attributes.color);
        style_components.background_color = get_button_background_color(&attributes.color);
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));

        let btn_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqButton,
                DefaultWidgetEntity::from(style_components),
                ButtonTextEntity(txt_entity),
                ButtonOverlayEntity(overlay_entity)
            ))
            .id();

        if has_tooltip {
            root_node.commands().entity(btn_entity).insert(FamiqToolTipText(tooltip_text.unwrap()));
        }
        utils::insert_id_and_class(root_node, btn_entity, &attributes.id, &attributes.class);
        utils::entity_add_children(root_node, &vec![overlay_entity, txt_entity], btn_entity);
        btn_entity
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
                    bg_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.2);
                    bd_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.2);
                },
                "press" => {
                    bg_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.5);
                    bd_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.5);
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
        mut builder_res: ResMut<FamiqResource>,
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
        >,
        mut tooltip_text_q: Query<&mut Text, With<IsFamiqToolTipText>>,

        window: Single<Entity, With<Window>>,
        mut commands: Commands,
        cursor_icons: Res<CursorIcons>,
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
                            FaToolTip::_update_toolitp_text(&text.0, &mut tooltip_text_q);
                            tooltip_res.show(text.0.clone(), computed.size(), transform.translation());
                        }
                        FaButton::_update_overlay(&mut overlay_q, border_radius, node, computed, overlay_entity.0, "hover");
                        utils::_change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                    },
                    Interaction::Pressed => {
                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                        FaButton::_update_overlay(&mut overlay_q, border_radius, node, computed, overlay_entity.0, "press");
                        utils::_change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                    },
                    Interaction::None => {
                        if tooltip_text.is_some() {
                            tooltip_res.hide();
                        }
                        FaButton::_update_overlay(&mut overlay_q, border_radius, node, computed, overlay_entity.0, "none");
                        utils::_change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
                    },
                }
            }
        }
    }
}

/// Builder for creating `fa_button`.
pub struct FaButtonBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub text: String,
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
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            text,
            root_node,
            has_tooltip: false,
            tooltip_text: String::new()
        }
    }

    /// Method to add tooltip to button.
    pub fn tooltip(mut self, text: &str) -> Self {
        self.has_tooltip = true;
        self.tooltip_text = text.to_string();
        self
    }

    /// Spawn the button to UI world.
    pub fn build(&mut self) -> Entity {
        self._process_built_in_color_class();
        self._process_built_in_size_class();
        self._node();
        FaButton::new(
            self.attributes.clone(),
            self.text.as_str(),
            &mut self.root_node,
            self.has_tooltip,
            Some(self.tooltip_text.clone())
        )
    }
}

impl<'a> SetWidgetAttributes for FaButtonBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_button_node();

        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }

        utils::process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create a `FaButtonBuilder`.
pub fn fa_button<'a>(builder: &'a mut FamiqBuilder, text: &str) -> FaButtonBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
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
    !button_q.is_empty()
}
