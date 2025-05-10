#![cfg(test)]

use crate::button;
use crate::FamiqPlugin;
use crate::widgets::button::*;
use crate::widgets::FamiqResource;
use bevy::input::InputPlugin;
use crate::scroll;
use super::*;


fn setup_test_default_scroll(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    scroll!(id: "#test-scroll");
}

fn setup_test_scroll_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let button_one = button!(text: "Button 1");
    let button_two = button!(text: "Button 2");

    scroll!(children: [button_one, button_two]);
}

#[test]
fn test_create_default_scroll() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_plugins(InputPlugin::default());
    app.add_systems(Startup, setup_test_default_scroll);

    app.update();

    let scroll_q = app.world_mut()
        .query::<(&WidgetId, &Children, &IsFamiqScroll)>()
        .single(app.world());

    assert!(scroll_q.is_ok(), "There should be only 1 scroll");

    let scroll_id = scroll_q.as_ref().unwrap().0;
    assert_eq!("#test-scroll".to_string(), scroll_id.0, "Should match");

    // The listview itself has only 1 child which is move_panel.
    // All the children provided by users belong to move_panel
    assert_eq!(1 as usize, scroll_q.unwrap().1.len(), "Should be 1");
}

#[test]
fn test_create_scroll_with_children() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_plugins(InputPlugin::default());
    app.add_systems(Startup, setup_test_scroll_with_children);
    app.update();

    let move_panel_q = app.world_mut()
        .query::<(&Children, &IsFamiqScrollMovePanel)>()
        .single(app.world());

    assert_eq!(2 as usize, move_panel_q.unwrap().0.len());
}
