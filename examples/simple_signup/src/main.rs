use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin) // add plugin
        .add_systems(Startup, setup)
        .add_systems(Update, press)
        .run();
}

fn setup(
    mut commands: Commands,
    mut builder_res: ResMut<FamiqResource>, // required
    asset_server: ResMut<AssetServer>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server)
        .register_tooltip()
        .hot_reload();

    fa_fps(&mut builder).class("fps").right_side().change_color().build();

    let txt = fa_text(&mut builder, "SIGN UP").build();

    let f_name = fa_text_input(&mut builder, "First name").id("#first-name").build();
    let l_name = fa_text_input(&mut builder, "Last name").id("#last-name").build();
    let name_container = fa_container(&mut builder)
        .id("#name-container")
        .children([f_name, l_name])
        .build();

    let ask = fa_selection(&mut builder, "Select your subscription")
        .choices(["Personal", "Team"])
        .build();
    let ask_container = fa_container(&mut builder).id("#ask-container")
        .children([ask])
        .build();


    let btn_1 = fa_button(&mut builder, "Test").id("#1").class("is-danger").tooltip("hello").build();
    let btn = fa_button(&mut builder, "Confirm").id("#2").class("is-danger").build();

    let progress = fa_progress_bar(&mut builder)
        .id("#bar")
        .set_color("blue")
        .class("is-danger").build();

    let cir = fa_circular(&mut builder)
        .size(90.0)
        .class("is-secondary")
        .tooltip("aaaa")
        .build();

    fa_container(&mut builder).id("#container")
        .children([txt, name_container, ask_container, btn_1, btn, progress, cir])
        .build();
}

fn press(
    mut events: EventReader<FaInteractionEvent>,
    mut pr: ResMut<FaProgressBarResource>,
) {
    for e in events.read() {
        if e.is_button_pressed() {
            if let Some(id) = e.widget_id.as_ref() {
                match id.as_str() {
                    "#1" => pr.set_percentage_by_id("#bar", None),
                    "#2" => pr.set_percentage_by_id("#bar", Some(40.0)),
                    _ => {}
                }
            }
        }
    }
}
