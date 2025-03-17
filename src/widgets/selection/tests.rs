#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::color::PRIMARY_COLOR;
use crate::widgets::FamiqResource;
use super::*;

fn setup_test_default_selection(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    fa_selection(&mut builder, "Test select choice").id("#test-selection").build();
}

fn setup_test_selection_with_built_in_class_color(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    fa_selection(&mut builder, "Test select choice")
        .class("is-primary")
        .build();
}

fn setup_test_selection_with_choices(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    fa_selection(&mut builder, "Test select choice")
        .choices(vec!["Test one", "Test two"])
        .build();
}

#[test]
fn test_create_default_selection() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_selection);
    app.update();

    let selection_q = app.world_mut()
        .query::<(&FamiqWidgetId, &IsFamiqSelectionSelector)>()
        .get_single(app.world());

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
        .get_single(app.world());

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
        .get_single(app.world());

    // 2 provided choices, 1 default "-/-"
    assert_eq!(3 as usize, panel_q.unwrap().0.len());
}

#[test]
fn test_get_value_by_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_selection);
    app.update();

    let selection_res = app.world_mut().resource::<FaSelectionResource>();
    let value = selection_res.get_value("#test-selection");

    assert_eq!("".to_string(), value);
}
