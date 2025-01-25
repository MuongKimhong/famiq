use bevy::prelude::*;
use famiq::prelude::*;
use super::{LikeCount, LikeTextEntity};

pub fn create_post(
    builder: &mut FamiqWidgetBuilder,
    username: &str,
    caption: &str,
    image_path: &str
) -> Entity {
    let user = fa_text(builder, username).class("username").build();

    let title = fa_text(builder, caption).class("title").build();
    let title_wrapper = fa_container(builder)
        .class("title-wrapper")
        .children(vec![title])
        .build();

    let image = fa_image(builder, image_path)
        .class("image")
        .size(Val::Percent(100.0), Val::Px(450.0))
        .build();

    let like_txt = fa_text(builder, "0").class("like-txt").build();
    builder.insert_component(like_txt, LikeCount(0));


    let like_btn = fa_button(builder, "♥")
        .class("like-btn is-small is-primary-dark")
        .build();
    builder.insert_component(like_btn, LikeTextEntity(like_txt));

    let action_container = fa_container(builder)
        .class("action-container")
        .children(vec![like_txt, like_btn])
        .build();

    fa_container(builder)
        .class("post-container")
        .children(vec![user, title_wrapper, image, action_container])
        .build()
}

pub fn set_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Engine - Famiq".into(),
            resolution: (500., 1000.).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }
}
