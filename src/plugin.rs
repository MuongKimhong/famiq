//! Famiq's plugin.

use crate::event_writer;
use crate::resources::*;
use crate::widgets::{
    list_view::*,
    selection::*,
    text_input::*,
    fps::*,
    button::*,
    circular::*,
    modal::*,
    text::*,
    progress_bar::*,
    image::*,
    bg_image::*,
    container::*,
    *
};

use bevy::utils::Duration;
use bevy::time::common_conditions::on_timer;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::asset::embedded_asset;
use bevy::winit::cursor::CursorIcon;
use bevy::window::{SystemCursorIcon, WindowResized};

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

fn internal_styles_systems(app: &mut App) {
    app.add_systems(Update, FaStyleResource::detect_new_widget_with_id);
    app.add_systems(
        PostUpdate,
        (
            FaStyleResource::detect_internal_widget_style_change,
            FaStyleResource::detect_internal_text_style_change
        )
        .chain()
    );
}

fn fa_selection_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::selection_interaction_system,
            event_writer::selection_choice_interaction_system,
            handle_show_and_hide_choices_panel,
            handle_selection_interaction_system,
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
            event_writer::text_input_interaction_system,
            event_writer::text_input_toggle_password_icon_interaction_system,
            FaTextInput::handle_text_input_on_typing_system,
            FaTextInput::handle_text_input_interaction_system,
            FaTextInput::handle_text_input_on_focused_system,
            FaTextInput::handle_cursor_blink_system,
            FaTextInput::handle_toggle_password_icon_interaction_system,
            FaTextInput::detect_new_text_input_widget_system
        )
        .run_if(can_run_text_input_systems)
    );
}

fn fa_button_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::btn_interaction_system,
            FaButton::handle_button_on_interaction_system
        )
        .run_if(can_run_button_systems)
    );
}

fn fa_text_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::text_interaction_system,
            FaText::update_text_value_system,
            FaText::detect_new_text_widget_system,
            FaText::handle_text_interaction_system
        )
        .run_if(can_run_text_systems)
    );
}

fn fa_listview_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::listview_interaction_system,
            FaListView::on_hover_system,
            // event_writer::listview_item_interaction_system,
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
        (
            event_writer::fps_interaction_system,

            // update fps every 450 millisecond, default Update schedule is too fast
            FaFpsText::update_fps_count_system.run_if(
                on_timer(Duration::from_millis(450)).and(can_run_fps_systems)
            )
        )
    );
}

fn fa_circular_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::circular_interaction_system,
            FaCircular::handle_circular_interaction_system,
            FaCircular::detect_new_circular_widget_system,
            FaCircular::_update_circular_material_u_time
        )
        .run_if(can_run_circular_systems)
    );
}

fn fa_modal_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaModal::hide_or_display_modal_system,
            FaModal::detect_new_modal_system
        )
        .run_if(can_run_modal_systems)
    );
}

fn fa_image_systems(app: &mut App) {
    app.add_systems(
        Update,
        event_writer::image_interaction_system.run_if(can_run_image_systems)
    );
}

fn fa_progress_bar_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::progress_bar_interaction_system,
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

fn fa_container_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::container_interaction_system,
            FaContainer::detect_new_container_system
        )
        .run_if(can_run_container_systems)
    );
}

pub struct FamiqPlugin;

impl Plugin for FamiqPlugin {
    fn build(&self, app: &mut App) {
        // embedded assets
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-regular.ttf");
        embedded_asset!(app, "embedded_assets/shaders/progress_bar.wgsl");
        embedded_asset!(app, "embedded_assets/shaders/circular.wgsl");
        embedded_asset!(app, "embedded_assets/logo.jpeg"); // for testing

        app.add_systems(PreStartup, _spawn_root_node);
        app.add_systems(Update, handle_window_resized_system);

        app.add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default());
        app.add_plugins(UiMaterialPlugin::<CircularMaterial>::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(StylesKeyValueResource::default());
        app.insert_resource(FamiqResource::new());
        app.insert_resource(FaStyleResource::default());
        app.insert_resource(FaBgImageResource::default());
        app.insert_resource(FaContainableResource::default());
        app.insert_resource(CanBeScrolledListView { entity: None });
        app.insert_resource(FaSelectionResource::default());
        app.insert_resource(FaTextInputResource::default());
        app.insert_resource(FaTextInputCursorBlinkTimer::default());
        app.insert_resource(FaProgressBarResource::default());
        app.insert_resource(FaModalState::default());
        app.insert_resource(FaTextResource::default());
        app.insert_resource(CursorIcons::default());

        app.add_event::<event_writer::FaInteractionEvent>();

        external_styles_file_systems(app);
        internal_styles_systems(app);
        fa_button_systems(app);
        fa_text_systems(app);
        fa_selection_systems(app);
        fa_listview_systems(app);
        fa_text_input_systems(app);
        fa_fps_text_systems(app);
        fa_circular_systems(app);
        fa_modal_systems(app);
        fa_image_systems(app);
        fa_progress_bar_systems(app);
        fa_bg_image_systems(app);
        fa_container_systems(app);

        app.add_systems(
            PostUpdate,
            detect_fa_containable_resource_change
        );
    }
}

fn _spawn_root_node(mut commands: Commands, mut res: ResMut<FamiqResource>) {
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
