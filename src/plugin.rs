use crate::event_writer;
use crate::widgets::{
    list_view::*,
    selection::*,
    text_input::*,
    fps::*,
    button::*,
    circular::*,
    modal::*,
    text::*,
    tooltip::*,
    progress_bar::*,
    *
};

use bevy::utils::Duration;
use bevy::time::common_conditions::on_timer;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::asset::embedded_asset;

fn external_styles_file_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            style::read_styles_from_file_system,
            style::detect_external_style_changes,
            style::apply_widgets_styles_system,
            style::apply_text_style_system
        )
            .chain()
            .run_if(hot_reload_is_enabled)
    );
    app.add_systems(
        Update,
        (
            style::read_styles_from_file_system,
            style::inject_external_style,
            style::apply_widgets_styles_system,
            style::apply_text_style_system,
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
            event_writer::selection_interaction_system,
            event_writer::selection_choice_interaction_system,
            update_choices_panel_position_and_width_system,
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
            FaText::detect_new_text_widget_system
        )
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
        ,
    );
}

fn fa_fps_text_systems(app: &mut App) {
    // update fps every 450 millisecond, default Update schedule is too fast
    app.add_systems(
        Update,
        FaFpsText::update_fps_count_system.run_if(
            on_timer(Duration::from_millis(450)).and(can_run_fps_systems)

        )
    );
}

fn fa_circular_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::circular_interaction_system,
            FaCircular::rotate_spinner,
            FaCircular::update_spinner_speed,
            FaCircular::handle_circular_interaction_system
        )
        .run_if(can_run_circular_systems)
    );
}

fn fa_modal_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaModal::hide_or_display_modal_system
        )
        .run_if(can_run_modal_systems)
    );
}

fn fa_image_systems(app: &mut App) {
    app.add_systems(Update, event_writer::image_interaction_system);
}

fn fa_progress_bar_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            FaProgressBar::move_progress_value_as_indeterminate_system
                .run_if(can_move_progress_value_as_indeterminate_system),

            FaProgressBar::handle_progress_value_change_by_id
                .run_if(can_run_handle_progress_value_change),

            FaProgressBar::handle_progress_value_change_by_entity
                .run_if(can_run_handle_progress_value_change),

            FaProgressBar::detect_new_progress_bar_widget_system
                .run_if(can_run_handle_progress_value_change),
        )
    );
}

pub struct FamiqPlugin;

impl Plugin for FamiqPlugin {
    fn build(&self, app: &mut App) {
        // embedded assets
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-regular.ttf");
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-medium.ttf");
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-bold.ttf");
        embedded_asset!(app, "embedded_assets/logo.jpeg"); // for testing

        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(StylesKeyValueResource(StylesKeyValue::new()));
        app.insert_resource(CanBeScrolledListView { entity: None });
        app.insert_resource(FaSelectionResource::default());
        app.insert_resource(FaTextInputResource::default());
        app.insert_resource(FaTextInputCursorBlinkTimer::default());
        app.insert_resource(IndeterminateAnimationTimer::default());
        app.insert_resource(FaProgressBarResource::default());
        app.insert_resource(FaToolTipResource::default());
        app.insert_resource(FaModalState::default());
        app.insert_resource(FaTextResource::default());
        app.insert_resource(FamiqWidgetResource::default());

        app.add_event::<event_writer::FaInteractionEvent>();

        external_styles_file_systems(app);
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

        app.add_systems(Update, FaToolTip::handle_show_hide_tooltip_system);
    }
}
