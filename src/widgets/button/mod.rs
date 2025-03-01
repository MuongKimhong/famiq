pub mod components;
pub mod styling;
pub mod tests;

pub use components::*;
use styling::*;

use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct FaButton;

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
                IsFamiqButtonText
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    pub fn new(
        attributes: WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        has_tooltip: bool,
        tooltip_text: Option<String>
    ) -> Entity {
        let txt_entity = Self::_build_text(&attributes, text, root_node);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node;
        style_components.border_color = get_color(&attributes.color).into();
        style_components.background_color = get_color(&attributes.color).into();
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));

        let btn_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqButton,
                DefaultWidgetEntity::from(style_components),
                ButtonTextEntity(txt_entity),
                ButtonColorWasDarkened(false)
            ))
            .id();

        if has_tooltip {
            let tooltip = build_tooltip_node(
                &tooltip_text.unwrap(),
                attributes.font_handle.clone().unwrap(),
                root_node
            );
            root_node.commands().entity(btn_entity).insert(FamiqTooltipEntity(tooltip));
            root_node.commands().entity(btn_entity).add_child(tooltip);
        }
        insert_id_and_class(root_node, btn_entity, &attributes.id, &attributes.class);
        entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }

    /// Internal system to handle `fa_button` interaction events.
    pub(crate) fn handle_button_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut builder_res: ResMut<FamiqResource>,
        mut button_q: Query<
            (
                &GlobalTransform,
                &mut BackgroundColor,
                &mut ButtonColorWasDarkened,
                Option<&FamiqTooltipEntity>
            ),
            With<IsFamiqButton>
        >,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,

        window: Single<Entity, With<Window>>,
        mut commands: Commands,
        cursor_icons: Res<CursorIcons>,
    ) {
        for e in events.read() {
            if let Ok((
                btn_transform,
                mut background_color,
                mut was_darkened,
                tooltip_entity
            )) = button_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
  
                        if was_darkened.0 {
                            if let Some(lightened_color) = lighten_color(20.0, &background_color.0) {
                                background_color.0 = lightened_color;
                                was_darkened.0 = false;
                            }
                        }
                        show_tooltip(
                            tooltip_entity,
                            &mut tooltip_q,
                            btn_transform.translation()
                        ); 
                    },
                    Interaction::Pressed => {
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);

                        if let Some(darkened_color) = darken_color(20.0, &background_color.0) {
                            background_color.0 = darkened_color;
                            was_darkened.0 = true;
                        }
                    },
                    Interaction::None => {
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);

                        if was_darkened.0 {
                            if let Some(lightened_color) = lighten_color(20.0, &background_color.0) {
                                background_color.0 = lightened_color;
                                was_darkened.0 = false;
                            }
                        }
                        hide_tooltip(tooltip_entity, &mut tooltip_q);
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

        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
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
