pub mod helper;
pub mod tests;

use crate::utils::{entity_add_child, insert_id_and_class, process_spacing_built_in_class};
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity, DefaultTextSpanEntity,
    FamiqBuilder, WidgetStyle, ExternalStyleHasChanged
};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

use super::color::{GREEN_COLOR, WHITE_COLOR, WARNING_COLOR, DANGER_COLOR};

const DEFAULT_FPS_TEXT_SIZE: f32 = 20.0;

/// Marker component for identifying the label part of the FPS text (e.g., "FPS:").
#[derive(Component)]
pub struct IsFamiqFPSTextLabel;

/// Marker component for identifying the FPS count text (e.g., "60.0").
#[derive(Component)]
pub struct IsFamiqFPSTextCount;


/// Component to indicate whether the FPS text color can change dynamically.
/// - `true`: The FPS text will change color based on the FPS value.
/// - `false`: The FPS text color remains constant.
#[derive(Component)]
pub struct CanChangeColor(pub bool);


pub struct FaFpsText;

impl<'a> FaFpsText {
    fn _build_fps(
        font_handle: Handle<Font>,
        change_color: bool,
        id: Option<String>,
        class: Option<String>,
        right_side: bool,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let mut node = default_fps_text_container_node();
        process_spacing_built_in_class(&mut node, &class);

        if right_side {
            node.left = Val::Auto;
            node.right = Val::Px(6.0);
        }
        let label_txt_font = TextFont {
            font: font_handle,
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let count_txt_font = label_txt_font.clone();

        let label_txt_entity = root_node
            .commands()
            .spawn((
                Text::new("FPS:"),
                label_txt_font.clone(),
                TextColor(WHITE_COLOR),
                TextLayout::new_with_justify(JustifyText::Center),
                DefaultTextEntity::new(
                    Text::new("FPS:"),
                    label_txt_font,
                    TextColor(WHITE_COLOR),
                    TextLayout::new_with_justify(JustifyText::Center),
                ),
                IsFamiqFPSTextLabel,
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .insert((
                node.clone(),
                BorderColor::default(),
                BorderRadius::default(),
                BackgroundColor::default(),
                ZIndex::default(),
                Visibility::Visible,
                DefaultWidgetEntity::new(
                    node,
                    BorderColor::default(),
                    BorderRadius::default(),
                    BackgroundColor::default(),
                    ZIndex::default(),
                    Visibility::Visible,
                ),
                Interaction::default(),
                GlobalZIndex(6)
            ))
            .id();

        let count_txt_entity = root_node
            .commands()
            .spawn((
                TextSpan::default(),
                count_txt_font.clone(),
                TextColor(GREEN_COLOR),
                IsFamiqFPSTextCount,
                CanChangeColor(change_color),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false),
                DefaultTextSpanEntity::new(
                    TextSpan::default(),
                    count_txt_font,
                    TextColor(GREEN_COLOR),
                )
            ))
            .id();

        insert_id_and_class(root_node, label_txt_entity, &id, &class);
        insert_id_and_class(root_node, count_txt_entity, &id, &class);
        entity_add_child(root_node, count_txt_entity, label_txt_entity);
        label_txt_entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        change_color: bool,
        right_side: bool,
    ) -> Entity {
        Self::_build_fps(font_handle, change_color, id, class, right_side, root_node)
    }

    /// Internal system to update the FPS count and optionally change its color based on the value.
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

/// Builder for creating an FPS text widget.
pub struct FaFpsTextBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub change_color: Option<bool>,
    pub right_side: Option<bool>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaFpsTextBuilder<'a> {
    pub fn new(font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            root_node,
            font_handle,
            change_color: Some(false),
            right_side: Some(false)
        }
    }

    /// Enables dynamic color changes based on FPS value.
    pub fn change_color(mut self) -> Self {
        self.change_color = Some(true);
        self
    }

    /// Method to add id to fps.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Method to add class to fps.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Aligns the FPS widget to the right top corner of the screen.
    pub fn right_side(mut self) -> Self {
        self.right_side = Some(true);
        self
    }

    /// Spawn fps into UI World.
    pub fn build(&mut self) -> Entity {
        FaFpsText::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_node,
            self.font_handle.clone(),
            self.change_color.unwrap(),
            self.right_side.unwrap()
        )
    }
}

/// API to create an `FaFpsTextBuilder`.
pub fn fa_fps<'a>(builder: &'a mut FamiqBuilder) -> FaFpsTextBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaFpsTextBuilder::new(
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

/// a system to check if FPS internal system(s) can run.
///
/// True only if fps widget is created.
pub fn can_run_fps_systems(fps_q: Query<&IsFamiqFPSTextLabel>) -> bool {
    !fps_q.is_empty()
}
