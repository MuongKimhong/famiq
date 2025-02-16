#![cfg(test)]

use crate::utils::create_test_app;
use crate::plugin::FamiqPlugin;
use crate::widgets::{FamiqWidgetId, FamiqWidgetClasses, FamiqResource};
use super::*;

#[derive(Resource)]
struct TestEntityForUpdateByEntity(Entity);

fn setup_test_default_bar(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_progress_bar(&mut builder).id("#test-bar").build();
}

fn setup_test_bar_with_built_in_class(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_progress_bar(&mut builder)
        .class("is-primary is-large")
        .build();
}

fn setup_test_bar_with_percentage(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_progress_bar(&mut builder)
        .id("#test-bar")
        .percentage(50.0)
        .build();
}

fn setup_test_update_by_entity(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut builder_res: ResMut<FamiqResource>,
) {
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
    let bar = fa_progress_bar(&mut builder)
        .percentage(40.0)
        .build();

    commands.insert_resource(TestEntityForUpdateByEntity(bar));
}

#[test]
fn test_create_default_bar() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
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
    app.add_systems(Startup, setup_test_default_bar);
    app.update();

    let bar_res = app.world_mut().resource::<FaProgressBarResource>();
    let percentage = bar_res.get_percentage_by_id("#random-id");
    assert_eq!(None, percentage);
}



#[test]
fn test_get_percentage_by_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_update_by_entity);
    app.update();

    let bar_entity = app.world_mut().resource::<TestEntityForUpdateByEntity>().0;
    let bar_res = app.world_mut().resource::<FaProgressBarResource>();
    let percentage = bar_res.get_percentage_by_entity(bar_entity);
    assert_eq!(Some(40.0), percentage);
}

#[test]
fn test_update_percentage_by_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_update_by_entity);
    app.add_systems(Update, FaProgressBar::handle_progress_value_change_by_entity);
    app.update();

    let bar_entity = app.world_mut().resource::<TestEntityForUpdateByEntity>().0;
    let mut bar_res = app.world_mut().resource_mut::<FaProgressBarResource>();
    let old_percentage = bar_res.get_percentage_by_entity(bar_entity);
    assert_eq!(Some(40.0), old_percentage);

    bar_res.set_percentage_by_entity(bar_entity, Some(10.0));
    app.update(); // update again so handle_progress_value_change_by_entity run again;

    let value_q = app.world_mut().query::<&FaProgressValuePercentage>()
        .get_single(app.world());

    let new_percentage = value_q.unwrap().0;
    assert_eq!(10.0, new_percentage);
}
