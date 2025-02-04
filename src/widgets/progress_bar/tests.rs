#![cfg(test)]

use crate::utils::create_test_app;
use crate::plugin::FamiqPlugin;
use crate::widgets::{FamiqWidgetId, FamiqWidgetClasses, FamiqWidgetResource};
use super::*;

fn setup_test_default_bar(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqWidgetResource>,
) {
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_progress_bar(&mut builder).id("#test-bar").build();
}

fn setup_test_bar_with_built_in_class(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqWidgetResource>,
) {
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_progress_bar(&mut builder)
        .class("is-primary is-large")
        .build();
}

fn setup_test_bar_with_percentage(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqWidgetResource>,
) {
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_progress_bar(&mut builder)
        .id("#test-bar")
        .percentage(50.0)
        .build();
}

#[test]
fn test_create_default_bar() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_default_bar);
    app.update();

    let bar_q = app.world_mut()
        .query::<(&FamiqWidgetId, &IsFamiqProgressBar)>()
        .get_single(app.world());

    let bar_id = bar_q.unwrap().0;
    assert_eq!("#test-bar".to_string(), bar_id.0);
}

#[test]
fn test_create_bar_with_built_in_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_bar_with_built_in_class);
    app.update();

    let bar_q = app.world_mut()
        .query::<(&FamiqWidgetClasses, &IsFamiqProgressBar)>()
        .get_single(app.world());

    assert_eq!("is-primary is-large".to_string(), bar_q.unwrap().0.0);
}

#[test]
fn test_create_bar_with_percentage() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_bar_with_percentage);
    app.update();

    let bar_q = app.world_mut()
        .query::<(&FaProgressValuePercentage, &IsFamiqProgressValue)>()
        .get_single(app.world());

    assert_eq!(50.0, bar_q.unwrap().0.0);
}

#[test]
fn test_get_percentage_by_non_exist_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_default_bar);
    app.update();

    let bar_res = app.world_mut().resource::<FaProgressBarResource>();
    let percentage = bar_res.get_percentage_by_id("#random-id");
    assert_eq!(None, percentage);
}

#[test]
fn test_get_percentage_by_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.insert_resource(FamiqWidgetResource::default());
    app.add_systems(Startup, setup_test_bar_with_percentage);
    app.update();

    let bar_res = app.world_mut().resource::<FaProgressBarResource>();
    let percentage = bar_res.get_percentage_by_id("#test-bar");
    assert_eq!(Some(50.0), percentage);
}
