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
    asset_server: ResMut<AssetServer>, // required
    mut commands: Commands,
    mut builder_res: ResMut<FamiqWidgetResource>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);

    let name = fa_text_input(&mut builder, "Enter your name")
        .id("#name")
        .class("is-secondary")
        .build();

    let done_btn = fa_button(&mut builder, "Done").id("#btn").class("is-secondary").build();

    fa_container(&mut builder)
        .id("#container")
        .children([name, done_btn])
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
    modal_q: Single<&mut FaModalState>,
    mut text_q: Query<(&FamiqWidgetId, &mut Text), Without<FaModalState>>,
    input_resource: Res<FaTextInputResource>,
) {
    for e in events.read() {
        if e.widget == WidgetType::Button && e.interaction == Interaction::Pressed {
            if let Some(id) = e.widget_id.as_ref() {
                match id.as_str() {
                    "#btn" => {
                        if let Some(name) = input_resource.inputs.get("#name") {
                            if !name.trim().is_empty() {
                                for (text_id, mut text) in text_q.iter_mut() {
                                    if text_id.0 == "#welcome-txt" {
                                        text.0 = format!("Welcome {name}, this example is built with Famiq.");
                                    }
                                }
                                // Open modal
                                let mut state = modal_q.into_inner();
                                state.0 = true;
                                break;
                            }
                        }
                    },
                    "#close-btn" => {
                        // Close modal
                        let mut state = modal_q.into_inner();
                        state.0 = false;
                        break;
                    }
                    _ => ()
                }
            }
        }
    }
}
