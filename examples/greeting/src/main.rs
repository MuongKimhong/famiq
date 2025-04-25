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
    fa_query.insert_str("test_class", "primary");
    fa_query.insert_str("color", "red");

    FamiqBuilder::new(&mut fa_query, &mut famiq_res).hot_reload().inject();

    container!(
        id: "#scroll",
        color: "$[color]",
        children: [
            text!(text: "Test text $[plan]", class: "$[test_class]"),
            text!(text: "Test name $[name]", class: "$[test_class]"),
            text!(text: "Enter", id: "#input"),
            text!(text: "Enter name", class: "primary"),
            button!(text: "Hello", id: "#press", class: "$[test_class] large"),
            button!(text: "one o", id: "#hello", class: "$[test_class] large")
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
                    fa_query.mutate_data("test_class", RVal::Str("danger".into()));
                    fa_query.mutate_data("color", RVal::Str("blue".into()));
                }
                "#hello" => {
                    fa_query.mutate_data("test_class", RVal::Str("info".into()));
                    fa_query.mutate_data("color", RVal::Str("pink".into()));
                }
                _ => {}
            }
        }
    }
}
