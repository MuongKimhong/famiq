pub mod helper;
pub mod tests;

use crate::utils::{entity_add_child, insert_id_and_class, process_spacing_built_in_class};
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity, DefaultTextSpanEntity,
    WidgetStyle, ExternalStyleHasChanged
};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use helper::*;

use super::color::{GREEN_COLOR, WHITE_COLOR, WARNING_COLOR, DANGER_COLOR};
use super::{FamiqWidgetClasses, FamiqWidgetId, FamiqWidgetResource, IsFaWidgetRoot};

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

/// Component to indicate whether the FPS text appears at right top corner of the screen.
#[derive(Component)]
pub struct RightSide(pub bool);


pub struct FaFpsText;

// Doesn't need container
impl FaFpsText {
    fn _build_container(
        id: &Option<String>,
        class: &Option<String>,
        right_side: bool,
        commands: &mut Commands
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

        let entity = commands
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

        insert_id_and_class(commands, entity, id, class);
        entity
    }

    fn _build_text_count(
        id: &Option<String>,
        class: &Option<String>,
        commands: &mut Commands,
        font_handle: Handle<Font>,
        change_color: bool
    ) -> Entity {
        let count_txt_font = TextFont {
            font: font_handle,
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let count_txt_entity = commands
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
        insert_id_and_class(commands, count_txt_entity, id, class);
        count_txt_entity
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

    pub fn _detect_fa_fps_creation_system(
        mut commands: Commands,
        fps_q: Query<
            (Entity, &CanChangeColor, &RightSide, Option<&FamiqWidgetId>, Option<&FamiqWidgetClasses>),
            Added<IsFamiqFPSTextLabel>
        >,
        asset_server: Res<AssetServer>,
        famiq_res: Res<FamiqWidgetResource>,
        root_q: Query<Entity, With<IsFaWidgetRoot>>
    ) {
        for (entity, change_color, right_side, id, class) in fps_q.iter() {
            let font_handle: Handle<Font> = asset_server.load(&famiq_res.font_path);
            let id_ref = id.map(|s| s.0.clone());
            let class_ref = class.map(|s| s.0.clone());

            let container_entity = FaFpsText::_build_container(&id_ref, &class_ref, right_side.0, &mut commands);
            let count_entity = FaFpsText::_build_text_count(&id_ref, &class_ref, &mut commands, font_handle.clone(), change_color.0);

            let label_txt_font = TextFont {
                font: font_handle,
                font_size: DEFAULT_FPS_TEXT_SIZE,
                ..default()
            };
            commands
                .entity(entity)
                .add_child(count_entity)
                .insert((
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
                    Visibility::Inherited,
                    WidgetStyle::default(),
                    ExternalStyleHasChanged(false)
                ));

            entity_add_child(&mut commands, entity, container_entity);

            if let Ok(root_entity) = root_q.get_single() {
                commands.entity(root_entity).add_child(container_entity);
            }
        }
    }
}

/// Builder for creating an FPS text widget.
pub struct FaFpsTextBuilder<'w, 's> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub change_color: bool,
    pub right_side: bool,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> FaFpsTextBuilder<'w, 's> {
    pub fn new(commands: Commands<'w, 's>) -> Self {
        Self {
            id: None,
            class: None,
            commands,
            change_color: false,
            right_side: false
        }
    }

    /// Enables dynamic color changes based on FPS value.
    pub fn change_color(mut self) -> Self {
        self.change_color = true;
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
        self.right_side = true;
        self
    }

    /// Spawn fps into UI World.
    pub fn build(&mut self) -> Entity {
        let entity = self.commands.spawn((
            IsFamiqFPSTextLabel,
            CanChangeColor(self.change_color),
            RightSide(self.right_side)
        ))
        .id();
        insert_id_and_class(&mut self.commands, entity, &self.id, &self.class);
        entity
    }
}

/// API to create an `FaFpsTextBuilder`.
pub fn fa_fps<'w, 's>(commands: &'w mut Commands) -> FaFpsTextBuilder<'w, 's>
where
    'w: 's
{
    FaFpsTextBuilder::new(commands.reborrow())
}

/// a system to check if FPS internal system(s) can run.
///
/// True only if fps widget is created.
pub fn can_run_fps_systems(fps_q: Query<&IsFamiqFPSTextContainer>) -> bool {
    fps_q.iter().count() > 0
}
