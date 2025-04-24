use crate::widgets::selection::*;
use crate::widgets::FamiqResource;
use bevy::prelude::*;

pub fn handle_show_and_hide_choices_panel(
    selection_q: Query<(
        Entity,
        &ComputedNode,
        &GlobalTransform,
        &SelectionChoicesPanelEntity,
        &SelectorArrowIconEntity
    )>,
    mut panel_q: Query<
        (&mut Node, &mut Transform),
        With<IsFamiqSelectionChoicesPanel>
    >,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,
    builder_res: Res<FamiqResource>
) {
    if !builder_res.is_changed() || builder_res.is_added() {
        return;
    }
    for (entity, computed_node, transform, panel_entity, arrow_entity) in selection_q.iter() {
        let Some(focused) = builder_res.get_widget_focus_state(&entity) else { continue };
        let Ok((mut panel_node, mut panel_transform)) = panel_q.get_mut(panel_entity.0) else { continue };

        if focused {
            panel_transform.translation = transform.translation();
            panel_node.width = Val::Percent(100.0);
            panel_node.top = Val::Px(computed_node.size().y * computed_node.inverse_scale_factor());
            panel_node.display = Display::Flex;
            SelectionBuilder::arrow_up(&mut arrow_q, arrow_entity.0);
        }
        else {
            panel_node.display = Display::None;
            SelectionBuilder::arrow_down(&mut arrow_q, arrow_entity.0);
        }
    }
}

pub fn handle_selection_choice_interaction_system(
    mut selection_choice_q: Query<
        (
            &mut BackgroundColor,
            &Interaction,
            &SelectionChoiceTextEntity,
            &SelectorEntity
        ),
        (With<IsFamiqSelectionChoice>, Changed<Interaction>)
    >,
    mut selection_q: Query<(Entity, Option<&ReactiveModelKey>), With<IsFamiqSelectionSelector>>,
    mut fa_query: FaQuery,
    mut text_q: Query<&mut Text>,
    mut builder_res: ResMut<FamiqResource>,
) {
    for (mut choice_bg_color, interaction, choice_txt_entity, selector_entity) in selection_choice_q.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *choice_bg_color = ITEM_ON_HOVER_BG_COLOR.into();
            },
            Interaction::Pressed => {
                if let Ok((selection_entity, model_key)) = selection_q.get_mut(selector_entity.0) {
                    if let Some(key) = model_key {
                        let mut selected_choice = String::new();

                        if let Ok(text) = text_q.get_mut(choice_txt_entity.0) {
                            selected_choice = if text.0 == "-/-" {
                                String::from("")
                            } else {
                                text.0.clone()
                            };
                        }
                        if let Some(value) = fa_query.get_data_mut(&key.0) {
                            match value {
                                RVal::Str(v) => *v = selected_choice,
                                _ => {}
                            }
                        }
                        println!("{:?}", fa_query.reactive_data.data);
                        // set selection to unfocus after choice is selected
                        builder_res.update_or_insert_focus_state(selection_entity, false);
                        *choice_bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                    }
                }
            },
            _ => {
                *choice_bg_color = ITEM_NORMAL_BG_COLOR.into();
            }
        }
    }
}

pub fn set_placeholder_with_model(
    placeholder_q: &mut Query<&mut Text, With<SelectorPlaceHolder>>,
    reactive_data: &HashMap<String, RVal>,
    ph_entity: Entity,
    ph_text: &String,
    key: &String
) {
    if let Some(r_value) = reactive_data.get(key) {
        if let Ok(mut text) = placeholder_q.get_mut(ph_entity) {
            match r_value {
                RVal::Str(v) => {
                    if v.trim() == "" {
                        text.0 = ph_text.to_owned();
                    } else {
                        text.0 = v.to_owned();
                    }
                },
                _ => {}
            }
        }
    }
}

pub fn detect_selection_reactive_model_change(
    reactive_data: Res<RData>,
    selector_q: Query<
        (&SelectorPlaceHolderEntity, Option<&ReactiveModelKey>, &SelectorPlaceholderText),
        With<IsFamiqSelectionSelector>
    >,
    mut ph_q: Query<&mut Text, With<SelectorPlaceHolder>>
) {
    if !reactive_data.is_changed() && reactive_data.is_added() {
        return;
    }
    for (ph_entity, model_key, ph_text) in selector_q.iter() {
        if model_key.is_none() {
            return;
        }
        let key = &model_key.unwrap().0;
        set_placeholder_with_model(&mut ph_q, &reactive_data.data, ph_entity.0, &ph_text.0, key);
    }
}
