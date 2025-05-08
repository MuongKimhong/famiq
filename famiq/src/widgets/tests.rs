#![cfg(test)]

use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use crate::widgets::container::*;
use crate::plugin::FamiqPlugin;
use crate::button;
use crate::modal;
use crate::container;
use bevy::input::InputPlugin;
use crate::utils::*;
use crate::widgets::scroll::*;
use super::button::*;
use super::text::*;
use super::modal::*;
use crate::scroll;
use crate::text;
use super::*;

#[derive(Resource)]
struct TestResource(Entity);

fn setup_test_fa_query(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery,
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let btn = button!(text: "Press me", id: "#test-btn");
    commands.insert_resource(TestResource(btn));
}

fn setup_test_fa_query_for_text(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery,
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text = text!(text: "Hello", id: "#test-text");
    commands.insert_resource(TestResource(text));
}

fn setup_test_containable_for_container(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let container = container!(id: "#test-container");
    fa_query.commands.insert_resource(TestResource(container));
}

fn run_add_children_for_container(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    test_res: Res<TestResource>
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text_one = text!(text: "Hello");
    let text_two = text!(text: "Hello");
    fa_query.add_children(WidgetSelector::ID("#test-container"), &[text_one]);
    fa_query.add_children(WidgetSelector::ENTITY(test_res.0), &[text_two]);
}

fn run_insert_children_for_container(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    test_res: Res<TestResource>
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text_one = text!(text: "Hello");
    let text_two = text!(text: "Hello");
    fa_query.insert_children(WidgetSelector::ID("#test-container"), 0, &[text_one]);
    fa_query.insert_children(WidgetSelector::ENTITY(test_res.0), 0, &[text_two]);
}

fn setup_test_containable_for_modal(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let modal = modal!(id: "#test-modal");
    fa_query.commands.insert_resource(TestResource(modal));
}

fn run_add_children_for_modal(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    test_res: Res<TestResource>
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text_one = text!(text: "Hello");
    let text_two = text!(text: "Hello");
    fa_query.add_children(WidgetSelector::ID("#test-modal"), &[text_one]);
    fa_query.add_children(WidgetSelector::ENTITY(test_res.0), &[text_two]);
}

fn run_insert_children_for_modal(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    test_res: Res<TestResource>
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text_one = text!(text: "Hello");
    let text_two = text!(text: "Hello");
    fa_query.insert_children(WidgetSelector::ID("#test-modal"), 0, &[text_one]);
    fa_query.insert_children(WidgetSelector::ENTITY(test_res.0), 0, &[text_two]);
}

fn setup_test_containable_for_listview(
    mut famiq_res: ResMut<FamiqResource>,
    mut fa_query: FaQuery
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let listview = scroll!(id: "#test-listview");
    fa_query.commands.insert_resource(TestResource(listview));
}

fn run_add_children_for_listview(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    test_res: Res<TestResource>
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text_one = text!(text: "Hello");
    let text_two = text!(text: "Hello");
    fa_query.add_children(WidgetSelector::ID("#test-listview"), &[text_one]);
    fa_query.add_children(WidgetSelector::ENTITY(test_res.0), &[text_two]);
}

fn run_insert_children_for_listview(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    test_res: Res<TestResource>
) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    let text_one = text!(text: "Hello");
    let text_two = text!(text: "Hello");
    fa_query.insert_children(WidgetSelector::ID("#test-listview"), 0, &[text_one]);
    fa_query.insert_children(WidgetSelector::ENTITY(test_res.0), 0, &[text_two]);
}

fn run_update_by_id_for_get_style_mut(mut fa_style_q: FaStyleQuery) {
    if let Some(mut style) = fa_style_q.get_style_mut(WidgetSelector::ID("#test-btn")) {
        style.background_color.0 = Color::from(BLUE);
    }
}

fn run_update_by_entity_for_get_style_mut(mut fa_style_q: FaStyleQuery, test_res: Res<TestResource>) {
    if let Some(mut style) = fa_style_q.get_style_mut(WidgetSelector::ENTITY(test_res.0)) {
        style.background_color.0 = Color::from(GREEN);
    }
}

fn run_set_text_color_entity(mut fa_style_q: FaStyleQuery, test_res: Res<TestResource>) {
    if let Some(mut style) = fa_style_q.get_text_style_mut(WidgetSelector::ENTITY(test_res.0)) {
        style.text_color.0 = Color::from(GREEN);
    }
}

fn setup_test_built_in_alignment_class(mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
    container!(id: "#container-one", class: "jc-center ac-center");
    container!(id: "#container-two", class: "ji-center ai-center");
    container!(id: "#container-three", class: "js-center as-center");
}

#[test]
fn test_built_in_alignment_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_built_in_alignment_class);
    app.update();

    let mut query = app.world_mut().query::<(&WidgetId, &Node)>();

    for (id, node) in query.iter(app.world_mut()) {
        if id.0 == "#container-one" {
            assert_eq!(node.justify_content, JustifyContent::Center);
            assert_eq!(node.align_content, AlignContent::Center);
        }
        else if id.0 == "#container-two" {
            assert_eq!(node.justify_items, JustifyItems::Center);
            assert_eq!(node.align_items, AlignItems::Center);
        }
        else if id.0 == "#container-three" {
            assert_eq!(node.justify_self, JustifySelf::Center);
            assert_eq!(node.align_self, AlignSelf::Center);
        }
    }
}

#[test]
fn test_add_children_for_container() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_containable_for_container);
    app.update();
    app.add_systems(Update, run_add_children_for_container);
    app.update();

    let query = app.world_mut().query::<(&Children, &IsFamiqContainer)>().single(app.world());
    assert_eq!(query.unwrap().0.iter().count(), 2);
}

#[test]
fn test_insert_children_for_container() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_containable_for_container);
    app.update();
    app.add_systems(Update, run_insert_children_for_container);
    app.update();

    let query = app.world_mut().query::<(&Children, &IsFamiqContainer)>().single(app.world());
    assert_eq!(query.unwrap().0.iter().count(), 2);
}

#[test]
fn test_add_children_for_modal() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_containable_for_modal);
    app.update();
    app.add_systems(Update, run_add_children_for_modal);
    app.update();

    let query = app.world_mut().query::<(&Children, &IsFamiqModal)>().single(app.world());
    assert_eq!(query.unwrap().0.iter().count(), 2);
}

#[test]
fn test_insert_children_for_modal() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_containable_for_modal);
    app.update();
    app.add_systems(Update, run_insert_children_for_modal);
    app.update();

    let query = app.world_mut().query::<(&Children, &IsFamiqModal)>().single(app.world());
    assert_eq!(query.unwrap().0.iter().count(), 2);
}

#[test]
fn test_add_children_for_listview() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_plugins(InputPlugin::default());
    app.add_systems(Startup, setup_test_containable_for_listview);
    app.update();
    app.add_systems(Update, run_add_children_for_listview);
    app.update();

    let query = app.world_mut().query::<(&Children, &IsFamiqScrollMovePanel)>().single(app.world());
    assert_eq!(query.unwrap().0.iter().count(), 2);
}

#[test]
fn test_insert_children_for_listview() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_plugins(InputPlugin::default());
    app.add_systems(Startup, setup_test_containable_for_listview);
    app.update();
    app.add_systems(Update, run_insert_children_for_listview);
    app.update();

    let query = app.world_mut().query::<(&Children, &IsFamiqScrollMovePanel)>().single(app.world());
    assert_eq!(query.unwrap().0.iter().count(), 2);
}

#[test]
fn test_set_text_color_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query_for_text);
    app.update();
    app.add_systems(Update, run_set_text_color_entity);
    app.update();
    let text_q = app.world_mut().query::<(&TextColor, &IsFamiqText)>().single(app.world());
    assert_eq!(Color::from(GREEN), text_q.unwrap().0.0);
}

#[test]
fn test_set_background_color_id() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_update_by_id_for_get_style_mut);
    app.update();
    let btn_q = app.world_mut().query::<(&BackgroundColor, &IsFamiqButton)>().single(app.world());
    assert_eq!(Color::from(BLUE), btn_q.unwrap().0.0);
}

#[test]
fn test_set_background_color_entity() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_fa_query);
    app.update();
    app.add_systems(Update, run_update_by_entity_for_get_style_mut);
    app.update();
    let btn_q = app.world_mut().query::<(&BackgroundColor, &IsFamiqButton)>().single(app.world());
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
