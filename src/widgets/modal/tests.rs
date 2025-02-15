#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::FamiqWidgetResource;
use crate::widgets::text::fa_text;
use super::*;

fn setup_test_default_modal(mut commands: Commands) {
    fa_modal(&mut commands).id("#test-modal").build();
}

fn setup_test_modal_with_children(mut commands: Commands) {
    let txt_one = fa_text(&mut commands, "Text one").build();
    let txt_two = fa_text(&mut commands, "Text two").build();

    fa_modal(&mut commands)
        .children(vec![txt_one, txt_two])
        .build();
}

#[test]
fn test_create_default_modal() {
    let mut app = utils::create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_default_modal);
    app.add_systems(Update, FaModal::_detect_fa_modal_creation_system);
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
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_modal_with_children);
    app.add_systems(Update, FaModal::_detect_fa_modal_creation_system);
    app.update();

    let modal_q = app.world_mut().query::<(&Children, &IsFamiqModalContainer)>().get_single(app.world());
    assert_eq!(2 as usize, modal_q.unwrap().0.len());
}
