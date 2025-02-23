use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin) // add plugin
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>, // required
    asset_server: Res<AssetServer>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server);

    let txt = fa_text(&mut builder, "SIGN UP").build();

    let f_name = fa_text_input(&mut builder, "First name").class("text-input").build();
    let l_name = fa_text_input(&mut builder, "Last name").class("text-input").build();
    let name_container = fa_container(&mut builder)
        .id("#name-container")
        .children([f_name, l_name])
        .build();

    let ask = fa_selection(&mut builder, "Select subscription").choices(["Personal", "Team"]).build();
    let ask_container = fa_container(&mut builder).id("#ask-container")
        .children([ask])
        .build();

    let btn = fa_button(&mut builder, "Confirm").build();

    fa_container(&mut builder)
        .id("#container")
        .children([txt, name_container, ask_container, btn])
        .build();
}
