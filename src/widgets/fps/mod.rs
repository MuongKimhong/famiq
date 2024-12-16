use crate::utils::strip_assets_prefix;
use crate::widgets::{DefaultTextBundle, FaWidgetBundle, FamiqWidgetId};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

// TextBundle Wrapped inside FaWidgetBundle
const DEFAULT_FPS_TEXT_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct IsFamiqFPSText;

#[derive(Component)]
pub struct IsFamiqFPSTextContainer;

pub struct FaFpsText;

impl<'a> FaFpsText {
    fn _build_fps_text_container(id: &str, root_node: &'a mut EntityCommands) -> Entity {
        let container_bundle = FaWidgetBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Auto,
                height: Val::Auto,
                left: Val::Px(5.0),
                top: Val::Px(5.0),
                ..default()
            },
            ..default()
        };
        root_node
            .commands()
            .spawn((
                container_bundle,
                IsFamiqFPSTextContainer,
                FamiqWidgetId(format!("{id}_fps_text_container")),
            ))
            .id()
    }

    fn _build_fps_text(
        id: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let path = strip_assets_prefix(font_path).unwrap();

        let fps_label = TextSection::new(
            "FPS: ",
            TextStyle {
                font: asset_server.load(&path),
                font_size: DEFAULT_FPS_TEXT_SIZE,
                ..default()
            },
        );
        let fps_count = TextSection::new(
            "0",
            TextStyle {
                font: asset_server.load(&path),
                font_size: DEFAULT_FPS_TEXT_SIZE,
                ..default()
            },
        );
        let fps_text_bundle = TextBundle::from_sections([fps_label.clone(), fps_count.clone()]);

        root_node
            .commands()
            .spawn((
                fps_text_bundle,
                IsFamiqFPSText,
                FamiqWidgetId(id.to_string()),
                DefaultTextBundle(TextBundle::from_sections([fps_label, fps_count])),
            ))
            .id()
    }

    pub fn new(
        id: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
    ) -> Entity {
        let container_entity = Self::_build_fps_text_container(id, root_node);
        let fps_entity = Self::_build_fps_text(id, root_node, asset_server, font_path);

        root_node
            .commands()
            .entity(container_entity)
            .add_child(fps_entity);

        fps_entity
    }

    pub fn update_fps_count_system(
        diagnostics: Res<DiagnosticsStore>,
        mut query: Query<&mut Text, With<IsFamiqFPSText>>,
    ) {
        for mut text in &mut query {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    text.sections[1].value = format!("{value:.2}");
                }
            }
        }
    }
}
