#![cfg(test)]

use crate::FamiqPlugin;
use crate::widgets::color::PRIMARY_COLOR;
use crate::widgets::WidgetClasses;
use crate::utils;
use crate::text_input;
use bevy::input::InputPlugin;
use super::*;

fn setup_test_default_input(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    text_input!(placeholder: "First name", id: "#test-input");
}

fn setup_test_input_with_built_in_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    text_input!(placeholder: "First name", class: "primary");
}

#[test]
fn test_create_default_input() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_plugins(InputPlugin::default());
    app.add_systems(Startup, setup_test_default_input);
    app.update();

    let input_q = app.world_mut()
        .query::<(&WidgetId, &IsFamiqTextInput)>()
        .single(app.world());

    let input_id = input_q.unwrap().0;
    assert_eq!("#test-input".to_string(), input_id.0);
}

#[test]
fn test_create_input_with_built_in_class() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_plugins(InputPlugin::default());
    app.add_systems(Startup, setup_test_input_with_built_in_class);
    app.update();

    let input_q = app.world_mut()
        .query::<(&WidgetClasses, &BackgroundColor, &IsFamiqTextInput)>()
        .single(app.world());

    let input_class = input_q.as_ref().unwrap().0;
    assert_eq!("primary".to_string(), input_class.0);

    let input_bg = input_q.as_ref().unwrap().1;
    assert_eq!(BackgroundColor(PRIMARY_COLOR), *input_bg);
}
