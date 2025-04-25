use bevy::prelude::*;
use famiq::prelude::*;

#[derive(Resource)]
struct TestEntity(Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin) // add plugin
        .add_systems(Startup, setup)
        .add_systems(Update, handle_btn_press)
        .run();
}

fn setup(mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    fa_query.insert_str("plan", "");
    fa_query.insert_str("name", "");
    fa_query.insert_str("test_class", "test-one");

    FamiqBuilder::new(&mut fa_query, &mut famiq_res).hot_reload().inject();

    scroll!(
        id: "#scroll",
        children: [
            text!(text: "Test text $[plan]", class: "$[test_class]"),
            text!(text: "Test name $[name]", class: "$[test_class]"),
            text_input!(placeholder: "Enter", model: "plan", id: "#input"),
            text_input!(placeholder: "Enter name", model: "name"),
            button!(text: "Hello", id: "#press")
        ]
    );

}

fn handle_btn_press(
    mut fa_query: FaQuery,
    mut events: EventReader<FaMouseEvent>,
    mut famiq_res: ResMut<FamiqResource>,
) {
    for e in events.read() {
        if let Some(id) = e.button_press() {
            match id.as_str() {
                "#press" => {
                    println!("pressed");
                }
                _ => {}
            }
        }
    }
}
