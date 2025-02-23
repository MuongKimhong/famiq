use bevy::prelude::*;
use famiq::prelude::*;

pub fn create_inputs(builder: &mut FamiqBuilder) -> Entity {
    let input_one = fa_text_input(builder, "First name").id("#firstname").class("input").build();
    let input_two = fa_text_input(builder, "Last name").id("#lastname").class("input is-primary").build();

    fa_container(builder)
        .class("flex-container")
        .children([input_one, input_two])
        .build()
}

pub fn create_selections(builder: &mut FamiqBuilder) -> Entity {
    let select_one = fa_selection(builder, "Flash or Teleport?")
        .id("#select-one")
        .class("select")
        .choices(["Flash", "Teleport", "Nah I'm good"])
        .build();

    let select_two = fa_selection(builder, "Shield or blade?")
        .id("#select-two")
        .class("select is-info")
        .choices(["Doran's shield", "Doran's blade", "Nah I'm bringing my fist"])
        .build();

    fa_container(builder).class("flex-container").children([select_one, select_two]).build()
}

pub fn create_modal(builder: &mut FamiqBuilder) -> Entity {
    let your_name = fa_text(builder, "").id("#yourname").class("my-2 h3").build();
    let flash_or_tp = fa_text(builder, "").id("#flash-tp").class("my-2 h3").build();
    let shield_blade = fa_text(builder, "").id("#shield-blade").class("my-2 h3").build();
    let close_btn = fa_button(builder, "Close").id("#close-btn").class("mt-5 mb-2 h3").build();

    let modal_container = fa_container(builder)
        .id("#modal-container")
        .children([your_name, flash_or_tp, shield_blade, close_btn])
        .build();

    fa_modal(builder).id("#modal").children([modal_container]).build()
}

pub fn create_circulars(builder: &mut FamiqBuilder) -> Entity {
    let custom_cir = fa_circular(builder).size(90.0).class("is-warning")
        .tooltip("This is circular with Custom size 50.0")
        .build();

    let large_cir = fa_circular(builder).class("is-large is-success")
        .tooltip("This is large circular & success")
        .build();

    let normal_cir = fa_circular(builder).tooltip("This is default circular").build();
    let info_cir = fa_circular(builder).tooltip("This is info circular").class("is-info").build();
    let second_cir = fa_circular(builder).tooltip("This is secondary circular").class("is-secondary").build();
    let small_cir = fa_circular(builder).tooltip("This is small circular & danger").class("is-small is-danger").build();

    fa_container(builder)
        .class("flex-container my-5")
        .children([custom_cir, large_cir, normal_cir, info_cir, second_cir, small_cir])
        .build()
}

pub fn create_progress_bars(builder: &mut FamiqBuilder) -> Entity {
    let large_bar = fa_progress_bar(builder).class("is-large is-danger my-5").build();

    let normal_bar = fa_progress_bar(builder)
        .id("#normal-bar")
        .class("my-5 is-info")
        .percentage(50.0)
        .build();

    let small_bar = fa_progress_bar(builder).class("my-5 is-small is-success").build();

    let update_bar_btn_minus = fa_button(builder, "Decrease").id("#minus-btn").class("mr-3 is-danger").build();
    let update_bar_btn_plus = fa_button(builder, "Increase").id("#plus-btn").class("ml-3 is-primary").build();
    let update_bar_container = fa_container(builder)
        .id("#update-bar-container")
        .children([update_bar_btn_minus, update_bar_btn_plus])
        .build();

    fa_container(builder).children([large_bar, normal_bar, small_bar, update_bar_container]).build()
}
