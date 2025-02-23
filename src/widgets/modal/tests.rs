#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::widgets::FamiqResource;
use crate::widgets::text::fa_text;
use super::*;

fn setup_test_default_modal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_modal(&mut builder).id("#test-modal").build();
}

fn setup_test_modal_with_children(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    let txt_one = fa_text(&mut builder, "Text one").build();
    let txt_two = fa_text(&mut builder, "Text two").build();

    fa_modal(&mut builder)
        .children(vec![txt_one, txt_two])
        .build();
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
