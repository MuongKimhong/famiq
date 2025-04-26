use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FamiqPlugin))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, handle_btn_press)
        .run();
}

fn setup_ui(mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    fa_query.insert_str("first_name", "");
    fa_query.insert_str("last_name", "");
    fa_query.insert_str("select", "");
    fa_query.insert_num("counter", 0);

    #[cfg(target_arch = "wasm32")]
    {
        FamiqBuilder::new(&mut fa_query, &mut famiq_res).use_style_path("styles.json").inject();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        FamiqBuilder::new(&mut fa_query, &mut famiq_res).hot_reload().inject();
    }

    let logo_container = container!(
        class: "sub-container",
        children: [
            image!(path: "bevy_logo.png", class: "mr-5", width: "240px", height: "130px"),
            image!(path: "famiq_logo.png", class: "ml-5", width: "220px", height: "130px")
        ]
    );
    let input_container = container!(
        class: "sub-container",
        children: [
            text_input!(placeholder: "Enter first name", class: "input", model: "first_name"),
            text_input!(placeholder: "Enter last name", class: "input", model: "last_name"),
        ]
    );
    let counter_buttons = container!(
        class: "sub-container",
        children: [
            button!(text: "Increase", id: "#increase", class: "success-dark mr-2"),
            button!(text: "Decrease", id: "#decrease", class: "warning-dark ml-2")
        ]
    );
    container!(
        id: "#container",
        class: "mx-auto my-auto",
        children: [
            logo_container,
            text!(text: "Welcome to Bevy + Famiq\n What you see are not HTML!", class: "h3 mt-2"),

            circular!(color: "cyan", size: 50.0, class: "mt-2"),
            text!(text: "Name: $[first_name] $[last_name]", class: "h4 mt-3 mb-2 primary"),

            selection!(
                placeholder: "Choose one",
                class: "input",
                model: "select",
                choices: ["something", "nah"]
            ),
            input_container,

            text!(text: "Counter: $[counter]", class: "h3 mt-2"),
            counter_buttons,
        ]
    );
}

fn handle_btn_press(
    mut events: EventReader<FaMouseEvent>,
    mut fa_query: FaQuery
) {
    for e in events.read() {
        if let Some(counter) = fa_query.get_data_mut("counter") {
            let mut counter = counter.as_num();

            if e.is_button_pressed("#increase") {
                counter += 1;
                fa_query.mutate_num("counter", counter);
            }
            else if e.is_button_pressed("#decrease") {
                counter -= 1;
                fa_query.mutate_num("counter", counter);
            }
        }
    }
}
