pub mod btn_text;
pub mod helper;

use crate::utils;
use crate::widgets::{DefaultTextBundle, DefaultWidgetBundle, FamiqWidgetId};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use btn_text::*;
use helper::*;

#[derive(Component)]
pub struct ButtonText(pub String);

#[derive(Component)]
pub struct IsFamiqButton;

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
    pub fn normal_btn(
        id: &str,
        text: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        variant: Option<BtnVariant>,
        size: Option<BtnSize>,
    ) -> Entity {
        let text_bundle = create_button_text(text, &size, &variant, asset_server, font_path);
        let button_bundle = create_default_button_bundle(&variant, &size, &text_bundle);

        let txt_entity = root_node
            .commands()
            .spawn((
                text_bundle,
                ButtonText(text.to_string()),
                FamiqWidgetId(format!("{id}_btn_text")),
                DefaultTextBundle(create_button_text(
                    text,
                    &size,
                    &variant,
                    asset_server,
                    font_path,
                )),
            ))
            .id();

        let btn_entity = root_node
            .commands()
            .spawn((
                button_bundle.clone(),
                FamiqWidgetId(id.to_string()),
                IsFamiqButton,
                DefaultWidgetBundle(button_bundle),
            ))
            .id();

        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }
}
