#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::utils::create_test_app;
use crate::widgets::{FamiqResource, FamiqWidgetId, FamiqWidgetClasses};
use super::*;

fn setup_test_default_fps(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_fps(&mut builder)
        .id("#test-fps")
        .class("test-class")
        .build();
}

fn setup_test_fps_with_change_color(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_fps(&mut builder)
        .change_color()
        .build();
}

fn setup_test_fps_with_right_side(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_fps(&mut builder)
        .right_side()
        .build();
}

#[test]
fn test_create_default_fps() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_fps);
    app.update();

    let fps_q = app.world_mut()
        .query::<(&FamiqWidgetId, &FamiqWidgetClasses, &IsFamiqFPSTextLabel)>()
        .get_single(app.world());

    let fps_id = fps_q.as_ref().unwrap().0;
    assert_eq!("#test-fps".to_string(), fps_id.0);

    let fps_class = fps_q.unwrap().1;
    assert_eq!("test-class".to_string(), fps_class.0);
}

#[test]
fn test_create_fps_with_change_color() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fps_with_change_color);
    app.update();

    let fps_q = app.world_mut().query::<(&CanChangeColor, &IsFamiqFPSTextCount)>().get_single(app.world());

    let fps_can_change_color_flag = fps_q.unwrap().0;
    assert_eq!(true, fps_can_change_color_flag.0);
}

#[test]
fn test_create_fps_with_right_side() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fps_with_right_side);
    app.update();

    let fps_q = app.world_mut().query::<(&Node, &IsFamiqFPSTextLabel)>().get_single(app.world());

    let fps_node = fps_q.unwrap().0;

    // when right_side is true, right is Val::Px(6.0) and left is Val::Auto by default
    assert_eq!(Val::Px(6.0), fps_node.right);
    assert_eq!(Val::Auto, fps_node.left);
}
