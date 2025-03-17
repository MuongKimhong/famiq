#![cfg(test)]

use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use crate::plugin::FamiqPlugin;
use crate::utils::*;
use super::button::*;
use super::text::*;
use super::*;

#[derive(Resource)]
struct TestResource(Entity);

fn setup_test_fa_query(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery,
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    let btn = fa_button(&mut builder, "Press me").id("#test-btn").build();
    commands.insert_resource(TestResource(btn));
}

fn setup_test_fa_query_for_text(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery,
) {
    let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
    let text = fa_text(&mut builder, "Hello").id("#test-text").build();
    commands.insert_resource(TestResource(text));
}

fn run_set_background_color_id(mut fa_query: FaQuery) {
    fa_query.set_background_color(WidgetSelector::ID("#test-btn"), Color::from(BLUE));
}

fn run_set_background_color_entity(mut fa_query: FaQuery, test_res: Res<TestResource>) {
    fa_query.set_background_color(WidgetSelector::ENTITY(test_res.0), Color::from(GREEN));
}

fn run_set_border_color_id(mut fa_query: FaQuery) {
    fa_query.set_border_color(WidgetSelector::ID("#test-btn"), Color::from(BLUE));
}

fn run_set_border_color_entity(mut fa_query: FaQuery, test_res: Res<TestResource>) {
    fa_query.set_border_color(WidgetSelector::ENTITY(test_res.0), Color::from(GREEN));
}

fn run_set_size_id(mut fa_query: FaQuery) {
    fa_query.set_size(WidgetSelector::ID("#test-btn"), (Val::Px(200.0), Val::Px(20.0)));
}

fn run_set_size_entity(mut fa_query: FaQuery, test_res: Res<TestResource>) {
    fa_query.set_size(WidgetSelector::ENTITY(test_res.0), (Val::Px(200.0), Val::Px(20.0)));
}

fn run_set_text_color_id(mut fa_query: FaQuery) {
    fa_query.set_color(WidgetSelector::ID("#test-text"), Color::from(BLUE));
}

fn run_set_text_color_entity(mut fa_query: FaQuery, test_res: Res<TestResource>) {
    fa_query.set_color(WidgetSelector::ENTITY(test_res.0), Color::from(GREEN));
}

#[test]
fn test_set_text_color_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query_for_text);
    app.update();
    app.add_systems(Update, run_set_text_color_id);
    app.update();
    let text_q = app.world_mut().query::<(&TextColor, &IsFamiqText)>().get_single(app.world());
    assert_eq!(Color::from(BLUE), text_q.unwrap().0.0);
}

#[test]
fn test_set_text_color_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query_for_text);
    app.update();
    app.add_systems(Update, run_set_text_color_entity);
    app.update();
    let text_q = app.world_mut().query::<(&TextColor, &IsFamiqText)>().get_single(app.world());
    assert_eq!(Color::from(GREEN), text_q.unwrap().0.0);
}

#[test]
fn test_set_size_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_set_size_id);
    app.update();
    let btn_q = app.world_mut().query::<(&Node, &IsFamiqButton)>().get_single(app.world());
    let btn = btn_q.unwrap();
    assert_eq!(Val::Px(200.0), btn.0.width);
    assert_eq!(Val::Px(20.0), btn.0.height);
}

#[test]
fn test_set_size_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_set_size_entity);
    app.update();
    let btn_q = app.world_mut().query::<(&Node, &IsFamiqButton)>().get_single(app.world());
    let btn = btn_q.unwrap();
    assert_eq!(Val::Px(200.0), btn.0.width);
    assert_eq!(Val::Px(20.0), btn.0.height);
}

#[test]
fn test_set_background_color_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_set_background_color_id);
    app.update();
    let btn_q = app.world_mut().query::<(&BackgroundColor, &IsFamiqButton)>().get_single(app.world());
    assert_eq!(Color::from(BLUE), btn_q.unwrap().0.0);
}

#[test]
fn test_set_background_color_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_set_background_color_entity);
    app.update();
    let btn_q = app.world_mut().query::<(&BackgroundColor, &IsFamiqButton)>().get_single(app.world());
    assert_eq!(Color::from(GREEN), btn_q.unwrap().0.0);
}

#[test]
fn test_set_border_color_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_set_border_color_id);
    app.update();
    let btn_q = app.world_mut().query::<(&BorderColor, &IsFamiqButton)>().get_single(app.world());
    assert_eq!(Color::from(BLUE), btn_q.unwrap().0.0);
}

#[test]
fn test_set_border_color_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_set_border_color_entity);
    app.update();
    let btn_q = app.world_mut().query::<(&BorderColor, &IsFamiqButton)>().get_single(app.world());
    assert_eq!(Color::from(GREEN), btn_q.unwrap().0.0);
}

#[test]
fn test_widget_style_from_external() {
    let mut local_style = WidgetStyle {
        color: Some("red".to_string()),
        ..default()
    };

    let external_style = WidgetStyle {
        color: Some("blue".to_string()),
        background_color: Some("yellow".to_string()),
        ..default()
    };

    // Update the local style with the external style
    local_style.from_external(&external_style);

    assert_eq!(
        local_style.color,
        Some("blue".to_string())
    );
    assert_eq!(
        local_style.background_color,
        Some("yellow".to_string()),
    );
}

#[test]
fn test_widget_style_update_from() {
    let mut local_style = WidgetStyle {
        color: Some("red".to_string()),
        font_size: None,
        background_color: Some("white".to_string()),
        ..default()
    };

    let external_style = WidgetStyle {
        color: Some("blue".to_string()),
        font_size: Some("16px".to_string()),
        background_color: None,
        ..default()
    };

    // Update the local style with the external style
    local_style.update_from(&external_style);

    assert_eq!(
        local_style.color,
        Some("blue".to_string())
    );
    assert_eq!(
        local_style.font_size,
        Some("16px".to_string())
    );
    assert_eq!(
        local_style.background_color,
        None,
    );
}
