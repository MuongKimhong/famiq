use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::render::renderer::RenderAdapterInfo;
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
        .add_systems(Update, test_mouse_event)
        .add_systems(Update, handle_value_change)
        .run();
}

fn test_mouse_event(
    mut events: EventReader<FaMouseEvent>
) {
    for e in events.read() {
        println!("{:?}", e);
    }
}

fn create_buttons(builder: &mut FamiqBuilder) -> Entity {
    let mut buttons: Vec<Entity> = Vec::new();

    for class_name in COLORS.iter() {
        let btn_class = format!("{class_name} mx-2 my-2");
        let button = fa_button(builder, class_name).tooltip("hello").class(btn_class.as_str()).build();
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
    let input_text_one = fa_text(builder, "").id("#input-text-one").class("input").build();
    let input_one = fa_text_input(builder, "What's on your mind?")
        .id("#input-one")
        .class("input mx-2")
        .build();

    let input_text_two = fa_text(builder, "").id("#input-text-two").class("input").build();
    let input_two = fa_text_input(builder, "What's on your mind?")
        .id("#input-two")
        .class("input is-dark mx-2")
        .build();

    fa_container(builder).class("my-2  block")
        .children([input_text_one, input_text_two, input_one, input_two])
        .build()
}

fn create_selections(builder: &mut FamiqBuilder) -> Entity {
    let choices: [&str; 2] = ["choice1", "choice2"];

    let text_one = fa_text(builder, "").id("#selection-text-one").class("input").build();
    let selection_one = fa_selection(builder, "Choose a choice")
        .id("#selection-one")
        .class("input mx-2")
        .choices(choices)
        .build();

    let text_two = fa_text(builder, "").id("#selection-text-two").class("input").build();
    let selection_two = fa_selection(builder, "Choose a choice")
        .id("#selection-two")
        .class("input is-dark mx-2")
        .choices(choices)
        .build();

    fa_container(builder).class("my-2 block")
        .children([text_one, text_two, selection_one, selection_two])
        .build()
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
    asset_server: Res<AssetServer>,
    adapter: Res<RenderAdapterInfo>
) {
    commands.spawn(Camera2d::default());

    let adapter_info = adapter.0.clone().into_inner();
    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
        .hot_reload();

    fa_fps(&mut builder).right_side().build();
    fa_bg_image(&mut builder, "wallpaper.jpg").build();

    let title = fa_text(&mut builder, "Welcome to Famiq").class("h2 my-2 mx-auto").build();

    let name = fa_text(&mut builder, &format!("GPU: {}", adapter_info.name))
        .class("ml-2")
        .build();

    let driver = fa_text(&mut builder, &format!("Driver version: {}", adapter_info.driver_info))
        .class("ml-2")
        .build();

    let backend = fa_text(&mut builder, &format!("Rendering engine: {}", adapter_info.backend.to_str()))
        .class("ml-2")
        .build();

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
            title, name, driver, backend,
            btn_container, circular_container, text_input_container, selection_container,
            image_container, bar_container
        ])
        .build();
}

fn handle_value_change(
    mut text_input_change: EventReader<FaTextInputChangeEvent>,
    mut selection_change: EventReader<FaSelectionChangeEvent>,
    mut text_res: ResMut<FaTextResource>
) {
    for input_change in text_input_change.read() {
        if let Some(changed_id) = input_change.widget_id.as_ref() {
            match changed_id.as_str() {
                "#input-one" => {
                    text_res.update_value("#input-text-one", &input_change.new_value);
                },
                "#input-two" => {
                    text_res.update_value("#input-text-two", &input_change.new_value);
                },
                _ => {}
            }
        }
    }

    for select_change in selection_change.read() {
        if let Some(changed_id) = select_change.widget_id.as_ref() {
            match changed_id.as_str() {
                "#selection-one" => {
                    text_res.update_value("#selection-text-one", &select_change.new_value);
                },
                "#selection-two" => {
                    text_res.update_value("#selection-text-two", &select_change.new_value);
                },
                _ => {}
            }
        }
    }
}
