pub mod helper;

use crate::utils;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses};
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

#[derive(Component)]
pub struct IsFamiqButton;

#[derive(Component)]
pub struct IsFamiqButtonText;

#[derive(Component)]
pub struct IsFamiqButtonTextContainer;

#[derive(Component)]
pub struct ButtonTextEntity(pub Entity);

#[derive(Component)]
pub struct ButtonTextContainerEntity(pub Entity);

#[derive(Component)]
pub struct FaButtonText(pub String);

pub enum BtnColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

pub enum BtnSize {
    Small,
    Normal,
    Large,
}

pub enum BtnShape {
    Default,
    Round,
    Rectangle
}

pub struct FaButton;

// Needs container
impl<'a> FaButton {
    fn _build_text(
        id: &str,
        text: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        color: &BtnColor,
        size: &BtnSize,
    ) -> Entity {
        let txt = Text::new(text);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(size),
            ..default()
        };
        let txt_color = TextColor(get_text_color(color));
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                FamiqWidgetId(format!("{id}_btn_text")),
                FaButtonText(text.to_string()),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqButtonText
            ))
            .id()
    }

    fn _build_text_container(root_node: &'a mut EntityCommands, height: Val, border_width: Val) -> Entity {
        root_node
            .commands()
            .spawn((
                default_button_text_container_node(height, border_width),
                BorderColor(Color::NONE),
                BorderRadius::default(),
                BackgroundColor(Color::NONE),
                ZIndex::default(),
                Visibility::Inherited,
                IsFamiqButtonTextContainer
            ))
            .id()
    }

    pub fn new(
        id: &str,
        classes: &str,
        text: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        color: BtnColor,
        size: BtnSize,
        shape: BtnShape
    ) -> Entity {
        let txt_entity = Self::_build_text(id, text, root_node, asset_server, font_path, &color, &size);
        let (height, border_width) = get_button_size(size);
        let txt_container_entity = Self::_build_text_container(root_node, height, border_width);

        utils::entity_add_child(root_node, txt_entity, txt_container_entity);

        let node = default_button_node(height);
        let border_color = get_button_border_color(&color);
        let bg_color = get_button_background_color(&color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;
        let mut border_radius =  BorderRadius::all(Val::Px(5.0));

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
                FamiqWidgetId(id.to_string()),
                FamiqWidgetClasses(classes.to_string()),
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
                ButtonTextContainerEntity(txt_container_entity)
            ))
            .id();

        utils::entity_add_child(root_node, txt_container_entity, btn_entity);
        btn_entity
    }

    pub fn handle_button_on_hover_system(
        mut events: EventReader<FaInteractionEvent>,
        mut text_container_q: Query<(
            &IsFamiqButtonTextContainer,
            &mut BackgroundColor,
            &mut BorderColor
        )>,
        button_q: Query<(&IsFamiqButton, &ButtonTextContainerEntity)>,
    ) {
        for e in events.read() {
            if let Ok((_, txt_container_entity)) = button_q.get(e.entity) {
                if let Ok((_, mut bg_color, mut border_color)) = text_container_q.get_mut(txt_container_entity.0) {
                    match e.interaction {
                        Interaction::Hovered => {
                            *bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5));
                            *border_color = BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.5));
                        }
                        Interaction::None => {
                            *bg_color = BackgroundColor(Color::NONE);
                            *border_color = BorderColor(Color::NONE);
                        }
                        _ => ()
                    }
                }
            }
        }
    }
}
