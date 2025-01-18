pub mod helper;
use helper::*;

use bevy::prelude::*;
use bevy::window::WindowTheme;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Engine - Famiq".into(),
                    resolution: (500., 1000.).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(famiq_plugin) // add plugin
        .add_systems(Startup, setup)
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

    builder.fa_fps("#fps", true);

    let post_1 = create_post(
        &mut builder,
        "Richard",
        "My dog is so lovely",
        "dog.jpg"
    );
    let post_2 = create_post(
        &mut builder,
        "Lux from LOL",
        "Feeling sad today",
        "lux.jpg"
    );
    let post_3 = create_post(
        &mut builder,
        "Sett from LOL",
        "I'm sett",
        "sett.jpg"
    );
    let post_4 = create_post(
        &mut builder,
        "Jubal",
        "Cat is like water",
        "cat.jpg"
    );

    builder.fa_list_view(
        "#container",
        "",
        &vec![post_1, post_2, post_3, post_4]
    );
}
