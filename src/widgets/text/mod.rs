use super::color::WHITE_COLOR;
use crate::widgets::{DefaultTextEntity, FamiqWidgetId, FamiqWidgetClasses};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqText;

// Needs container
pub fn fa_text<'a>(
    id: &str,
    classes: &str,
    value: &str,
    root_node: &'a mut EntityCommands,
    asset_server: &'a ResMut<'a, AssetServer>,
    font_path: &String,
) -> Entity {
    let txt = Text::new(value);
    let txt_font = TextFont {
        font: asset_server.load(font_path),
        ..default()
    };
    let txt_color = TextColor(WHITE_COLOR);
    let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

    root_node
        .commands()
        .spawn((
            txt.clone(),
            txt_font.clone(),
            txt_color.clone(),
            txt_layout.clone(),
            FamiqWidgetId(id.to_string()),
            FamiqWidgetClasses(classes.to_string()),
            DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
            Interaction::default()
        ))
        .id()
}
