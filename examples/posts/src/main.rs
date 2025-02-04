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
        .add_plugins(FamiqPlugin) // add plugin
        .add_systems(Startup, setup)
        .add_systems(Update, handle_like_btn_press)
        .run();
}

fn setup(
    asset_server: ResMut<AssetServer>, // required
    mut commands: Commands,
    mut builder_res: ResMut<FamiqWidgetResource>, // required
) {
    commands.spawn(Camera2d::default());

    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
    fa_fps(&mut builder).change_color().build();

    let post_1 = create_post(&mut builder, "Richard", "My dog is so lovely", "dog.jpg");
    let post_2 = create_post(&mut builder, "Lux", "Feeling sad today", "lux.jpg");
    let post_3 = create_post(&mut builder, "Sett", "I'm sett the beast", "sett.jpg");
    let post_4 = create_post(&mut builder, "Ray", "Cat is like water", "cat.jpg");

    fa_listview(&mut builder)
        .id("#container")
        .children([post_1, post_2, post_3, post_4])
        .build();
}

fn handle_like_btn_press(
    mut events: EventReader<FaInteractionEvent>,
    mut text_res: ResMut<FaTextResource>,
    mut like_txt_q: Query<(Entity, &mut LikeCount)>,
    like_btn_q: Query<&LikeTextEntity>
) {
    for e in events.read() {
        if e.is_button_pressed() {
            if let Ok(txt_entity) = like_btn_q.get(e.entity) {

                if let Ok((entity, mut count)) = like_txt_q.get_mut(txt_entity.0) {
                    count.0 += 1;
                    text_res.update_value_by_entity(entity, &count.0.to_string());
                    break;
                }
            }
        }
    }
}
