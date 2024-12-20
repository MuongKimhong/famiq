pub mod helper;

use crate::utils;
// use crate::widgets::{DefaultTextBundle, DefaultWidgetBundle, FamiqWidgetId};
use crate::widgets::FamiqWidgetId;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

#[derive(Component)]
pub struct IsFamiqButton;

#[derive(Component)]
pub struct FaButtonText(pub String);

pub enum BtnVariant {
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

pub struct FaButton;

// buttons need to be inside a container
impl<'a> FaButton {
    pub fn new(
        id: &str,
        text: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        variant: BtnVariant,
        size: BtnSize,
    ) -> Entity {
        let txt_entity = root_node
            .commands()
            .spawn((
                Text::new(text),
                TextFont {
                    font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
                    font_size: get_text_size(&size),
                    ..default()
                },
                TextColor(get_text_color(&variant)),
                TextLayout::new_with_justify(JustifyText::Center),
                FamiqWidgetId(format!("{id}_btn_text")),
                FaButtonText(text.to_string()),
            ))
            .id();

        let (height, border_width) = get_button_size(size);
        let btn_entity = root_node
            .commands()
            .spawn((
                default_button_node(height, border_width),
                get_button_border_color(&variant),
                get_button_background_color(&variant),
                FamiqWidgetId(id.to_string()),
                IsFamiqButton,
                BorderRadius::all(Val::Px(5.0)),
            ))
            .id();

        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }
}
