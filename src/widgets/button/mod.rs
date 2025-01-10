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
pub struct ButtonTextEntity(pub Entity);

#[derive(Component)]
pub struct ButtonTextContainerEntity(pub Entity);

#[derive(Component)]
pub struct FaButtonText(pub String);

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
                ButtonTextEntity(txt_entity)
            ))
            .id();

        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }

    pub fn handle_button_on_hover_system(
        mut events: EventReader<FaInteractionEvent>,
        mut button_q: Query<(&IsFamiqButton, &DefaultWidgetEntity, &mut BackgroundColor, &mut BorderColor)>
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
                    },
                    Interaction::None => {
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                    },
                }
            }
        }
    }
}
