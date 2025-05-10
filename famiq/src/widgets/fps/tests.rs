#![cfg(test)]

use crate::FamiqPlugin;
use crate::utils::create_test_app;
use crate::widgets::{FamiqResource, WidgetId, WidgetClasses, builder_mut};
use crate::fps;
use super::*;

fn setup_test_default_fps(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    fps!(id: "#test-fps", class: "test-class");
}

fn setup_test_fps_with_change_color(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    fps!(change_color: true);
}

fn setup_test_fps_with_right_side(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    fps!(right_side: true);
}

#[test]
fn test_create_default_fps() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_default_fps);
    app.update();

    let fps_q = app.world_mut()
        .query::<(&WidgetId, &WidgetClasses, &IsFPSTextLabel)>()
        .single(app.world());

    let fps_id = fps_q.as_ref().unwrap().0;
    assert_eq!("#test-fps".to_string(), fps_id.0);

    let fps_class = fps_q.unwrap().1;
    assert_eq!("test-class".to_string(), fps_class.0);
}

#[test]
fn test_create_fps_with_change_color() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_fps_with_change_color);
    app.update();

    let fps_q = app.world_mut().query::<(&CanChangeColor, &IsFPSTextCount)>().single(app.world());

    let fps_can_change_color_flag = fps_q.unwrap().0;
    assert_eq!(true, fps_can_change_color_flag.0);
}

#[test]
fn test_create_fps_with_right_side() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_fps_with_right_side);
    app.update();

    let fps_q = app.world_mut().query::<(&Node, &IsFPSTextLabel)>().single(app.world());

    let fps_node = fps_q.unwrap().0;

    // when right_side is true, right is Val::Px(6.0) and left is Val::Auto by default
    assert_eq!(Val::Px(6.0), fps_node.right);
    assert_eq!(Val::Auto, fps_node.left);
}
