use crate::event_writer;
use crate::widgets::{
    list_view::*,
    selection::*,
    text_input::*,
    fps::*,
    button::*,
    circular::*,
    modal::*,
    container::*,
    *
};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::asset::embedded_asset;
use bevy::utils::HashMap;

fn external_styles_file_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            style::read_styles_from_file_system,
            style::apply_widgets_styles_system,
            style::apply_text_style_system,
            style::finish_style_applying_system,
        )
            .chain(),
    );
}

fn fa_selection_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::selection_interaction_system,
            event_writer::selection_choice_interaction_system,
            update_selector_placeholder_color_system,
            update_selector_arrow_icon_system,
            update_selection_choices_panel_visibility_system,
            update_choices_panel_position_and_width_system,
            handle_selection_interaction_system,
            handle_selection_choice_interaction_system,
        ),
    );
}

fn fa_text_input_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::text_input_interaction_system,
            FaTextInput::handle_text_input_on_typing_system,
            FaTextInput::handle_text_input_interaction_system,
            FaTextInput::handle_text_input_cursor_on_focused_system,
            FaTextInput::handle_cursor_blink_system
        ),
    );
}

fn fa_button_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::btn_interaction_system,
            FaButton::handle_button_on_interaction_system
        )
    );
}

fn fa_text_systems(app: &mut App) {
    app.add_systems(Update, event_writer::text_interaction_system);
}

fn fa_listview_systems(app: &mut App) {
    app.add_systems(FixedUpdate, FaListView::on_hover_system);
    app.add_systems(
        Update,
        (
            event_writer::listview_interaction_system,
            event_writer::listview_item_interaction_system,
            FaListView::on_scroll_system,
        )
            .chain(),
    );
}

fn fa_fps_text_systems(app: &mut App) {
    // run system every 30 millisecond, Update Scedule is too fast
    app.add_systems(FixedUpdate, FaFpsText::update_fps_count_system);
}

fn fa_circular_systems(app: &mut App) {
    app.add_systems(Update, (FaCircular::rotate_spinner, FaCircular::update_spinner_speed));
}

fn fa_modal_systems(app: &mut App) {
    app.add_systems(Update, FaModal::hide_or_display_modal_system);
}

fn fa_image_systems(app: &mut App) {
    app.add_systems(Update, event_writer::image_interaction_system);
}

fn fa_container_systems(app: &mut App) {
    app.add_systems(Update, FaContainer::handle_container_on_interaction_system);
}

pub struct FamiqPlugin;

impl Plugin for FamiqPlugin {
    fn build(&self, app: &mut App) {
        // embedded assets
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-regular.ttf");
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-medium.ttf");
        embedded_asset!(app, "embedded_assets/fonts/fira-mono-bold.ttf");

        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.insert_resource(Time::<Fixed>::from_seconds(0.30));
        app.insert_resource(StylesKeyValueResource(StylesKeyValue::new()));
        app.insert_resource(CanBeScrolledListView { entity: None });
        app.insert_resource(SelectedChoicesResource {
            choices: HashMap::new(),
        });
        app.insert_resource(FaTextInputResource {
            inputs: HashMap::new(),
        });
        app.insert_resource(FaTextInputCursorBlinkTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            is_transparent: false
        });
        app.insert_resource(FamiqWidgetBuilderResource::default());

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
        fa_container_systems(app);
    }
}
