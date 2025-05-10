#![cfg(test)]

use crate::utils;
use crate::FamiqPlugin;
use crate::widgets::button::*;
use crate::widgets::{FamiqResource, WidgetId, WidgetClasses, builder_mut};
use super::*;

fn setup_test_default_container(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    crate::container!(id: "#test-container");
}

fn setup_test_container_with_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    crate::container!(
        id: "#test-container",
        class: "test-class-one test-class-two"
    );
}

fn setup_test_container_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    crate::container!(
        id: "#test-container",
        children: [
            crate::button!(text: "Button 1"),
            crate::button!(text: "Button 2")
        ]
    );
}

#[test]
fn test_create_default_container() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_default_container);
    app.update();

    let container_q = app.world_mut().query::<(&WidgetId, &IsFamiqContainer)>().single(app.world());
    assert!(container_q.is_ok(), "There should be only 1 container");

    let container_id = container_q.unwrap().0;
    assert_eq!("#test-container".to_string(), container_id.0);
}

#[test]
fn test_create_container_with_class() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_container_with_class);
    app.update();

    let container_q = app.world_mut().query::<(&WidgetClasses, &IsFamiqContainer)>().single(app.world());
    assert!(container_q.is_ok(), "There should be only 1 container");

    let container_class = container_q.unwrap().0;
    assert_eq!("test-class-one test-class-two".to_string(), container_class.0);
}

#[test]
fn test_create_container_with_children() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_container_with_children);
    app.update();

    let container_q = app.world_mut()
        .query::<(&Children, &IsFamiqContainer)>()
        .single(app.world());

    assert_eq!(2 as usize, container_q.unwrap().0.len());
}
