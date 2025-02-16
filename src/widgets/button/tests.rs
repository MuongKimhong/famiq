#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::{FamiqWidgetId, FamiqWidgetClasses};
use super::*;

fn setup_test_default_button(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_button(&mut builder, "Press me").id("#test-btn").build();
}

fn setup_test_button_with_built_in_class(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_button(&mut builder, "Press me")
        .id("#test-btn")
        .class("is-primary is-large is-round")
        .build();
}

#[test]
fn test_create_default_button() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_button);
    app.update();

    let btn_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqButton)>().get_single(app.world());
    assert!(btn_q.is_ok(), "There should be only 1 button");

    let btn_id = btn_q.unwrap().0;
    assert_eq!(
        "#test-btn".to_string(),
        btn_id.0
    );

    let btn_text_q = app.world_mut().query::<(&Text, &IsFamiqButtonText)>()
                    .get_single(app.world());

    assert_eq!(
        "Press me".to_string(),
        btn_text_q.unwrap().0.0
    );
}

#[test]
fn test_create_button_with_built_in_class() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_button_with_built_in_class);
    app.update();

    let btn_q = app.world_mut().query::<(&FamiqWidgetClasses, &IsFamiqButton)>().get_single(app.world());
    assert_eq!(
        "is-primary is-large is-round".to_string(),
        btn_q.unwrap().0.0
    );
}
