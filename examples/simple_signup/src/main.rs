use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(famiq_plugin) // add plugin
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>, // required
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
        false,
    );

    let txt = builder.fa_text("#signup-txt", "", "SIGN UP");

    let f_name = builder.fa_text_input("#first-name", "", "First name");
    let l_name = builder.fa_text_input("#last-name", "", "Last name");
    let name_container = builder.fa_container("#name-container", "", &vec![f_name, l_name]);

    let ask = builder.fa_selection(
        "#ask",
        "",
        "Select your subscription",
        &vec!["Personal".to_string(), "Team".to_string()],
    );
    let ask_container = builder.fa_container("#ask-container", "", &vec![ask]);

    let btn = builder.fa_button("#btn", "is-secondary", "Confirm");

    builder.fa_container(
        "#container",
        "",
        &vec![txt, name_container, ask_container, btn],
    );
}
