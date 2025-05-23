#![cfg(test)]

use crate::FamiqPlugin;
use crate::utils::create_test_app;
use crate::widgets::{WidgetId, WidgetClasses, FamiqResource};
use crate::progress_bar;
use super::*;

fn setup_test_default_bar(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    progress_bar!(id: "#test-bar");
}

fn setup_test_bar_with_built_in_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    progress_bar!(class: "is-primary is-large");
}

#[test]
fn test_create_default_bar() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_default_bar);
    app.update();

    let bar_q = app.world_mut()
        .query::<(&WidgetId, &IsFamiqProgressBar)>()
        .single(app.world());

    let bar_id = bar_q.unwrap().0;
    assert_eq!("#test-bar".to_string(), bar_id.0);
}

#[test]
fn test_create_bar_with_built_in_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin::new());
    app.add_systems(Startup, setup_test_bar_with_built_in_class);
    app.update();

    let bar_q = app.world_mut()
        .query::<(&WidgetClasses, &IsFamiqProgressBar)>()
        .single(app.world());

    assert_eq!("is-primary is-large".to_string(), bar_q.unwrap().0.0);
}
