use crate::widgets::selection::*;
use crate::widgets::FamiqResource;
use super::FaSelection;
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
            FaSelection::arrow_up(&mut arrow_q, arrow_entity.0);
        }
        else {
            panel_node.display = Display::None;
            FaSelection::arrow_down(&mut arrow_q, arrow_entity.0);
        }
    }
}

/// Internal system to detect new selection being created.
pub fn detect_new_selection_widget_system(
    selection_q: Query<Option<&FamiqWidgetId> , Added<IsFamiqSelectionSelector>>,
    mut selection_res: ResMut<FaSelectionResource>
) {
    for id in selection_q.iter() {
        if let Some(id) = id {
            if !selection_res.exists(id.0.as_str()) {
                selection_res._insert(id.0.clone(), String::new());
            }
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
    mut selection_q: Query<(
        Entity,
        &Selection,
        &mut SelectionValue,
        Option<&FamiqWidgetId>,
        &mut SelectorPlaceHolderEntity,
    )>,
    mut selection_res: ResMut<FaSelectionResource>,
    mut text_q: Query<&mut Text>,
    mut builder_res: ResMut<FamiqResource>
) {
    for (mut choice_bg_color, interaction, choice_txt_entity, selector_entity) in selection_choice_q.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *choice_bg_color = ITEM_ON_HOVER_BG_COLOR.into();
            },
            Interaction::Pressed => {
                if let Ok((
                    selection_entity,
                    selection,
                    mut selection_value,
                    selection_id,
                    placeholder_entity,
                )) = selection_q.get_mut(selector_entity.0) {
                    let mut selected_choice = String::new();

                    // Update selected items resource
                    if let Ok(text) = text_q.get_mut(choice_txt_entity.0) {
                        selected_choice = if text.0 == "-/-" {
                            String::from("")
                        } else {
                            text.0.clone()
                        };
                        if let Some(id) = selection_id {
                            selection_res._insert(id.0.clone(), text.0.clone());
                        }
                    }

                    // update placeholder value
                    if let Ok(mut text) = text_q.get_mut(placeholder_entity.0) {
                        if selected_choice != "" {
                            text.0 = selected_choice.clone();
                            selection_value.0 = selected_choice.clone();
                        } else {
                            text.0 = selection.placeholder.clone();
                            selection_value.0 = String::new();
                        }
                    }

                    // set selection to unfocus after choice is selected
                    builder_res.update_or_insert_focus_state(selection_entity, false);
                    *choice_bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                }
            },
            _ => {
                *choice_bg_color = ITEM_NORMAL_BG_COLOR.into();
            }
        }
    }
}
