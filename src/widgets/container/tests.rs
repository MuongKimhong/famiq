#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::fa_container;
use crate::fa_button;
use crate::widgets::{FamiqResource, FamiqWidgetId, FamiqWidgetClasses};
use super::*;

fn setup_test_default_container(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);

    fa_container!(&mut builder, id: "#test-container");
}

fn setup_test_container_with_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    fa_container!(
        &mut builder,
        id: "#test-container",
        class: "test-class-one test-class-two"
    );
}

fn setup_test_container_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);

    let test_btn_1 = fa_button!(&mut builder, text: "Button 1");
    let test_btn_2 = fa_button!(&mut builder, text: "Button 2");

    fa_container!(
        &mut builder,
        id: "#test-container",
        children: [test_btn_1, test_btn_2]
    );
}

#[test]
fn test_create_default_container() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_container);
    app.update();

    let container_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqContainer)>().get_single(app.world());
    assert!(container_q.is_ok(), "There should be only 1 container");

    let container_id = container_q.unwrap().0;
    assert_eq!("#test-container".to_string(), container_id.0);
}

#[test]
fn test_create_container_with_class() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_container_with_class);
    app.update();

    let container_q = app.world_mut().query::<(&FamiqWidgetClasses, &IsFamiqContainer)>().get_single(app.world());
    assert!(container_q.is_ok(), "There should be only 1 container");

    let container_class = container_q.unwrap().0;
    assert_eq!("test-class-one test-class-two".to_string(), container_class.0);
}

#[test]
fn test_create_container_with_children() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_container_with_children);
    app.update();

    let container_q = app.world_mut()
        .query::<(&Children, &IsFamiqContainer)>()
        .get_single(app.world());

    assert_eq!(2 as usize, container_q.unwrap().0.len());
}
