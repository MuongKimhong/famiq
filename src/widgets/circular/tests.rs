#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::{FamiqWidgetResource, FamiqWidgetId, FamiqWidgetClasses};
use crate::utils::create_test_app;
use super::*;

fn setup_test_default_circular(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqWidgetResource>,
) {
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_circular(&mut builder).id("#test-circular").build();
}

fn setup_test_circular_with_built_in_class(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqWidgetResource>,
) {
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_circular(&mut builder)
        .id("#test-circular")
        .class("is-primary is-large")
        .build();
}

fn set_up_circular_with_custom_size(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqWidgetResource>,
) {
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_circular(&mut builder)
        .size(90.0)
        .build();
}

#[test]
fn test_create_default_circular() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_default_circular);
    app.update();

    let circular_q = app.world_mut().query::<(&FamiqWidgetId, &Node, &IsFamiqCircular)>().get_single(app.world());
    assert!(circular_q.is_ok(), "There should be only 1 circular");

    let circular_id = circular_q.as_ref().unwrap().0;
    assert_eq!(
        "#test-circular".to_string(),
        circular_id.0
    );

    // default width & height is 46px or Val::Px(46.0)
    let circular_node = circular_q.unwrap().1;
    assert_eq!(
        Val::Px(46.0),
        circular_node.width,
        "Default width should be 46px"
    );
    assert_eq!(
        Val::Px(46.0),
        circular_node.height,
        "Default height should be 46px"
    );
}

#[test]
fn test_create_circular_with_built_in_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_circular_with_built_in_class);
    app.update();

    let circular_q = app.world_mut().query::<(&FamiqWidgetClasses, &Node, &IsFamiqCircular)>().get_single(app.world());

    let circular_class = circular_q.as_ref().unwrap().0;
    assert_eq!(
        "is-primary is-large".to_string(),
        circular_class.0
    );

    // default width & height is 46px or Val::Px(46.0)
    let circular_node = circular_q.unwrap().1;
    assert_eq!(
        Val::Px(52.0),
        circular_node.width,
        "Circular with class is-large has width of 52px"
    );
    assert_eq!(
        Val::Px(52.0),
        circular_node.height,
        "Circular with class is-large has height of 52px"
    );
}

#[test]
fn test_create_circular_with_custom_size() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, set_up_circular_with_custom_size);
    app.update();

    let circular_q = app.world_mut().query::<(&Node, &IsFamiqCircular)>().get_single(app.world());

    let circular_node = circular_q.unwrap().0;
    assert_eq!(
        Val::Px(90.0),
        circular_node.width,
        "Custom size width is 90.0"
    );
    assert_eq!(
        Val::Px(90.0),
        circular_node.height,
        "Custom size height is 90.0"
    );
}
