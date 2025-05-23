#![cfg(test)]

use crate::FamiqPlugin;
use crate::widgets::*;
use crate::circular;
use crate::utils::create_test_app;
use super::*;

fn setup_test_default_circular(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    circular!(id: "#test-circular");
}

fn setup_test_circular_with_built_in_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    circular!(id: "#test-circular", class: "primary large");
}

fn set_up_circular_with_custom_size(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    circular!(size: 90.0);
}

#[test]
fn test_create_default_circular() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_default_circular);
    app.update();

    let circular_q = app.world_mut().query::<(&WidgetId, &Node, &IsFamiqCircular)>().single(app.world());
    assert!(circular_q.is_ok(), "There should be only 1 circular");

    let circular_id = circular_q.as_ref().unwrap().0;
    assert_eq!(
        "#test-circular".to_string(),
        circular_id.0
    );

    // default width & height is 46px or Val::Px(46.0)
    let circular_node = circular_q.unwrap().1;
    assert_eq!(
        Val::Px(50.0),
        circular_node.width,
        "Default width should be 50px"
    );
    assert_eq!(
        Val::Px(50.0),
        circular_node.height,
        "Default height should be 50px"
    );
}

#[test]
fn test_create_circular_with_built_in_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_circular_with_built_in_class);
    app.update();

    let circular_q = app.world_mut().query::<(&WidgetClasses, &Node, &IsFamiqCircular)>().single(app.world());

    let circular_class = circular_q.as_ref().unwrap().0;
    assert_eq!(
        "primary large".to_string(),
        circular_class.0
    );

    // default width & height is 46px or Val::Px(46.0)
    let circular_node = circular_q.unwrap().1;
    assert_eq!(
        Val::Px(65.0),
        circular_node.width,
        "Circular with class large has width of 65px"
    );
    assert_eq!(
        Val::Px(65.0),
        circular_node.height,
        "Circular with class    large has height of 65px"
    );
}

#[test]
fn test_create_circular_with_custom_size() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, set_up_circular_with_custom_size);
    app.update();

    let circular_q = app.world_mut().query::<(&Node, &IsFamiqCircular)>().single(app.world());

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
