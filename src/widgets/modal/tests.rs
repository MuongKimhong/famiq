#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::FamiqResource;
use crate::fa_text;
use crate::fa_modal;
use super::*;

fn setup_test_default_modal(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);
    fa_modal!(id: "#test-modal");
}

fn setup_test_modal_with_children(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    inject_builder(&mut builder);

    fa_modal!(children: [
        fa_text!(text: "Text one"),
        fa_text!(text: "Text two")
    ]);
}

#[test]
fn test_create_default_modal() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_modal);
    app.update();

    let modal_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqModalBackground)>().get_single(app.world());
    assert!(modal_q.is_ok(), "There should be only 1 listview");

    let modal_id = modal_q.unwrap().0;
    assert_eq!("#test-modal".to_string(), modal_id.0);
}

#[test]
fn test_create_modal_with_children() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_modal_with_children);
    app.update();

    let modal_q = app.world_mut().query::<(&Children, &IsFamiqModalContainer)>().get_single(app.world());
    assert_eq!(2 as usize, modal_q.unwrap().0.len());
}
