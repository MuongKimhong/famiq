use bevy::prelude::*;
use famiq::prelude::*;
use super::{LikeCount, LikeTextEntity};

pub fn create_post(
    builder: &mut FamiqWidgetBuilder,
    username: &str,
    caption: &str,
    image_path: &str
) -> Entity {
    let user = builder.fa_text(
        format!("#post-{username}").as_str(),
        "username",
        username
    );
    let title = builder.fa_text(
        format!("#post-{caption}").as_str(),
        "title",
        caption
    );
    let title_wrapper = builder.fa_container(
        format!("#post-{caption}-wrapper").as_str(),
        "title-wrapper",
        &vec![title]
    );
    let image = builder.fa_image(
        format!("#post-{image_path}").as_str(),
        "image",
        "100%",
        "450px",
        image_path
    );
    let like_txt = builder.fa_text(
        format!("#post-{username}-like-txt").as_str(),
        "like-txt",
        "0"
    );
    builder.insert_component(like_txt, LikeCount(0));

    let like_btn = builder.fa_button(
        format!("#post-{username}-like-btn").as_str(),
        "like-btn is-small is-primary-dark",
        "♥"
    );
    builder.insert_component(like_btn, LikeTextEntity(like_txt));

    let action_container = builder.fa_container(
        format!("#post-{username}-action-container").as_str(),
        "action-container",
        &vec![like_txt, like_btn]
    );

    builder.fa_container(
        format!("#post-{username}-container").as_str(),
        "post-container",
        &vec![user, title_wrapper, image, action_container]
    )
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
