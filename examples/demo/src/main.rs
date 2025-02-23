mod left;
mod right;

use bevy::prelude::*;
use bevy::window::PresentMode;
use famiq::prelude::*;

pub fn set_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Famiq - Demo".into(),
            present_mode: PresentMode::Immediate,
            resizable: false,
            ..default()
        }),
        ..default()
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(set_window()))
        .add_plugins(FamiqPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_left_buttons_press, handle_like_button_press))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut famiq_res: ResMut<FamiqResource>,
) {
    commands.spawn(Camera2d::default());

    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
        .hot_reload()
        .register_tooltip();

    fa_fps(&mut builder).change_color().build();
    fa_bg_image(&mut builder, "wallpaper.jpg").build();

    // left
    let inputs = left::create_inputs(&mut builder);
    let selections = left::create_selections(&mut builder);
    let enter_btn = fa_button(&mut builder, "Enter now").id("#enter-btn").class("is-success-dark mt-2").build();
    let modal = left::create_modal(&mut builder);
    let circulars = left::create_circulars(&mut builder);
    let progress_bars = left::create_progress_bars(&mut builder);

    let left_container = fa_container(&mut builder)
        .id("#left-container")
        .children([
            inputs, selections, enter_btn, circulars, progress_bars
        ])
        .build();

    // right
    let posts = right::create_posts(&mut builder);
    let right_container = fa_container(&mut builder)
        .id("#right-container")
        .children([posts])
        .build();

    fa_container(&mut builder)
        .id("#main-container")
        .children([left_container, right_container])
        .build();
}


fn handle_left_buttons_press(
    mut events: EventReader<FaInteractionEvent>,
    mut progress_res: ResMut<FaProgressBarResource>,
    mut text_res: ResMut<FaTextResource>,
    mut modal_state: ResMut<FaModalState>,
    input_res: Res<FaTextInputResource>,
    select_res: Res<FaSelectionResource>,
) {
    for e in events.read() {
        if e.is_pressed(WidgetType::Button) {
            if let Some(id) = e.widget_id.as_ref() {
                match id.as_str() {
                    "#minus-btn" => {
                        let mut current_percent = progress_res.get_percentage_by_id("#normal-bar").unwrap();

                        if current_percent > 0.0{
                            current_percent -= 10.0;
                            progress_res.set_percentage_by_id("#normal-bar", Some(current_percent));
                        }
                    },
                    "#plus-btn" => {
                        let mut current_percent = progress_res.get_percentage_by_id("#normal-bar").unwrap();

                        if current_percent < 100.0{
                            current_percent += 10.0;
                            progress_res.set_percentage_by_id("#normal-bar", Some(current_percent));
                        }
                    },
                    "#enter-btn" => {
                        let first_name = input_res.get_value_by_id("#firstname");
                        let last_name = input_res.get_value_by_id("#lastname");
                        let flash_or_tp = select_res.get_value_by_id("#select-one");
                        let shield_or_blade = select_res.get_value_by_id("#select-two");

                        if !first_name.is_empty() && !last_name.is_empty() && !flash_or_tp.is_empty() && !shield_or_blade.is_empty() {
                            text_res.update_value_by_id(
                                "#yourname",
                                format!("Name: {first_name} {last_name}").as_str()
                            );
                            text_res.update_value_by_id(
                                "#flash-tp",
                                format!("Spell: {flash_or_tp}").as_str()
                            );
                            text_res.update_value_by_id(
                                "#shield-blade",
                                format!("Weapon: {shield_or_blade}").as_str()
                            );
                            modal_state.show_by_id("#modal");
                        }
                    },
                    "#close-btn" => {
                        modal_state.hide_by_id("#modal");
                    }
                    _ => {}
                }
            }

        }
    }
}

fn handle_like_button_press(
    mut events: EventReader<FaInteractionEvent>,
    mut text_res: ResMut<FaTextResource>,
    mut like_txt_q: Query<(Entity, &mut right::LikeCount)>,
    like_btn_q: Query<&right::LikeTextEntity>
) {
    for e in events.read() {
        if e.is_pressed(WidgetType::Button) {
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
