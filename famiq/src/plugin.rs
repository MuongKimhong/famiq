//! Famiq's plugin.

use crate::event_writer;
use crate::resources::*;
use crate::reactivity::*;
use crate::widgets::{
    selection::*,
    text_input::*,
    scroll::*,
    fps::*,
    circular::*,
    dialog::*,
    progress_bar::*,
    *
};

use std::time::Duration;
use bevy::time::common_conditions::on_timer;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::asset::embedded_asset;
use bevy::winit::cursor::CursorIcon;
use bevy::window::SystemCursorIcon;
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

// fn handle_window_resized_system(
//     mut resize_events: EventReader<WindowResized>,
//     mut fa_bg_q: Query<&mut Sprite, With<IsFamiqBgImage>>
// ) {
//     for resize_event in resize_events.read() {
//         if let Ok(mut sprite) = fa_bg_q.get_single_mut() {
//             sprite.custom_size = Some(Vec2::new(resize_event.width, resize_event.height));
//         }
//     }
// }

fn external_styles_file_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            style::detect_widget_external_styles_change,
            style::detect_text_external_styles_change
        )
        .chain()
    );

    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_systems(
            PreUpdate,
            style::read_styles_from_file_system.run_if(hot_reload_is_enabled)
        );
        app.add_systems(
            PreUpdate,
            (
                style::read_styles_from_file_system,
                style::finish_style_apply_system
            )
            .chain()
            .run_if(hot_reload_is_disabled)
        );
    }

    #[cfg(target_arch = "wasm32")]
    {
        app.add_systems(PreUpdate, style::load_json_style_asset_wasm);
    }
}

fn fa_selection_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_show_and_hide_choices_panel,
            handle_selection_choice_interaction_system,
            detect_selection_reactive_model_change
        )
        .run_if(can_run_selection_systems)
    );
}

fn fa_text_input_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_text_input_on_typing,
            detect_text_input_text_style_change.after(detect_new_text_input_widget_system),
            handle_text_input_on_focused,
            handle_cursor_blink_system,
            detect_new_text_input_widget_system,

            #[cfg(target_arch = "wasm32")]
            on_wasm_paste,
        )
        .run_if(cosmic_font_system_exists.and(can_run_text_input_systems))
    );
    app.add_systems(
        PostUpdate,
        on_request_redraw_editor_buffer
            .run_if(cosmic_font_system_exists.and(can_run_text_input_systems))
    );
}

fn fa_scroll_systems(app: &mut App) {
    app.add_systems(
        Update,
        (on_hover_system, on_scroll_system).chain().run_if(can_run_scroll_systems)
    );
    app.add_systems(Update, detect_new_scroll_system.run_if(can_run_scroll_systems));
}

fn fa_fps_text_systems(app: &mut App) {
    app.add_systems(
        Update,
        // update fps every 450 millisecond, default Update schedule is too fast
        FpsBuilder::update_fps_count_system.run_if(
            on_timer(Duration::from_millis(450)).and(can_run_fps_systems)
        )
    );
}

fn fa_circular_systems(app: &mut App) {
    app.add_systems(
        Update,
        (detect_new_circular, update_circular_material_u_time).run_if(can_run_circular_systems)
    );
}

fn fa_progress_bar_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            detect_new_progress_bar,
            detect_reactive_model_change,
            update_progress_bar_material_u_time
        )
        .run_if(can_run_fa_progress_bar_systems)
    );
}

pub struct FamiqPlugin {
    /// Setup Camera2d by default
    pub default_camera: bool
}

impl FamiqPlugin {
    pub fn new() -> Self {
        Self {
            default_camera: true
        }
    }

    pub fn new_no_camera() -> Self {
        Self {
            default_camera: false
        }
    }
}

impl Plugin for FamiqPlugin {
    fn build(&self, app: &mut App) {
        // embedded assets
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-medium.ttf");
        embedded_asset!(app, "embedded_assets/shaders/progress_bar.wgsl");
        embedded_asset!(app, "embedded_assets/shaders/circular.wgsl");
        embedded_asset!(app, "embedded_assets/shaders/text_input.wgsl");

        if self.default_camera {
            app.add_systems(PreStartup, _spawn_root_node_camera);
        } else {
            app.add_systems(PreStartup, _spawn_root_node_no_camera);
        }
        app.add_systems(PostStartup, adjust_position_system);
        app.add_systems(Update, detect_new_widget_with_id);
        // app.add_systems(Update, handle_window_resized_system);
        app.add_systems(
            Update,
            (
                hide_or_display_dialog_system,
                detect_dialog_reactive_model_change
            )
            .run_if(can_run_dialog_systems)

        );
        app.add_systems(PostUpdate, on_update_subscriber_event);
        app.add_systems(PostUpdate, detect_reactive_data_change);

        app.add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default());
        app.add_plugins(UiMaterialPlugin::<CircularMaterial>::default());
        app.add_plugins(UiMaterialPlugin::<TextInputMaterial>::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(RData::default());
        app.insert_resource(StylesKeyValueResource::default());
        app.insert_resource(FamiqResource::new());
        // app.insert_resource(ContainableChildren::default());

        #[cfg(not(target_arch = "wasm32"))]
        {
            app.insert_resource(CosmicFontSystem(FontSystem::new_with_fonts([])));
        }

        #[cfg(target_arch = "wasm32")]
        {
            let (tx, rx) = crossbeam_channel::bounded::<WasmPaste>(1);
            app.insert_resource(WasmPasteAsyncChannel { tx, rx });
        }

        app.insert_resource(CosmicSwashCache(SwashCache::new()));
        app.insert_resource(RSubscriber::default());
        app.insert_resource(CanBeScrolled { entity: None });
        app.insert_resource(FaDialogState::default());
        app.insert_resource(CursorIcons::default());

        app.init_resource::<JsonStyleAssetState>();
        app.init_asset::<JsonStyleAsset>();
        app.init_asset_loader::<JsonStyleAssetLoader>();

        app.add_event::<event_writer::FaMouseEvent>();
        app.add_event::<RequestRedrawBuffer>();
        app.add_event::<UpdateReactiveSubscriberEvent>();

        external_styles_file_systems(app);
        fa_selection_systems(app);
        fa_scroll_systems(app);
        fa_text_input_systems(app);
        fa_fps_text_systems(app);
        fa_circular_systems(app);
        fa_progress_bar_systems(app);
    }
}

/// Detect when a widget with id is created
fn detect_new_widget_with_id(widget_q: Query<&WidgetId, Added<MainWidget>>) {
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

fn _spawn_root_node_camera(mut commands: Commands, mut res: ResMut<FamiqResource>) {
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

fn _spawn_root_node_no_camera(mut commands: Commands, mut res: ResMut<FamiqResource>) {
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
