#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::color::PRIMARY_COLOR;
use crate::widgets::FamiqWidgetClasses;
use crate::utils;
use bevy::input::InputPlugin;
use super::*;

fn setup_test_default_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_text_input(&mut builder, "First name").id("#test-input").build();
}

fn setup_test_input_with_built_in_class(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_text_input(&mut builder, "First name")
        .class("is-primary")
        .build();
}

#[test]
fn test_create_default_input() {
    let mut app = utils::create_test_app();
    app.add_plugins(InputPlugin::default());
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_input);
    app.update();

    let input_q = app.world_mut()
        .query::<(&FamiqWidgetId, &IsFamiqTextInput)>()
        .get_single(app.world());

    let input_id = input_q.unwrap().0;
    assert_eq!("#test-input".to_string(), input_id.0);
}

#[test]
fn test_create_input_with_built_in_class() {
    let mut app = utils::create_test_app();
    app.add_plugins(InputPlugin::default());
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_input_with_built_in_class);
    app.update();

    let input_q = app.world_mut()
        .query::<(&FamiqWidgetClasses, &BackgroundColor, &IsFamiqTextInput)>()
        .get_single(app.world());

    let input_class = input_q.as_ref().unwrap().0;
    assert_eq!("is-primary".to_string(), input_class.0);

    let input_bg = input_q.as_ref().unwrap().1;
    assert_eq!(BackgroundColor(PRIMARY_COLOR), *input_bg);
}

#[test]
fn test_get_value_by_id() {
    let mut app = utils::create_test_app();
    app.add_plugins(InputPlugin::default());
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_input);
    app.update();

    let input_res = app.world_mut().resource::<FaTextInputResource>();
    let value = input_res.get_value("#test-input");

    // default value is empty string
    assert_eq!("".to_string(), value);
}
