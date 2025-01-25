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
    mut builder_res: ResMut<FamiqWidgetResource>, // required
    asset_server: ResMut<AssetServer>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server)
        .hot_reload();

    let txt = fa_text(&mut builder, "SIGN UP").build();

    let f_name = fa_text_input(&mut builder, "First name").id("#first-name").build();
    let l_name = fa_text_input(&mut builder, "Last name").id("#last-name").build();
    let name_container = fa_container(&mut builder)
        .id("#name-container")
        .children(vec![f_name, l_name])
        .build();

    let ask = fa_selection(&mut builder, "Select your subscription")
        .choices(vec!["Personal", "Team"])
        .build();
    let ask_container = fa_container(&mut builder).id("#ask-container")
        .children(vec![ask])
        .build();

    let btn = fa_button(&mut builder, "Confirm").build();

    fa_container(&mut builder).id("#container")
        .children(vec![txt, name_container, ask_container, btn])
        .build();
}
