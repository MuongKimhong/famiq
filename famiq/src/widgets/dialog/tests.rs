#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::FamiqResource;
use crate::widgets::text::*;
use crate::dialog;
use crate::text;
use super::*;

fn setup_test_default_dialog(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    dialog!(id: "#test-modal");
}

fn setup_test_dialog_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    dialog!(children: [
        text!(text: "Text one"),
        text!(text: "Text two")
    ]);
}

#[test]
fn test_create_default_dialog() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_dialog);
    app.update();

    let modal_q = app.world_mut().query::<(&WidgetId, &IsFamiqDialog)>().single(app.world());
    assert!(modal_q.is_ok(), "There should be only 1 listview");

    let modal_id = modal_q.unwrap().0;
    assert_eq!("#test-modal".to_string(), modal_id.0);
}

#[test]
fn test_create_dialog_with_children() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_dialog_with_children);
    app.update();

    let modal_q = app.world_mut().query::<(&Children, &IsFamiqDialog)>().single(app.world());
    assert_eq!(2 as usize, modal_q.unwrap().0.len());
}
