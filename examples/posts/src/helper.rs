use bevy::prelude::*;
use famiq::prelude::*;

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
        "",
        "",
        image_path
    );
    let like_btn = builder.fa_button(
        format!("#post-{username}-like-btn").as_str(),
        "like-btn is-small is-primary-dark",
        "♥"
    );

    builder.fa_container(
        format!("#post-{username}-container").as_str(),
        "post-container",
        &vec![user, title_wrapper, image, like_btn]
    )
}
