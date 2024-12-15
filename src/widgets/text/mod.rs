use crate::utils::strip_assets_prefix;
use crate::widgets::{DefaultTextBundle, FamiqWidgetId};
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
    custom_text_style: Option<TextStyle>,
    custom_style: Option<Style>,
) -> Entity {
    let path = strip_assets_prefix(font_path).unwrap();
    let font_handle = asset_server.load(path);

    let text_style = match custom_text_style {
        Some(v) => v,
        None => TextStyle {
            font: font_handle.clone(),
            ..default()
        },
    };
    let mut text_bundle =
        TextBundle::from_section(value, text_style.clone()).with_background_color(Color::NONE);

    if let Some(style) = custom_style {
        text_bundle = text_bundle.with_style(style);
    }
    root_node
        .commands()
        .spawn((
            text_bundle,
            FamiqWidgetId(id.to_string()),
            Interaction::default(),
            IsFamiqText,
            DefaultTextBundle(
                TextBundle::from_section(value, text_style).with_background_color(Color::NONE),
            ),
        ))
        .id()
}
