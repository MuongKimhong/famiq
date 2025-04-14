#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::fa_button;
use crate::widgets::{FamiqResource, inject_builder, builder_mut};
use bevy::input::InputPlugin;
use crate::fa_listview;
use super::*;


fn setup_test_default_listview(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);
    fa_listview!(id: "#test-listview");
}

fn setup_test_listview_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);
    let button_one = fa_button!(text: "Button 1");
    let button_two = fa_button!(text: "Button 2");

    fa_listview!(children: [button_one, button_two]);
}

#[test]
fn test_create_default_listview() {
    let mut app = utils::create_test_app();
    app.add_plugins(InputPlugin::default());
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_listview);

    app.update();

    let listview_q = app.world_mut()
        .query::<(&WidgetId, &Children, &IsFamiqListView)>()
        .get_single(app.world());

    assert!(listview_q.is_ok(), "There should be only 1 listview");

    let listview_id = listview_q.as_ref().unwrap().0;
    assert_eq!("#test-listview".to_string(), listview_id.0, "Should match");

    // The listview itself has only 1 child which is move_panel.
    // All the children provided by users belong to move_panel
    assert_eq!(1 as usize, listview_q.unwrap().1.len(), "Should be 1");
}

#[test]
fn test_create_listview_with_children() {
    let mut app = utils::create_test_app();
    app.add_plugins(InputPlugin::default());
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_listview_with_children);
    app.update();

    let move_panel_q = app.world_mut()
        .query::<(&Children, &IsFamiqListViewMovePanel)>()
        .get_single(app.world());

    assert_eq!(2 as usize, move_panel_q.unwrap().0.len());
}
