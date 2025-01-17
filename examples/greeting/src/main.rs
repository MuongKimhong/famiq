use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(famiq_plugin) // add plugin
        .add_systems(Startup, setup)
        .add_systems(Update, handle_btn_press)
        .run();
}

fn setup(
    asset_server: ResMut<AssetServer>, // required
    mut commands: Commands,
    mut builder_resource: ResMut<FamiqWidgetBuilderResource>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(
        &mut commands,
        &asset_server,
        &mut builder_resource,
        "assets/fonts/fira-mono-regular.ttf",
        "assets/styles.json",
        true,
    );

    let name = builder.fa_text_input("#name", "is-secondary", "Enter your name");
    let done_btn = builder.fa_button("#btn", "is-secondary", "Done");
    builder.fa_container("#container", "", &vec![
        name,
        done_btn
    ]);

    let close_btn = builder.fa_button("#close-btn", "is-small", "Close");
    let welcome_txt = builder.fa_text("#welcome-txt", "", ""); // update text value when button is pressed

    let modal_container = builder.fa_container("#modal-container", "", &vec![
        welcome_txt,
        close_btn
    ]);
    builder.fa_modal("#modal", "", &vec![modal_container]);
}

fn handle_btn_press(
    mut events: EventReader<FaInteractionEvent>,
    mut modal_q: Query<(&FamiqWidgetId, &mut FaModalState)>,
    mut text_q: Query<(&FamiqWidgetId, &mut Text), Without<FaModalState>>,
    input_resource: Res<FaTextInputResource>,
) {
    for e in events.read() {
        if e.widget == WidgetType::Button && e.interaction == Interaction::Pressed {
            match e.widget_id.as_str() {
                "#btn" => {
                    if let Some(name) = input_resource.inputs.get("#name") {
                        for (text_id, mut text) in text_q.iter_mut() {
                            if text_id.0 == "#welcome-txt" {
                                text.0 = format!("Welcome {name}, this example is built with Famiq.");
                            }
                        }

                        // Open modal
                        if let Some((_, mut state)) = modal_q.iter_mut().find(|(id, _)| id.0 == "#modal") {
                            state.0 = true;
                        }
                    }
                },
                "#close-btn" => {
                    // Close modal
                    if let Some((_, mut state)) = modal_q.iter_mut().find(|(id, _)| id.0 == "#modal") {
                        state.0 = false;
                    }
                }
                _ => ()
            }
        }
    }
}
