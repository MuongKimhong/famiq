use crate::widgets::selection::*;
use crate::widgets::color::*;
use crate::widgets::{WidgetType, FamiqWidgetResource};
use crate::event_writer::FaInteractionEvent;
use crate::utils;
use bevy::prelude::*;

pub fn update_selector_placeholder_color_system(
    selection_q: Query<(Entity, &BackgroundColor, &SelectorPlaceHolderEntity)>,
    mut text_q: Query<&mut TextColor>,
    builder_res: Res<FamiqWidgetResource>
) {
    for (entity, bg_color, placeholder) in selection_q.iter() {
        if let Ok(mut text_color) = text_q.get_mut(placeholder.0) {
            match builder_res.get_widget_focus_state(&entity) {
                Some(true) => {
                    if bg_color.0 == WHITE_COLOR {
                        text_color.0 = BLACK_COLOR
                    } else {
                        text_color.0 = PLACEHOLDER_COLOR_FOCUSED;
                    }
                },
                _ => text_color.0 = PLACEHOLDER_COLOR_UNFOCUSED
            }
        }
    }
}

// on focus use arrow up else use arrow down
pub fn update_selector_arrow_icon_system(
    selection_q: Query<(Entity, &SelectorArrowIconEntity)>,
    mut text_q: Query<&mut Text>,
    builder_res: Res<FamiqWidgetResource>
) {
    for (entity, arrow_icon_entity) in selection_q.iter() {
        if let Ok(mut text) = text_q.get_mut(arrow_icon_entity.0) {
            match builder_res.get_widget_focus_state(&entity) {
                Some(true) =>  text.0 = "▲".to_string(),
                _ => text.0 = "▼".to_string()
            }
        }
    }
}

// on focus show panel else hide
pub fn update_selection_choices_panel_visibility_system(
    selection_q: Query<(Entity, &SelectionChoicesPanelEntity)>,
    mut visibility_q: Query<(&mut Visibility, &mut DefaultWidgetEntity, &IsFamiqSelectionChoicesPanel)>,
    builder_res: Res<FamiqWidgetResource>
) {
    for (entity, choices_panel_entity) in selection_q.iter() {
        if let Ok((mut visibility, mut default_widget, _)) = visibility_q.get_mut(choices_panel_entity.0) {
            match builder_res.get_widget_focus_state(&entity) {
                Some(true) => {
                    *visibility = Visibility::Visible;
                    default_widget.visibility = Visibility::Visible;
                },
                _ => {
                    *visibility = Visibility::Hidden;
                    default_widget.visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn update_choices_panel_position_and_width_system(
    selection_q: Query<
        (
            Entity,
            &Node,
            &ComputedNode,
            &SelectionChoicesPanelEntity,
            Option<&SelectionLabelEntity>
        )
    >,
    mut panel_q: Query<
        (
            &IsFamiqSelectionChoicesPanel,
            &mut Node,
            &mut DefaultWidgetEntity,
        ),
        Without<SelectionChoicesPanelEntity>
    >,
    label_q: Query<(&Node, &IsFamiqSelectionLabel), Without<IsFamiqSelectionChoicesPanel>>,
    builder_res: Res<FamiqWidgetResource>
) {
    for (entity, selector_node, computed_node, panel_entity, label_entity) in selection_q.iter() {
        match builder_res.get_widget_focus_state(&entity) {
            Some(true) => {
                if let Ok((_, mut panel_node, mut default_widget)) = panel_q.get_mut(panel_entity.0) {
                    let mut top_pos: f32 = 0.0;
                    let top_offset: f32 = 6.0;

                    if let Some(label_entity) = label_entity {
                        if let Ok((label_node, _)) = label_q.get(label_entity.0) {
                            if let Some(label_height) = utils::extract_val(label_node.height) {
                                top_pos += label_height;
                            }
                        }
                    }
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

                    default_widget.node.top = Val::Px(top_pos + top_offset);
                    default_widget.node.left = selector_node.left;
                    default_widget.node.width = Val::Px(computed_node.size().x);
                }
            },
            _ => {}
        }
    }
}

pub fn handle_selection_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selector_q: Query<(&mut BoxShadow, Option<&FamiqWidgetId>, &DefaultWidgetEntity), With<Selection>>,
    mut builder_res: ResMut<FamiqWidgetResource>,
    mut selected_choices_res: ResMut<SelectedChoicesResource>,
) {
    for e in events.read() {
        if e.widget == WidgetType::Selection {
            if let Ok((mut box_shadow, id, default_style)) = selector_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        box_shadow.color = default_style.border_color.0.clone();
                    },
                    Interaction::Pressed => {
                        if let Some(state) = builder_res.get_widget_focus_state(&e.entity) {
                            if state {
                                builder_res.update_or_insert_focus_state(e.entity, false);
                                break;
                            }
                        }

                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);

                        if let Some(id) = id {
                            selected_choices_res.update_or_insert(id.0.clone(), "-/-".to_string());
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
            &mut DefaultWidgetEntity,
            &mut BackgroundColor,
            &SelectionChoiceTextEntity,

        ),
        With<IsFamiqSelectionChoice>
    >,
    mut selection_q: Query<(Entity, &Selection, Option<&FamiqWidgetId>, &mut SelectorPlaceHolderEntity)>,
    mut selected_choices_res: ResMut<SelectedChoicesResource>,
    mut text_q: Query<&mut Text>,
    mut builder_res: ResMut<FamiqWidgetResource>
) {
    for e in events.read() {
        if e.widget == WidgetType::SelectionChoice {
            let mut selected_choice = String::new();

            for (selection_entity, selection, selection_id, placeholder_entity) in selection_q.iter_mut() {
                match builder_res.get_widget_focus_state(&selection_entity) {
                    Some(true) => {
                        if let Ok((mut default_choice_widget, mut bg_color, choice_text_entity)) = selection_choice_q.get_mut(e.entity) {
                            match e.interaction {
                                Interaction::Hovered => {
                                    *bg_color = BackgroundColor(ITEM_ON_HOVER_BG_COLOR);
                                    default_choice_widget.background_color = BackgroundColor(ITEM_ON_HOVER_BG_COLOR);
                                }
                                Interaction::Pressed => {
                                    // Update selected items resource
                                    if let Ok(text) = text_q.get_mut(choice_text_entity.0) {
                                        selected_choice = if text.0 == "-/-" {
                                            String::from("")
                                        } else {
                                            text.0.clone()
                                        };
                                        if let Some(id) = selection_id {
                                            selected_choices_res.update_or_insert(id.0.clone(), text.0.clone());
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
                                    // selection.focused = false;
                                    builder_res.update_or_insert_focus_state(selection_entity, false);

                                    *bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                                    default_choice_widget.background_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                                }
                                _ => {
                                    *bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                                    default_choice_widget.background_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}
