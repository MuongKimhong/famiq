use bevy::prelude::*;
use famiq::prelude::*;

#[derive(Component)]
pub struct LikeCount(pub i32);

#[derive(Component)]
pub struct LikeTextEntity(pub Entity);

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
        .children([title])
        .build();

    let image = fa_image(builder, image_path)
        .class("image")
        .size(Val::Percent(100.0), Val::Px(400.0))
        .build();

    let like_txt = fa_text(builder, "0").class("like-txt").build();
    builder.insert_component(like_txt, LikeCount(0));


    let like_btn = fa_button(builder, "♥")
        .class("like-btn is-small is-primary-dark ml-1 mt-1")
        .build();
    builder.insert_component(like_btn, LikeTextEntity(like_txt));

    let action_container = fa_container(builder)
        .class("action-container")
        .children([like_txt, like_btn])
        .build();

    fa_container(builder)
        .class("post-container")
        .children([user, title_wrapper, image, action_container])
        .build()
}

pub fn create_posts(builder: &mut FamiqWidgetBuilder) -> Entity {
    let post_1 = create_post(builder, "Richard", "My dog is so lovely", "dog.jpg");
    let post_2 = create_post(builder, "Lux", "Feeling sad today", "lux.jpg");
    let post_3 = create_post(builder, "Sett", "I'm sett the beast", "sett.jpg");
    let post_4 = create_post(builder, "Ray", "Cat is like water", "cat.jpg");

    fa_listview(builder)
        .id("#posts-listview")
        .children([post_1, post_2, post_3, post_4])
        .build()
}
