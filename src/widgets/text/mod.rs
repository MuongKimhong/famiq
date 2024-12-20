use super::color::WHITE_COLOR;
use crate::utils::strip_assets_prefix;
use crate::widgets::FamiqWidgetId;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqText;

// need container
pub fn fa_text<'a>(
    id: &str,
    value: &str,
    root_node: &'a mut EntityCommands,
    asset_server: &'a ResMut<'a, AssetServer>,
    font_path: &String,
) -> Entity {
    let path = strip_assets_prefix(font_path).unwrap();

    root_node
        .commands()
        .spawn((
            Text::new(value),
            TextFont {
                font: asset_server.load(path),
                ..default()
            },
            TextColor(WHITE_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            FamiqWidgetId(id.to_string()),
        ))
        .id()
}
