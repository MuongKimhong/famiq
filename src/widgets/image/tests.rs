#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::{FamiqResource, FamiqWidgetId, FamiqWidgetClasses};
use crate::utils::{get_embedded_asset_path, create_test_app};
use super::*;

fn setup_test_default_image(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
    fa_image(&mut builder, path.as_str()).id("#test-image").build();
}

fn setup_test_image_with_class(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
    fa_image(&mut builder, path.as_str())
        .class("test-class-one")
        .build();
}

fn setup_test_image_with_custom_size(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
    fa_image(&mut builder, path.as_str())
        .size(Val::Px(200.0), Val::Px(200.0))
        .build();
}

#[test]
fn test_create_default_image() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_image);
    app.update();

    let img_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqImage)>().get_single(app.world());
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

    let img_q = app.world_mut().query::<(&FamiqWidgetClasses, &IsFamiqImage)>().get_single(app.world());
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
