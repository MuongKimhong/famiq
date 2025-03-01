use bevy::prelude::*;
use bevy::window::PresentMode;
use famiq::prelude::*;

const COLORS: [&str; 12] = [
    "is-primary", "is-secondary", "is-success", "is-danger", "is-info", "is-warning",
    "is-primary-dark", "is-success-dark", "is-danger-dark", "is-info-dark", "is-warning-dark",
    "is-dark"
];

fn custom_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Famiq demo".into(),
            resolution: (850.0, 650.0).into(),
            present_mode: PresentMode::Immediate,
            ..default()
        }),
        ..default()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(custom_window()))
        .add_plugins(FamiqPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn create_buttons(builder: &mut FamiqBuilder) -> Entity {
    let mut buttons: Vec<Entity> = Vec::new();

    for class_name in COLORS.iter() {
        let btn_class = format!("{class_name} mx-2 my-2");
        let button = fa_button(builder, class_name).class(btn_class.as_str()).build();
        buttons.push(button);
    }
    fa_container(builder).class("block my-4").children(buttons).build()
}

fn create_circulars(builder: &mut FamiqBuilder) -> Entity {
    let mut circulars: Vec<Entity> = Vec::new();

    for class_name in COLORS.iter() {
        let circular_class = format!("{class_name} mx-2 my-2");
        let circular = fa_circular(builder).class(circular_class.as_str()).build();
        circulars.push(circular);
    }
    fa_container(builder).class("block my-4").children(circulars).build()
}

fn create_text_inputs(builder: &mut FamiqBuilder) -> Entity {
    let input_one = fa_text_input(builder, "What's on your mind?").class("input mx-2").build();
    let input_two = fa_text_input(builder, "What's on your mind?").class("input is-dark mx-2").build();
    fa_container(builder).class("my-2  block").children([input_one, input_two]).build()
}

fn create_selections(builder: &mut FamiqBuilder) -> Entity {
    let choices: [&str; 2] = ["choice1", "choice2"];

    let selection_one = fa_selection(builder, "Choose a choice").class("input mx-2").choices(choices).build();
    let selection_two = fa_selection(builder, "Choose a choice").class("input is-dark mx-2").choices(choices).build();
    fa_container(builder).class("my-2 block").children([selection_one, selection_two]).build()
}

fn create_images(builder: &mut FamiqBuilder) -> Entity {
    let img_one = fa_image(builder, "logo.png")
        .class("mx-2")
        .set_size(Val::Px(150.), Val::Px(150.))
        .build();

    let img_two = fa_image(builder, "bevy_logo.png")
        .class("mx-2")
        .set_size(Val::Px(320.), Val::Px(150.))
        .build();

    let img_three = fa_image(builder, "rust_logo.png")
        .class("mx-2")
        .set_size(Val::Px(200.), Val::Px(150.))
        .build();

    fa_container(builder).class("my-3 block").children([img_one, img_two, img_three]).build()
}

fn create_progress_bar(builder: &mut FamiqBuilder) -> Entity {
    let bar_one = fa_progress_bar(builder).class("input mx-2").color("cyan_500").build();
    let bar_two = fa_progress_bar(builder)
        .class("input mx-2")
        .percentage(50.)
        .color("lime_900")
        .build();

    fa_container(builder).class("my-3 block").children([bar_one, bar_two]).build()
}

fn setup_ui(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d::default());

    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
        .hot_reload();

    fa_fps(&mut builder).change_color().build();
    fa_bg_image(&mut builder, "wallpaper.jpg").build();

    let title = fa_text(&mut builder, "Welcome to Famiq").class("h2 my-2 mx-auto").build();

    let btn_container = create_buttons(&mut builder);
    let circular_container = create_circulars(&mut builder);
    let text_input_container = create_text_inputs(&mut builder);
    let selection_container = create_selections(&mut builder);
    let image_container = create_images(&mut builder);
    let bar_container = create_progress_bar(&mut builder);

    fa_listview(&mut builder)
        .id("#main-container")
        .class("my-auto mx-auto")
        .children([
            title, btn_container, circular_container, text_input_container, selection_container,
            image_container, bar_container
        ])
        .build();
}
