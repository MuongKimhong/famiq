#![cfg(test)]

use crate::plugin::FamiqPlugin;
use crate::button;
use crate::widgets::{WidgetId, WidgetClasses, FaQuery};
use super::*;

fn setup_test_default_button(
    mut famiq_res: ResMut<FamiqResource>,
    mut famiq_query: FaQuery
) {
    FamiqBuilder::new(&mut famiq_query, &mut famiq_res).inject();
    button!(text: "Press me", id: "#test-btn");
}

fn setup_test_button_with_built_in_class(
    mut famiq_res: ResMut<FamiqResource>,
    mut famiq_query: FaQuery
) {
    FamiqBuilder::new(&mut famiq_query, &mut famiq_res).inject();
    button!(
        text: "Press me",
        id: "#test-btn",
        class: "is-primary is-large is-round"
    );
}

#[test]
fn test_create_default_button() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_default_button);
    app.update();

    let btn_q = app.world_mut().query::<(&WidgetId, &IsFamiqButton)>().single(app.world());
    assert!(btn_q.is_ok(), "There should be only 1 button");

    let btn_id = btn_q.unwrap().0;
    assert_eq!(
        "#test-btn".to_string(),
        btn_id.0
    );

    let btn_text_q = app.world_mut().query::<(&Text, &IsFamiqButtonText)>()
                    .single(app.world());

    assert_eq!(
        "Press me".to_string(),
        btn_text_q.unwrap().0.0
    );
}

#[test]
fn test_create_button_with_built_in_class() {
    let mut app = create_test_app();
    app.add_plugins(FamiqPlugin);
    app.add_systems(Startup, setup_test_button_with_built_in_class);
    app.update();

    let btn_q = app.world_mut().query::<(&WidgetClasses, &IsFamiqButton)>().single(app.world());
    assert_eq!(
        "is-primary is-large is-round".to_string(),
        btn_q.unwrap().0.0
    );
}
