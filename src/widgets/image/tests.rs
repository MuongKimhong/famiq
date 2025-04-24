#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::{FamiqResource, WidgetId, WidgetClasses, inject_builder, builder_mut};
use crate::utils::{get_embedded_asset_path, create_test_app};
use crate::image;
use super::*;

fn setup_test_default_image(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);
    let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
    image!(path: &path, id: "#test-image");
}

fn setup_test_image_with_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);
    let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
    image!(path: &path, class: "test-class-one");
}

fn setup_test_image_with_custom_size(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);
    let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
    image!(path: &path, width: "200px", height: "200px");
}

#[test]
fn test_create_default_image() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_image);
    app.update();

    let img_q = app.world_mut().query::<(&WidgetId, &IsFamiqImage)>().get_single(app.world());
    assert!(img_q.is_ok(), "There should be only 1 Image");

    let img_id = img_q.unwrap().0;
    assert_eq!("#test-image".to_string(), img_id.0);
}

#[test]
fn test_create_image_with_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_image_with_class);
    app.update();

    let img_q = app.world_mut().query::<(&WidgetClasses, &IsFamiqImage)>().get_single(app.world());
    assert!(img_q.is_ok(), "There should be only 1 Image");
    assert_eq!("test-class-one".to_string(), img_q.unwrap().0.0);
}

#[test]
fn test_create_image_with_custom_size() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_image_with_custom_size);
    app.update();

    let img_q = app.world_mut().query::<(&Node, &IsFamiqImage)>().get_single(app.world());
    let img_node = img_q.unwrap().0;
    assert_eq!(Val::Px(200.0), img_node.width);
    assert_eq!(Val::Px(200.0), img_node.height);
}
