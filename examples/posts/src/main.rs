pub mod helper;
use helper::*;

use bevy::prelude::*;
use famiq::prelude::*;

#[derive(Component)]
pub struct LikeCount(pub i32);

#[derive(Component)]
pub struct LikeTextEntity(pub Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(set_window()))
        .add_plugins(famiq_plugin) // add plugin
        .add_systems(Startup, setup)
        .add_systems(Update, handle_like_btn_press)
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

    let post_1 = create_post(&mut builder, "Richard", "My dog is so lovely", "dog.jpg");
    let post_2 = create_post(&mut builder, "Lux", "Feeling sad today", "lux.jpg");
    let post_3 = create_post(&mut builder, "Sett", "I'm sett the beast", "sett.jpg");
    let post_4 = create_post(&mut builder, "Ray", "Cat is like water", "cat.jpg");

    builder.fa_list_view(
        "#container",
        "",
        &vec![post_1, post_2, post_3, post_4]
    );
}

fn handle_like_btn_press(
    mut events: EventReader<FaInteractionEvent>,
    mut like_txt_q: Query<(&mut Text, &mut LikeCount)>,
    like_btn_q: Query<&LikeTextEntity>
) {
    for e in events.read() {
        if e.widget == WidgetType::Button && e.interaction == Interaction::Pressed {
            if let Ok(txt_entity) = like_btn_q.get(e.entity) {
                if let Ok((mut text, mut count)) = like_txt_q.get_mut(txt_entity.0) {
                    count.0 += 1;
                    text.0 = count.0.to_string();
                    break;
                }
            }
        }
    }
}
