use crate::widgets::selection::*;
use crate::widgets::{WidgetType, FamiqWidgetResource};
use crate::event_writer::FaInteractionEvent;
use crate::utils;
use super::FaSelection;
use bevy::prelude::*;

pub fn update_choices_panel_position_and_width_system(
    selection_q: Query<
        (
            Entity,
            &Node,
            &ComputedNode,
            &SelectionChoicesPanelEntity
        )
    >,
    mut panel_q: Query<
        (
            &IsFamiqSelectionChoicesPanel,
            &mut Node
        ),
        Without<SelectionChoicesPanelEntity>
    >,
    builder_res: Res<FamiqWidgetResource>
) {
    for (entity, selector_node, computed_node, panel_entity) in selection_q.iter() {

        let Some(focused) = builder_res.get_widget_focus_state(&entity) else { continue };

        if focused {
            let Ok((_, mut panel_node)) = panel_q.get_mut(panel_entity.0) else { continue };

            let mut top_pos: f32 = 0.0;
            let top_offset: f32 = 6.0;

            if let Some(m_top) = utils::extract_val(selector_node.margin.top) {
                top_pos += m_top;
            }
            if let Some(m_bottom) = utils::extract_val(selector_node.margin.bottom) {
                top_pos += m_bottom;
            }
            top_pos += computed_node.size().y;

            panel_node.top = Val::Px(top_pos + top_offset);
            panel_node.left = selector_node.left;
            panel_node.width = Val::Px(computed_node.size().x);
        }
    }
}

pub fn handle_selection_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selector_q: Query<
        (
            &mut BoxShadow,
            Option<&FamiqWidgetId>,
            &BackgroundColor,
            &DefaultWidgetEntity,
            &SelectorArrowIconEntity,
            &SelectionChoicesPanelEntity,
            &SelectorPlaceHolderEntity
        ),
        Without<IsFamiqSelectionChoicesPanel>
    >,
    mut builder_res: ResMut<FamiqWidgetResource>,
    mut selected_choices_res: ResMut<FaSelectionResource>,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,
    mut placeholder_q: Query<(&mut TextColor, &WidgetStyle), With<SelectorPlaceHolder>>,
    mut panel_q: Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,

) {
    for e in events.read() {
        if e.widget == WidgetType::Selection {
            if let Ok((
                mut box_shadow,
                id,
                bg_color,
                default_style,
                arrow_entity,
                panel_entity,
                placeholder_entity
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
                                FaSelection::set_placeholder_color(
                                    false,
                                    &mut placeholder_q,
                                    placeholder_entity.0,
                                    &bg_color.0
                                );
                                break;
                            }
                        }

                        // currently false, set back to true
                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                        FaSelection::arrow_up(&mut arrow_q, arrow_entity.0);
                        FaSelection::show_choice_panel(&mut panel_q, panel_entity.0);
                        FaSelection::set_placeholder_color(
                            true,
                            &mut placeholder_q,
                            placeholder_entity.0,
                            &bg_color.0
                        );

                        if let Some(id) = id {
                            if !selected_choices_res.exists(id.0.as_str()) {
                                selected_choices_res._update_or_insert(id.0.clone(), "-/-".to_string());
                            }
                        }
                    },
                    _ => {
                        box_shadow.color = Color::NONE;
                    }
                }
            }
        }
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
        &BackgroundColor,
        &mut SelectorPlaceHolderEntity,
        &SelectorArrowIconEntity,
        &SelectionChoicesPanelEntity
    )>,
    mut selected_choices_res: ResMut<FaSelectionResource>,
    mut text_q: Query<&mut Text, Without<ArrowIcon>>,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,
    mut placeholder_q: Query<(&mut TextColor, &WidgetStyle), With<SelectorPlaceHolder>>,
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
                        selection_bg_color,
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
                                selected_choices_res._update_or_insert(id.0.clone(), text.0.clone());
                            }
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
                        FaSelection::set_placeholder_color(
                            false,
                            &mut placeholder_q,
                            placeholder_entity.0,
                            &selection_bg_color.0
                        );

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
