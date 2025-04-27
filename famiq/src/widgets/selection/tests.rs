#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::color::PRIMARY_COLOR;
use crate::widgets::FamiqResource;
use crate::selection;
use super::*;

fn setup_test_default_selection(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    selection!(placeholder: "Test select choice", id: "#test-selection");
}

fn setup_test_selection_with_built_in_class_color(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    selection!(placeholder: "Test select choice", class: "primary");
}

fn setup_test_selection_with_choices(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    selection!(
        placeholder: "Test select choice",
        choices: ["Test one", "Test two"]
    );
}

#[test]
fn test_create_default_selection() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_selection);
    app.update();

    let selection_q = app.world_mut()
        .query::<(&WidgetId, &IsFamiqSelectionSelector)>()
        .single(app.world());

    assert!(selection_q.is_ok(), "There should be only 1 selection");

    let selection_id = selection_q.unwrap().0;
    assert_eq!("#test-selection".to_string(), selection_id.0);
}

#[test]
fn test_create_selection_with_built_in_class_color() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_selection_with_built_in_class_color);
    app.update();

    let selector_q = app.world_mut()
        .query::<(&BackgroundColor, &IsFamiqSelectionSelector)>()
        .single(app.world());

    let selector_bg = selector_q.unwrap().0;
    assert_eq!(
        BackgroundColor(PRIMARY_COLOR),
        *selector_bg
    );
}

#[test]
fn test_create_selection_with_choices() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_selection_with_choices);
    app.update();

    let panel_q = app.world_mut()
        .query::<(&Children, &IsFamiqSelectionChoicesPanel)>()
        .single(app.world());

    // 2 provided choices, 1 default "-/-"
    assert_eq!(3 as usize, panel_q.unwrap().0.len());
}
