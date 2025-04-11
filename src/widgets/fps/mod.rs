pub mod helper;
pub mod tests;

use crate::utils::{entity_add_child, insert_id_and_class, process_spacing_built_in_class};
use crate::widgets::*;
use crate::event_writer::*;
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
                IsFamiqMainWidget,
                IsFamiqFPSTextLabel,
                style_components.clone(),
                GlobalZIndex(6),
                DefaultWidgetEntity::from(style_components)
            ))
            .observe(FaFpsText::handle_on_mouse_over)
            .observe(FaFpsText::handle_on_mouse_out)
            .id();

        let count_txt_entity = root_node
            .commands()
            .spawn((
                TextSpan::default(),
                count_txt_font.clone(),
                TextColor(WHITE_COLOR),
                IsFamiqFPSTextCount,
                CanChangeColor(change_color),
                DefaultTextSpanEntity::new(
                    TextSpan::default(),
                    count_txt_font,
                    TextColor(WHITE_COLOR),
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

    fn handle_on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut writer: EventWriter<FaMouseEvent>,
        fps_q: Query<Option<&FamiqWidgetId>, With<IsFamiqFPSTextLabel>>
    ) {
        if let Ok(id) = fps_q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::FpsText, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut writer: EventWriter<FaMouseEvent>,
        fps_q: Query<Option<&FamiqWidgetId>, With<IsFamiqFPSTextLabel>>
    ) {
        if let Ok(id) = fps_q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::FpsText, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    /// Internal system to update the FPS count and optionally change its color based on the value.
    pub fn update_fps_count_system(
        diagnostics: Res<DiagnosticsStore>,
        mut text_q: Query<(&mut TextSpan, &mut TextColor, &CanChangeColor, &IsFamiqFPSTextCount)>
    ) {
        for (mut text, mut color, change_color, _) in text_q.iter_mut() {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    text.0 = format!("{value:.0}");

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
                    // else {
                    //     color.0 = WHITE_COLOR;
                    // }
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
    pub fn change_color(mut self, can_change: bool) -> Self {
        self.change_color = can_change;
        self
    }

    /// Aligns the FPS widget to the right or left top corner of the screen.
    pub fn side(mut self, right: bool) -> Self {
        self.right_side = right;
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
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create an `FaFpsTextBuilder`.
pub fn fa_fps_builder<'a>(builder: &'a mut FamiqBuilder) -> FaFpsTextBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaFpsTextBuilder::new(
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

#[macro_export]
macro_rules! fa_fps {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let builder = builder_mut();
        let mut fps = fa_fps_builder(builder);
        $(
            $crate::fa_fps_attributes!(fps, $key : $value);
        )*
        fps.build()
    }};
}

#[macro_export]
macro_rules! fa_fps_attributes {
    ($fps:ident, color: $color:expr) => {{
        $fps = $fps.color($color);
    }};

    ($fps:ident, right_side: $right_side:expr) => {{
        $fps.right_side = $right_side;
    }};

    ($fps:ident, change_color: $change_color:expr) => {{
        $fps.change_color = $change_color;
    }};

    // common attributes
    ($fps:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($fps, $key : $value);
    }};
}

/// a system to check if FPS internal system(s) can run.
///
/// True only if fps widget is created.
pub fn can_run_fps_systems(fps_q: Query<&IsFamiqFPSTextLabel>) -> bool {
    !fps_q.is_empty()
}
