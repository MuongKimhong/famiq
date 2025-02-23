use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin) // add plugin
        .add_systems(Startup, setup)
        .add_systems(Update, handle_btn_press)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>, // required
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server);

    let text = fa_text(&mut builder, "Press button to see a message!!")
        .class("h2")
        .build();

    let name = fa_text_input(&mut builder, "Enter your name")
        .id("#name")
        .class("is-secondary mt-10")
        .build();

    let done_btn = fa_button(&mut builder, "Done").id("#btn").class("is-secondary").build();

    fa_container(&mut builder)
        .id("#container")
        .children([text, name, done_btn])
        .build();


    let close_btn = fa_button(&mut builder, "Close").id("#close-btn").class("is-small").build();
    // update text value when button is pressed
    let welcome_txt = fa_text(&mut builder, "").id("#welcome-txt").build();
    let modal_container = fa_container(&mut builder)
        .id("#modal-container")
        .children([welcome_txt, close_btn])
        .build();

    fa_modal(&mut builder).id("#modal").children([modal_container]).build();
}

fn handle_btn_press(
    mut events: EventReader<FaInteractionEvent>,
    mut modal_state: ResMut<FaModalState>,
    mut text_res: ResMut<FaTextResource>,
    input_res: Res<FaTextInputResource>,
) {
    for e in events.read() {
        if e.is_pressed(WidgetType::Button) {
            if let Some(id) = e.widget_id.as_ref() {
                match id.as_str() {
                    "#btn" => {
                        let name = input_res.get_value_by_id("#name");
                        let new_txt = format!("Welcome {name}, this example is built with Famiq.");

                        text_res.update_value_by_id("#welcome-txt", new_txt.as_str());
                        modal_state.show_by_id("#modal");
                    },
                    "#close-btn" => {
                        modal_state.hide_by_id("#modal");
                    }
                    _ => ()
                }
            }
        }
    }
}
