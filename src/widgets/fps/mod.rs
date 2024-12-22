pub mod helper;

use crate::utils::{entity_add_child, strip_assets_prefix};
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

use super::color::{GREEN_COLOR, WHITE_COLOR};

const DEFAULT_FPS_TEXT_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct IsFamiqFPSTextLabel;

#[derive(Component)]
pub struct IsFamiqFPSTextCount;

#[derive(Component)]
pub struct IsFamiqFPSTextContainer;

pub struct FaFpsText;

impl<'a> FaFpsText {
    fn _build_container(id: &str, root_node: &'a mut EntityCommands) -> Entity {
        let node = default_fps_text_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex(50);
        let visibility = Visibility::Visible;

        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(format!("{id}_fps_text_container")),
                IsFamiqFPSTextContainer,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default()
            ))
            .id()
    }

    fn _build_text(
        id: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let label_txt = Text::new("FPS: ");
        let label_txt_font = TextFont {
            font: asset_server.load(strip_assets_prefix(font_path).unwrap()),
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let label_txt_color = TextColor(WHITE_COLOR);
        let label_txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let count_txt = TextSpan::default();
        let count_txt_font = label_txt_font.clone();
        let count_txt_color = TextColor(GREEN_COLOR);
        let count_txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let label_txt_entity = root_node
            .commands()
            .spawn((
                label_txt.clone(),
                label_txt_font.clone(),
                label_txt_color.clone(),
                label_txt_layout.clone(),
                FamiqWidgetId(id.to_string()),
                DefaultTextEntity::new(
                    label_txt,
                    label_txt_font,
                    label_txt_color,
                    label_txt_layout,
                ),
                IsFamiqFPSTextLabel,
            ))
            .id();

        let count_txt_entity = root_node
            .commands()
            .spawn((
                count_txt,
                count_txt_font,
                count_txt_color,
                count_txt_layout,
                IsFamiqFPSTextCount,
            ))
            .id();

        entity_add_child(root_node, count_txt_entity, label_txt_entity);
        label_txt_entity
    }

    pub fn new(
        id: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let container_entity = Self::_build_container(id, root_node);
        let text_entity = Self::_build_text(id, root_node, asset_server, font_path);

        entity_add_child(root_node, text_entity, container_entity);
        text_entity
    }

    //     pub fn update_fps_count_system(
    //         diagnostics: Res<DiagnosticsStore>,
    //         mut query: Query<&mut Text, With<IsFamiqFPSText>>,
    //     ) {
    //         for mut text in &mut query {
    //             if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
    //                 if let Some(value) = fps.smoothed() {
    //                     text.sections[1].value = format!("{value:.2}");
    //                 }
    //             }
    //         }
    //     }
}
