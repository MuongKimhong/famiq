pub mod helper;
pub mod tests;

use crate::utils::{entity_add_child, process_spacing_built_in_class};
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity, DefaultTextSpanEntity,
    FamiqWidgetId, FamiqWidgetBuilder, FamiqWidgetClasses,
    WidgetStyle, ExternalStyleHasChanged
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

/// Marker component for identifying the FPS text container that holds
/// both label & text count (eg. FPS: 60.0).
#[derive(Component)]
pub struct IsFamiqFPSTextContainer;

/// Component to indicate whether the FPS text color can change dynamically.
/// - `true`: The FPS text will change color based on the FPS value.
/// - `false`: The FPS text color remains constant.
#[derive(Component)]
pub struct CanChangeColor(pub bool);


/// Represents the Famiq FPS text widget.
pub struct FaFpsText;

// Doesn't need container
impl<'a> FaFpsText {
    fn _build_container(
        id: Option<String>,
        class: Option<String>,
        right_side: bool,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let mut node = default_fps_text_container_node();
        process_spacing_built_in_class(&mut node, &class);

        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        if right_side {
            node.left = Val::Auto;
            node.right = Val::Px(6.0);
        }

        let entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
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
                GlobalZIndex(6),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        root_node.add_child(entity);
        entity
    }

    fn _build_text(
        id: &Option<String>,
        class: &Option<String>,
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
                DefaultTextEntity::new(
                    label_txt,
                    label_txt_font,
                    label_txt_color,
                    label_txt_layout,
                ),
                IsFamiqFPSTextLabel,
                Visibility::Inherited,
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        let count_txt_entity = root_node
            .commands()
            .spawn((
                count_txt.clone(),
                count_txt_font.clone(),
                count_txt_color.clone(),
                IsFamiqFPSTextCount,
                CanChangeColor(change_color),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false),
                DefaultTextSpanEntity::new(
                    count_txt,
                    count_txt_font,
                    count_txt_color,
                )
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(label_txt_entity).insert(FamiqWidgetId(id.to_owned()));
            root_node.commands().entity(count_txt_entity).insert(FamiqWidgetId(id.to_owned()));
        }
        if let Some(class) = class {
            root_node.commands().entity(label_txt_entity).insert(FamiqWidgetClasses(class.to_owned()));
            root_node.commands().entity(count_txt_entity).insert(FamiqWidgetClasses(class.to_owned()));
        }

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
        let text_entity = Self::_build_text(&id, &class, root_node, font_handle, change_color);
        let container_entity = Self::_build_container(id, class, right_side, root_node);

        entity_add_child(root_node, text_entity, container_entity);
        text_entity
    }

    /// System to update the FPS count and optionally change its color based on the value.
    ///
    /// # Parameters
    /// - `diagnostics`: Diagnostics resource containing FPS data.
    /// - `text_q`: Query to retrieve FPS count text entities.
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

    /// Method to add id to fps
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Aligns the FPS widget to the right top corner of the screen.
    pub fn right_side(mut self) -> Self {
        self.right_side = Some(true);
        self
    }

    /// Spawn fps into UI World
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
pub fn fa_fps<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaFpsTextBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaFpsTextBuilder::new(
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

/// a system to check if FPS internal system(s) can run.
///
/// True only if fps widget is created.
pub fn can_run_fps_systems(fps_q: Query<&IsFamiqFPSTextContainer>) -> bool {
    fps_q.iter().count() > 0
}
