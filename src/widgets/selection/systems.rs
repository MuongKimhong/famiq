use crate::widgets::selection::*;
use crate::widgets::color::*;
use crate::widgets::{WidgetType, FamiqWidgetBuilderResource};
use crate::event_writer::FaInteractionEvent;
use crate::utils;
use bevy::prelude::*;

pub fn update_selector_placeholder_color_system(
    selection_q: Query<(&Selection, &BackgroundColor, &SelectorPlaceHolderEntity), Changed<Selection>>,
    mut text_q: Query<&mut TextColor>,
) {
    for (selection, bg_color, placeholder) in selection_q.iter() {
        if let Ok(mut text_color) = text_q.get_mut(placeholder.0) {
            if selection.focused {
                if bg_color.0 == WHITE_COLOR {
                    text_color.0 = BLACK_COLOR
                }
                else {
                    text_color.0 = PLACEHOLDER_COLOR_FOCUSED;
                }
            }
            else {
                text_color.0 = PLACEHOLDER_COLOR_UNFOCUSED;
            }
        }
    }
}

// on focus use arrow up else use arrow down
pub fn update_selector_arrow_icon_system(
    selection_q: Query<(&Selection, &SelectorArrowIconEntity), Changed<Selection>>,
    mut text_q: Query<&mut Text>,
) {
    for (selection, arrow_icon_entity) in selection_q.iter() {
        if let Ok(mut text) = text_q.get_mut(arrow_icon_entity.0) {
            text.0 = if selection.focused {
                "▲".to_string()
            } else {
                "▼".to_string()
            };
        }
    }
}

// on focus show panel else hide
pub fn update_selection_choices_panel_visibility_system(
    selection_q: Query<(&Selection, &SelectionChoicesPanelEntity), Changed<Selection>>,
    mut visibility_q: Query<(&mut Visibility, &mut DefaultWidgetEntity, &IsFamiqSelectionChoicesPanel)>,
) {
    for (selection, choices_panel_entity) in selection_q.iter() {
        if let Ok((mut visibility, mut default_widget, _)) = visibility_q.get_mut(choices_panel_entity.0) {
            if selection.focused {
                *visibility = Visibility::Visible;
                default_widget.visibility = Visibility::Visible;
            }
            else {
                *visibility = Visibility::Hidden;
                default_widget.visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn update_choices_panel_position_and_width_system(
    selection_q: Query<
        (
            &Selection,
            &Node,
            &ComputedNode,
            &SelectionChoicesPanelEntity,
            Option<&SelectionLabelEntity>
        ),
        Changed<Selection>
    >,
    mut panel_q: Query<
        (
            &IsFamiqSelectionChoicesPanel,
            &mut Node,
            &mut DefaultWidgetEntity,
        ),
        Without<Selection>
    >,
    label_q: Query<
        (&Node, &IsFamiqSelectionLabel),
        Without<IsFamiqSelectionChoicesPanel>
    >
) {
    for (selection, selector_node, computed_node, panel_entity, label_entity) in selection_q.iter() {
        if selection.focused {
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
        }
    }
}

pub fn handle_selection_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selection_q: Query<(&mut Selection, &FamiqWidgetId)>,
    mut builder_res: ResMut<FamiqWidgetBuilderResource>
) {
    for e in events.read() {
        if e.widget == WidgetType::Selection && e.interaction == Interaction::Pressed {
            for (mut selection, id) in selection_q.iter_mut() {
                if e.widget_id == id.0 {
                    selection.focused = !selection.focused;

                    // global focus
                    builder_res.update_all_focus_states(false);
                    builder_res.update_or_insert_focus_state(e.entity, true);
                }
                else {
                    selection.focused = false;
                }
            }
        }
    }
}

pub fn handle_selection_choice_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selection_choice_q: Query<(
        &mut DefaultWidgetEntity,
        &mut BackgroundColor,
        &SelectionChoiceTextEntity,
        &IsFamiqSelectionChoice
    )>,
    mut selection_q: Query<(&mut Selection, &FamiqWidgetId, &mut SelectorPlaceHolderEntity)>,
    mut selected_choices_res: ResMut<SelectedChoicesResource>,
    mut text_q: Query<&mut Text>,
) {
    for e in events.read() {
        if e.widget == WidgetType::SelectionChoice {
            let mut selected_choice = String::new();

            for (mut selection, selection_id, placeholder_entity) in selection_q.iter_mut() {
                if selection.focused {

                    if let Ok((mut default_choice_widget, mut bg_color, choice_text_entity, _)) = selection_choice_q.get_mut(e.entity) {
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
                                    selected_choices_res.update_or_insert(selection_id.0.clone(), text.0.clone());
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
                                selection.focused = false;

                                *bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                                default_choice_widget.background_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                            }
                            _ => {
                                *bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                                default_choice_widget.background_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_selector_on_hover_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selector_q: Query<(&mut BoxShadow, &DefaultWidgetEntity), With<Selection>>
) {
    for e in events.read() {
        if e.widget == WidgetType::Selection {
            if let Ok((mut box_shadow, default_style)) = selector_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        box_shadow.color = default_style.border_color.0.clone();
                    },
                    Interaction::None => {
                        box_shadow.color = Color::NONE;
                    },
                    _ => {}
                }
            }
        }
    }
}
