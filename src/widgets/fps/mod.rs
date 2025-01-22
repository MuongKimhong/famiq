pub mod helper;

use crate::utils::entity_add_child;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetBuilder};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

use super::color::{GREEN_COLOR, WHITE_COLOR, WARNING_COLOR, DANGER_COLOR};

const DEFAULT_FPS_TEXT_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct IsFamiqFPSTextLabel;

#[derive(Component)]
pub struct IsFamiqFPSTextCount;

#[derive(Component)]
pub struct IsFamiqFPSTextContainer;

// whether fps change color. green > 100, orange < 100, red < 60
#[derive(Component)]
pub struct CanChangeColor(pub bool);

pub struct FaFpsText;

// Doesn't need container
impl<'a> FaFpsText {
    fn _build_container(id: &str, right_side: bool, root_node: &'a mut EntityCommands) -> Entity {
        let mut node = default_fps_text_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        if right_side {
            node.left = Val::Auto;
            node.right = Val::Px(6.0);
        }

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
                Interaction::default(),
                GlobalZIndex(6)
            ))
            .id()
    }

    fn _build_text(
        id: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        change_color: bool
    ) -> Entity {
        let label_txt = Text::new("FPS:");
        let label_txt_font = TextFont {
            font: font_handle,
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let label_txt_color = TextColor(WHITE_COLOR);
        let label_txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let count_txt = TextSpan::default();
        let count_txt_font = label_txt_font.clone();
        let count_txt_color = TextColor(GREEN_COLOR);

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
                IsFamiqFPSTextCount,
                CanChangeColor(change_color)
            ))
            .id();

        entity_add_child(root_node, count_txt_entity, label_txt_entity);
        label_txt_entity
    }

    pub fn new(
        id: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        change_color: bool,
        right_side: bool,
    ) -> Entity {
        let container_entity = Self::_build_container(id, right_side, root_node);
        let text_entity = Self::_build_text(id, root_node, font_handle, change_color);

        entity_add_child(root_node, text_entity, container_entity);
        text_entity
    }

    pub fn update_fps_count_system(
        diagnostics: Res<DiagnosticsStore>,
        mut text_q: Query<(&mut TextSpan, &mut TextColor, &CanChangeColor, &IsFamiqFPSTextCount)>
    ) {
        for (mut text, mut color, change_color, _) in text_q.iter_mut() {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    text.0 = format!("{value:.2}");

                    if change_color.0 {
                        if value > 100.0 {
                            color.0 = GREEN_COLOR;
                        }
                        else if value > 60.0 && value < 100.0 {
                            color.0 = WARNING_COLOR;
                        }
                        else {
                            color.0 = DANGER_COLOR;
                        }
                    }
                    else {
                        color.0 = WHITE_COLOR;
                    }
                }
            }
        }
    }
}

pub struct FaFpsTextBuilder<'a> {
    pub id: String,
    pub change_color: Option<bool>,
    pub right_side: Option<bool>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaFpsTextBuilder<'a> {
    pub fn new(id: String, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        Self {
            id,
            root_node,
            font_handle,
            change_color: Some(false),
            right_side: Some(false)
        }
    }

    pub fn change_color(mut self) -> Self {
        self.change_color = Some(true);
        self
    }

    pub fn right_side(mut self) -> Self {
        self.right_side = Some(true);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaFpsText::new(
            self.id.as_str(),
            &mut self.root_node,
            self.font_handle.clone(),
            self.change_color.unwrap(),
            self.right_side.unwrap()
        )
    }
}

pub fn fa_fps<'a>(builder: &'a mut FamiqWidgetBuilder, id: &str) -> FaFpsTextBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaFpsTextBuilder::new(
        id.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}
