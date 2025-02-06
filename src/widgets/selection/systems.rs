use crate::widgets::selection::*;
use crate::widgets::{WidgetType, FamiqWidgetResource};
use crate::event_writer::FaInteractionEvent;
use super::FaSelection;
use bevy::prelude::*;

const PANEL_TOP_OFFSET: f32 = 10.0;

fn _set_choice_panel_position_and_width(
    selector_translation: &Vec3,
    selector_computed_node: &ComputedNode,
    panel_node: &mut Node,
) {
    let top_pos = selector_translation.y + (selector_computed_node.size().y / 2.0) + PANEL_TOP_OFFSET;
    let left_pos = selector_translation.x - (selector_computed_node.size().x / 2.0);

    panel_node.left = Val::Px(left_pos);
    panel_node.top = Val::Px(top_pos);
    panel_node.width = Val::Px(selector_computed_node.size().x);
}

pub fn update_choices_panel_position_and_width_system(
    selection_q: Query<
        (
            Entity,
            &ComputedNode,
            &GlobalTransform,
            &SelectionChoicesPanelEntity
        )
    >,
    mut panel_q: Query<&mut Node, With<IsFamiqSelectionChoicesPanel>>,
    builder_res: Res<FamiqWidgetResource>
) {
    for (entity, computed_node, transform, panel_entity) in selection_q.iter() {
        let Some(focused) = builder_res.get_widget_focus_state(&entity) else { continue };

        if focused {
            let Ok(mut panel_node) = panel_q.get_mut(panel_entity.0) else { continue };

            _set_choice_panel_position_and_width(
                &transform.translation(),
                computed_node,
                &mut panel_node
            );
        }
    }
}

pub fn handle_selection_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selector_q: Query<
        (
            &mut BoxShadow,
            &DefaultWidgetEntity,
            &SelectorArrowIconEntity,
            &SelectionChoicesPanelEntity,
        ),
        Without<IsFamiqSelectionChoicesPanel>
    >,
    mut builder_res: ResMut<FamiqWidgetResource>,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,
    mut panel_q: Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,

) {
    for e in events.read() {
        if e.widget == WidgetType::Selection {
            if let Ok((
                mut box_shadow,
                default_style,
                arrow_entity,
                panel_entity,
            )) = selector_q.get_mut(e.entity)
            {
                match e.interaction {
                    Interaction::Hovered => {
                        box_shadow.color = default_style.border_color.0.clone();
                    },
                    Interaction::Pressed => {
                        // currently true, set back to false
                        if let Some(state) = builder_res.get_widget_focus_state(&e.entity) {
                            if state {
                                builder_res.update_or_insert_focus_state(e.entity, false);
                                FaSelection::arrow_down(&mut arrow_q, arrow_entity.0);
                                FaSelection::hide_choice_panel(&mut panel_q, panel_entity.0);
                                break;
                            }
                        }

                        // currently false, set back to true
                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                        FaSelection::arrow_up(&mut arrow_q, arrow_entity.0);
                        FaSelection::show_choice_panel(&mut panel_q, panel_entity.0);
                    },
                    _ => {
                        box_shadow.color = Color::NONE;
                    }
                }
            }
        }
    }
}

/// Internal system to detect new selection being created.
pub fn detect_new_selection_widget_system(
    selection_q: Query<
        (
            Entity,
            Option<&FamiqWidgetId>,
            &ComputedNode,
            &GlobalTransform,
            &SelectionChoicesPanelEntity
        ),
        Added<IsFamiqSelectionSelector>
    >,
    mut panel_q: Query<&mut Node, With<IsFamiqSelectionChoicesPanel>>,
    mut selection_res: ResMut<FaSelectionResource>
) {
    for (entity, id, computed_node, transform, panel_entity) in selection_q.iter() {
        if let Some(id) = id {
            if !selection_res.exists_by_id(id.0.as_str()) {
                selection_res._insert_by_id(id.0.clone(), String::new());
            }
        }

        if !selection_res.exists_by_entity(entity) {
            selection_res._insert_by_entity(entity, String::new());
        }

        let Ok(mut panel_node) = panel_q.get_mut(panel_entity.0) else { continue };
        _set_choice_panel_position_and_width(
            &transform.translation(),
            computed_node,
            &mut panel_node
        );
    }
}

pub fn handle_selection_choice_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selection_choice_q: Query<
        (
            &mut BackgroundColor,
            &SelectionChoiceTextEntity,
            &SelectorEntity
        ),
        (With<IsFamiqSelectionChoice>, Without<IsFamiqSelectionChoicesPanel>, Without<SelectorPlaceHolderEntity>)
    >,
    mut selection_q: Query<(
        Entity,
        &Selection,
        Option<&FamiqWidgetId>,
        &mut SelectorPlaceHolderEntity,
        &SelectorArrowIconEntity,
        &SelectionChoicesPanelEntity
    )>,
    mut selection_res: ResMut<FaSelectionResource>,
    mut text_q: Query<&mut Text, Without<ArrowIcon>>,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,
    mut panel_q: Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,
    mut builder_res: ResMut<FamiqWidgetResource>
) {
    for e in events.read() {
        if let Ok((
            mut choice_bg_color,
            choice_txt_entity,
            selector_entity
        )) = selection_choice_q.get_mut(e.entity) {


            match e.interaction {
                Interaction::Hovered => {
                    *choice_bg_color = ITEM_ON_HOVER_BG_COLOR.into();
                },
                Interaction::Pressed => {
                    if let Ok((
                        selection_entity,
                        selection,
                        selection_id,
                        placeholder_entity,
                        arrow_entity,
                        panel_entity
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
                                selection_res._insert_by_id(id.0.clone(), text.0.clone());
                            }
                            selection_res._insert_by_entity(selection_entity, text.0.clone());
                        }

                        // update placeholder value
                        if let Ok(mut text) = text_q.get_mut(placeholder_entity.0) {
                            if selected_choice != "" {
                                text.0 = selected_choice.clone();
                            } else {
                                text.0 = selection.placeholder.clone();
                            }
                        }

                        // set selection to unfocus after choice is selected
                        builder_res.update_or_insert_focus_state(selection_entity, false);
                        FaSelection::arrow_down(&mut arrow_q, arrow_entity.0);
                        FaSelection::hide_choice_panel(&mut panel_q, panel_entity.0);

                        *choice_bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                    }
                },
                Interaction::None => {
                    *choice_bg_color = ITEM_NORMAL_BG_COLOR.into();
                }
            }
        }
    }
}
