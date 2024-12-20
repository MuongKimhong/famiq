pub mod helper;

use crate::utils;
// use crate::widgets::{DefaultTextBundle, DefaultWidgetBundle, FamiqWidgetId};
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId};
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
        let txt = Text::new(text);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(get_text_color(&variant));
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let txt_entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                FamiqWidgetId(format!("{id}_btn_text")),
                FaButtonText(text.to_string()),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
            ))
            .id();

        let (height, border_width) = get_button_size(size);
        let node = default_button_node(height, border_width);
        let border_color = get_button_border_color(&variant);
        let bg_color = get_button_background_color(&variant);
        let border_radius = BorderRadius::all(Val::Px(5.0));
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

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
                IsFamiqButton,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
            ))
            .id();

        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }
}
