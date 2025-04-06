//! Famiq's plugin.

use crate::event_writer;
use crate::resources::*;
use crate::widgets::{
    list_view::*,
    selection::*,
    text_input::*,
    fps::*,
    circular::*,
    modal::*,
    text::*,
    progress_bar::*,
    bg_image::*,
    *
};

use bevy::utils::Duration;
use bevy::time::common_conditions::on_timer;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::asset::embedded_asset;
use bevy::winit::cursor::CursorIcon;
use bevy::window::{SystemCursorIcon, WindowResized};
use cosmic_text::{FontSystem, SwashCache};

pub enum CursorType {
    Pointer,
    Text,
    Default
}

#[derive(Resource)]
pub struct CursorIcons {
    pub pointer: CursorIcon,
    pub text: CursorIcon,
    pub normal: CursorIcon
}

impl Default for CursorIcons {
    fn default() -> Self {
        Self {
            pointer: SystemCursorIcon::Pointer.into(),
            text: SystemCursorIcon::Text.into(),
            normal: SystemCursorIcon::Default.into(),
        }
    }
}

fn handle_window_resized_system(
    mut resize_events: EventReader<WindowResized>,
    mut fa_bg_q: Query<&mut Sprite, With<IsFamiqBgImage>>
) {
    for resize_event in resize_events.read() {
        if let Ok(mut sprite) = fa_bg_q.get_single_mut() {
            sprite.custom_size = Some(Vec2::new(resize_event.width, resize_event.height));
        }
    }
}

fn external_styles_file_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            style::detect_widget_external_styles_change,
            style::detect_text_external_styles_change
        )
        .chain()
    );

    app.add_systems(
        PreUpdate,
        style::read_styles_from_file_system.run_if(hot_reload_is_enabled)
    );

    app.add_systems(
        PreUpdate,
        (
            style::read_styles_from_file_system,
            style::finish_style_applying_system
        )
        .chain()
        .run_if(hot_reload_is_disabled)
    );
}

fn fa_selection_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_show_and_hide_choices_panel,
            handle_selection_choice_interaction_system,
            detect_new_selection_widget_system
        )
        .run_if(can_run_selection_systems)
    );
}

fn fa_text_input_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaTextInput::handle_text_input_on_typing,
            FaTextInput::detect_text_input_text_style_change.after(FaTextInput::detect_new_text_input_widget_system),
            FaTextInput::handle_text_input_on_focused,
            FaTextInput::handle_cursor_blink_system,
            FaTextInput::detect_new_text_input_widget_system,
        )
        .run_if(can_run_text_input_systems)
    );
    app.add_systems(PostUpdate, FaTextInput::on_request_redraw_editor_buffer);
}

fn fa_text_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaText::update_text_value_system,
            FaText::detect_new_text_widget_system
        )
        .run_if(can_run_text_systems)
    );
}

fn fa_listview_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaListView::on_hover_system,
            FaListView::on_scroll_system,
        )
        .chain()
        .run_if(can_run_list_view_systems)

    );
    app.add_systems(
        Update,
        FaListView::detect_new_listview_system.run_if(can_run_list_view_systems)
    );
}

fn fa_fps_text_systems(app: &mut App) {
    app.add_systems(
        Update,
        // update fps every 450 millisecond, default Update schedule is too fast
        FaFpsText::update_fps_count_system.run_if(
            on_timer(Duration::from_millis(450)).and(can_run_fps_systems)
        )
    );
}

fn fa_circular_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaCircular::detect_new_circular_widget_system,
            FaCircular::_update_circular_material_u_time
        )
        .run_if(can_run_circular_systems)
    );
}

fn fa_progress_bar_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaProgressBar::handle_progress_value_change,
            FaProgressBar::detect_new_progress_bar_widget_system,
            FaProgressBar::_update_progress_bar_material_u_time
        )
        .run_if(can_run_fa_progress_bar_systems)
    );
}

fn fa_bg_image_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaBgImage::detect_new_bg_image_system,
            FaBgImage::handle_image_changed
        )
        .run_if(can_run_bg_image_systems)
    );
}

pub struct FamiqPlugin;

impl Plugin for FamiqPlugin {
    fn build(&self, app: &mut App) {
        // embedded assets
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-medium.ttf");
        embedded_asset!(app, "embedded_assets/shaders/progress_bar.wgsl");
        embedded_asset!(app, "embedded_assets/shaders/circular.wgsl");
        embedded_asset!(app, "embedded_assets/shaders/text_input.wgsl");
        embedded_asset!(app, "embedded_assets/logo.jpeg"); // for testing

        app.add_systems(PreStartup, _spawn_root_node);
        app.add_systems(PostStartup, adjust_position_system);
        app.add_systems(Update, detect_new_widget_with_id);
        app.add_systems(Update, handle_window_resized_system);
        app.add_systems(Update, FaModal::hide_or_display_modal_system.run_if(can_run_modal_systems));
        app.add_systems(PostUpdate, detect_reactive_data_change);

        app.add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default());
        app.add_plugins(UiMaterialPlugin::<CircularMaterial>::default());
        app.add_plugins(UiMaterialPlugin::<TextInputMaterial>::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(RData::default());
        app.insert_resource(StylesKeyValueResource::default());
        app.insert_resource(FamiqResource::new());
        app.insert_resource(CosmicFontSystem(FontSystem::new()));
        app.insert_resource(CosmicSwashCache(SwashCache::new()));
        app.insert_resource(FaBgImageResource::default());
        app.insert_resource(CanBeScrolledListView { entity: None });
        app.insert_resource(FaSelectionResource::default());
        app.insert_resource(FaTextInputResource::default());
        app.insert_resource(FaProgressBarResource::default());
        app.insert_resource(FaModalState::default());
        app.insert_resource(FaTextResource::default());
        app.insert_resource(CursorIcons::default());

        app.add_event::<event_writer::FaValueChangeEvent>();
        app.add_event::<event_writer::FaMouseEvent>();

        app.add_event::<RequestRedrawBuffer>();

        external_styles_file_systems(app);
        fa_text_systems(app);
        fa_selection_systems(app);
        fa_listview_systems(app);
        fa_text_input_systems(app);
        fa_fps_text_systems(app);
        fa_circular_systems(app);
        fa_progress_bar_systems(app);
        fa_bg_image_systems(app);
    }
}

/// Detect when a widget with id is created
fn detect_new_widget_with_id(widget_q: Query<&FamiqWidgetId, Added<IsFamiqMainWidget>>) {
    let mut ids: Vec<&str> = Vec::new();

    for id in widget_q.iter() {
        if !ids.contains(&id.0.as_str()) {
            ids.push(&id.0);
        }
        else {
            panic!("\n[FamiqError]: ID {:?} is duplicated\n", id.0);
        }
    }
}

fn _spawn_root_node(mut commands: Commands, mut res: ResMut<FamiqResource>) {
    commands.spawn(Camera2d::default());

    let entity = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            ..default()
        },
        IsFaWidgetRoot,
        GlobalZIndex(1)
    )).id();

    res.root_node_entity = Some(entity);
}

// to fix positioning and scaling issue on different platforms
fn adjust_position_system(mut query: Query<&mut Transform>, windows: Single<&Window>) {
    let scale_factor = windows.scale_factor();

    for mut transform in query.iter_mut() {
        transform.translation.x *= scale_factor;
        transform.translation.y *= scale_factor;
    }
}
