pub mod helper;
pub mod tests;

use crate::utils::{entity_add_child, insert_id_and_class, process_spacing_built_in_class};
use crate::widgets::*;
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
        attributes: &WidgetAttributes,
        change_color: bool,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.visibility = Visibility::Visible;

        let label_txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
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
                style_components.clone(),
                GlobalZIndex(6),
                DefaultWidgetEntity::from(style_components)
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

        insert_id_and_class(root_node, label_txt_entity, &attributes.id, &attributes.class);
        insert_id_and_class(root_node, count_txt_entity, &attributes.id, &attributes.class);
        entity_add_child(root_node, count_txt_entity, label_txt_entity);
        label_txt_entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        change_color: bool,
    ) -> Entity {
        Self::_build_fps(attributes, change_color, root_node)
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
    pub attributes: WidgetAttributes,
    pub change_color: bool,
    pub right_side: bool,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaFpsTextBuilder<'a> {
    pub fn new(font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            root_node,
            change_color: false,
            right_side: false
        }
    }

    /// Enables dynamic color changes based on FPS value.
    pub fn change_color(mut self) -> Self {
        self.change_color = true;
        self
    }

    /// Aligns the FPS widget to the right top corner of the screen.
    pub fn right_side(mut self) -> Self {
        self.right_side = true;
        self
    }

    /// Spawn fps into UI World.
    pub fn build(&mut self) -> Entity {
        self._node();

        if self.right_side {
            self.attributes.node.left = Val::Auto;
            self.attributes.node.right = Val::Px(6.0);
        }

        FaFpsText::new(
            &self.attributes,
            &mut self.root_node,
            self.change_color
        )
    }
}

impl<'a> SetWidgetAttributes for FaFpsTextBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_fps_text_container_node();
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
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
