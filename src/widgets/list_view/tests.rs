#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::button::fa_button;
use crate::widgets::FamiqResource;
use bevy::input::InputPlugin;
use super::*;

fn setup_test_default_listview(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    fa_listview(&mut builder).id("#test-listview").build();
}

fn setup_test_listview_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    let button_one = fa_button(&mut builder, "Button 1").build();
    let button_two = fa_button(&mut builder, "Button 2").build();

    fa_listview(&mut builder)
        .children(vec![button_one, button_two])
        .build();
}

#[test]
fn test_create_default_listview() {
    let mut app = utils::create_test_app();
    app.add_plugins(InputPlugin::default());
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_listview);

    app.update();

    let listview_q = app.world_mut()
        .query::<(&FamiqWidgetId, &Children, &IsFamiqListView)>()
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
