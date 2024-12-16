use crate::event_handler;
use crate::event_writer;
use crate::widgets::{list_view::*, selection::*, text_input::*, *};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
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
            event_writer::selection_item_interaction_system,
            event_handler::handle_selection_interaction_system,
            event_handler::handle_selection_item_interaction_system,
            update_selector_placeholder_color_system,
            update_selector_arrow_icon_system,
            update_selection_items_panel_visibility_system,
            update_selection_items_panel_position_and_width_system,
        ),
    );
}

fn fa_text_input_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::text_input_interaction_system,
            event_handler::handle_text_input_on_click_system,
            event_handler::handle_text_input_on_typing_system,
        ),
    );
}

fn fa_button_systems(app: &mut App) {
    app.add_systems(Update, event_writer::btn_interaction_system);
}

fn fa_text_systems(app: &mut App) {
    app.add_systems(Update, event_writer::text_interaction_system);
}

fn fa_listview_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            event_writer::listview_interaction_system,
            event_writer::listview_item_interaction_system,
            FaListView::on_hover_system,
            FaListView::on_mouse_scroll_system,
        )
            .chain(),
    );
}

fn fa_fps_text_systems(app: &mut App) {
    // run system every 10 millisecond, Update Scedule is too fast
    app.insert_resource(Time::<Fixed>::from_seconds(0.10));
    app.add_systems(FixedUpdate, fps::FaFpsText::update_fps_count_system);
}

pub fn famiq_plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.insert_resource(StylesKeyValueResource(StylesKeyValue::new()));
    app.insert_resource(style::ExternalStylesApplyState(false));
    app.insert_resource(CanBeScrolledListView { entity: None });
    app.insert_resource(SelectedItemsResource {
        items: HashMap::new(),
    });
    app.insert_resource(TextInputResource {
        inputs: HashMap::new(),
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
}
