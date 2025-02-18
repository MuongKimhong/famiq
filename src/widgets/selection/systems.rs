use crate::widgets::selection::*;
use crate::widgets::{WidgetType, FamiqResource};
use crate::plugin::{CursorType, CursorIcons};
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
    mut panel_q: Query<(&mut Node, &mut Visibility), With<IsFamiqSelectionChoicesPanel>>,
    builder_res: Res<FamiqResource>
) {
    if !builder_res.is_changed() {
        return;
    }
    for (entity, computed_node, transform, panel_entity) in selection_q.iter() {
        let Some(focused) = builder_res.get_widget_focus_state(&entity) else { continue };
        let Ok((mut panel_node, mut panel_visibility)) = panel_q.get_mut(panel_entity.0) else { continue };

        if focused {
            *panel_visibility = Visibility::Visible;
            _set_choice_panel_position_and_width(
                &transform.translation(),
                computed_node,
                &mut panel_node
            );
        }
        else {
            *panel_visibility = Visibility::Hidden;
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
        ),
        Without<IsFamiqSelectionChoicesPanel>
    >,
    mut builder_res: ResMut<FamiqResource>,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,

    window: Single<Entity, With<Window>>,
    mut commands: Commands,
    cursor_icons: Res<CursorIcons>,
) {
    for e in events.read() {
        if e.widget == WidgetType::Selection {
            if let Ok((
                mut box_shadow,
                default_style,
                arrow_entity,
            )) = selector_q.get_mut(e.entity)
            {
                match e.interaction {
                    Interaction::Hovered => {
                        box_shadow.color = default_style.border_color.0.clone();
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                    },
                    Interaction::Pressed => {
                        // currently true, set back to false
                        if let Some(state) = builder_res.get_widget_focus_state(&e.entity) {
                            if state {
                                builder_res.update_or_insert_focus_state(e.entity, false);
                                FaSelection::arrow_down(&mut arrow_q, arrow_entity.0);
                                break;
                            }
                        }

                        // currently false, set back to true
                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                        FaSelection::arrow_up(&mut arrow_q, arrow_entity.0);
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                    },
                    _ => {
                        box_shadow.color = Color::NONE;
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
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
            Option<&FamiqWidgetId>
        ),
        Added<IsFamiqSelectionSelector>
    >,
    mut selection_res: ResMut<FaSelectionResource>
) {
    for (entity, id) in selection_q.iter() {
        if let Some(id) = id {
            if !selection_res.exists_by_id(id.0.as_str()) {
                selection_res._insert_by_id(id.0.clone(), String::new());
            }
        }

        if !selection_res.exists_by_entity(entity) {
            selection_res._insert_by_entity(entity, String::new());
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
        &mut SelectorPlaceHolderEntity,
        &SelectorArrowIconEntity,
    )>,
    mut selection_res: ResMut<FaSelectionResource>,
    mut text_q: Query<&mut Text, Without<ArrowIcon>>,
    mut arrow_q: Query<&mut Text, With<ArrowIcon>>,
    mut builder_res: ResMut<FamiqResource>,

    window: Single<Entity, With<Window>>,
    mut commands: Commands,
    cursor_icons: Res<CursorIcons>,
) {
    for e in events.read() {
        if e.widget == WidgetType::SelectionChoice {
            if let Ok((
                mut choice_bg_color,
                choice_txt_entity,
                selector_entity
            )) = selection_choice_q.get_mut(e.entity) {


                match e.interaction {
                    Interaction::Hovered => {
                        *choice_bg_color = ITEM_ON_HOVER_BG_COLOR.into();
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                    },
                    Interaction::Pressed => {
                        if let Ok((
                            selection_entity,
                            selection,
                            selection_id,
                            placeholder_entity,
                            arrow_entity,
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
                            *choice_bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR);
                        }
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
                    },
                    _ => {
                        *choice_bg_color = ITEM_NORMAL_BG_COLOR.into();
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
                    }
                }
            }
        }
    }
}
