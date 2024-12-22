use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(famiq_plugin) // add plugin
        .add_systems(Startup, setup)
        // .add_systems(Update, handle_btn_press)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>, // required
    mut builder_resource: ResMut<FamiqWidgetBuilderResource>, // required
) {
    commands.spawn(Camera2d::default());

    // create a builder
    let mut ui = FamiqWidgetBuilder::new(
        &mut commands,
        &asset_server,
        &mut builder_resource,
        "assets/fonts/fira-mono-regular.ttf", // should be inside assets folder & outside src folder
        "assets/styles.json",                 // should be inside assets folder & outside src folder
        true,                                 // hot-reload, useful during development
    );

    // fa_fps
    // ui.fa_fps("#fps-text");

    // // fa_text needs container
    // let txt = ui.fa_text("#signup-txt", "SIGN UP");
    // let txt_container = ui.fa_container("#signup-txt-container", &vec![txt]);

    // // fa_text_input needs container
    // let f_name = ui.fa_text_input("#f_name", "First name", None, None);
    // let l_name = ui.fa_text_input("#l_name", "Last name", None, None);
    // let name_container = ui.fa_container("#name-container", &vec![f_name, l_name]);

    // // fa_selection doesn't need containers
    // let choices = vec!["Personal".to_string(), "Team".to_string()];
    // let ask = ui.fa_selection(
    //     "#ask",
    //     "Select your subscription",
    //     &choices,
    //     None,
    //     None,
    //     None,
    // );

    // fa_button needs container
    ui.fa_fps("#fpstext");
    let txt = ui.fa_text("#txt", "Hello world");
    let btn = ui.fa_button("#btn", "Confirm", "secondary", "");
    let btn_one = ui.fa_button("#btn", "Confirm", "warning", "normal");
    let btn_two = ui.fa_button("#btn", "Confirm", "danger", "large");
    let input = ui.fa_text_input("#input", "Enter your name", "", "");

    let choices = vec!["hello".to_string(), "world".to_string()];
    let selection = ui.fa_selection("#hello", "Select", &choices, "", "", "");
    let btn_container = ui.fa_container("#btn-container", &vec![btn, btn_one, btn_two, txt, input, selection]);

    // ui.fa_container(
    //     "#container",
    //     &vec![txt_container, name_container, ask, btn_container],
    // );
}

// fn handle_btn_press(
//     mut events: EventReader<FaInteractionEvent>,
//     selected_items: Res<SelectedItemsResource>,
//     input_values: Res<TextInputResource>,
// ) {
//     for e in events.read() {
//         if e.widget_type == WidgetType::Button && e.interaction_type == Interaction::Pressed {
//             match e.widget_id.as_str() {
//                 "#btn" => {
//                     if let Some(selected) = selected_items.items.get("#ask") {
//                         println!("selected choice: {:?}", selected);
//                     }

//                     if let Some(first_name) = input_values.inputs.get("#f_name") {
//                         println!("Frist name: {:?}", first_name);
//                     }

//                     if let Some(last_name) = input_values.inputs.get("#l_name") {
//                         println!("Last name: {:?}", last_name);
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     }
// }
