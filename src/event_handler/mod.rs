use crate::event_writer::*;
use crate::widgets::{selection::*, text_input::*, FamiqWidgetId, WidgetType};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use smol_str::SmolStr;

pub fn handle_text_input_on_click_system(
    mut events: EventReader<FaInteractionEvent>,
    mut input_query: Query<(&Children, &mut TextInput, &FamiqWidgetId)>,
    mut text_query: Query<&mut Text>,
    mut input_resource: ResMut<TextInputResource>,
) {
    for e in events.read() {
        if e.interaction_type == Interaction::Pressed {
            FaTextInput::set_unfocused_all(&mut input_query, &mut text_query, &mut input_resource);
            FaTextInput::set_focus(
                &mut input_query,
                e.entity,
                &mut text_query,
                &mut input_resource,
            );
        }
    }
}

pub fn handle_text_input_on_typing_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut input_query: Query<(&Children, &mut TextInput, &FamiqWidgetId)>,
    mut text_query: Query<&mut Text>,
    mut input_resource: ResMut<TextInputResource>,
) {
    for e in evr_kbd.read() {
        if e.state == ButtonState::Released {
            continue;
        }
        match &e.logical_key {
            Key::Character(input) => {
                FaTextInput::add_text(
                    &mut input_query,
                    &mut text_query,
                    input,
                    &mut input_resource,
                );
            }
            Key::Space => {
                FaTextInput::add_text(
                    &mut input_query,
                    &mut text_query,
                    &SmolStr::new(" "),
                    &mut input_resource,
                );
            }
            Key::Backspace => {
                FaTextInput::delete_text(&mut input_query, &mut text_query, &mut input_resource);
            }
            _ => {
                continue;
            }
        }
    }
}

pub fn handle_selection_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selection_q: Query<&mut Selection>,
) {
    for e in events.read() {
        if e.widget_type == WidgetType::Selection {
            match e.interaction_type {
                Interaction::Pressed => {
                    if let Ok(selection) = selection_q.get_mut(e.entity) {
                        if selection.focused {
                            FaSelection::set_to_unfocus(&mut selection_q, e.entity, false);
                            break;
                        }
                    }
                    FaSelection::set_to_unfocus(&mut selection_q, e.entity, true);
                    FaSelection::set_to_focus(&mut selection_q, e.entity);
                }
                _ => (),
            }
            break;
        }
    }
}

pub fn handle_selection_item_interaction_system(
    mut events: EventReader<FaInteractionEvent>,
    mut selection_q: Query<(
        &mut Selection,
        &FamiqWidgetId,
        Entity,
        &mut SelectorPlaceHolderEntity,
    )>,
    mut selection_items_q: Query<(
        &mut Style,
        &mut BackgroundColor,
        &IsFamiqSelectionItem,
        &SelectionItemTextEntity,
    )>,
    mut selected_items_res: ResMut<SelectedItemsResource>,
    mut text_q: Query<&mut Text>,
) {
    for e in events.read() {
        if e.widget_type == WidgetType::SelectionItem {
            let mut focused_selection_id = String::new();
            let mut focused_selection_entity = None;
            let mut focused_selection_placeholder_entity = None;
            let mut selected_item_text = String::new();

            // Immutable iteration to find the focused selection
            for (selection, selection_id, selection_entity, placeholder_entity) in
                selection_q.iter()
            {
                if selection.focused {
                    focused_selection_id = selection_id.0.clone();
                    focused_selection_entity = Some(selection_entity);
                    focused_selection_placeholder_entity = Some(placeholder_entity.clone());
                    break;
                }
            }

            if let Ok((mut _style, mut bg_color, _, item_text_entity)) =
                selection_items_q.get_mut(e.entity)
            {
                match e.interaction_type {
                    Interaction::Hovered => {
                        *bg_color = BackgroundColor(ITEM_ON_HOVER_BG_COLOR);
                    }
                    Interaction::Pressed => {
                        // Update selected items resource
                        if let Ok(text) = text_q.get_mut(item_text_entity.0) {
                            selected_items_res.update_or_insert(
                                focused_selection_id,
                                text.sections[0].value.clone(),
                            );
                            selected_item_text = text.sections[0].value.clone();
                        }

                        // Set to unfocus as already selected
                        if let Some(selection_entity) = focused_selection_entity {
                            if let Ok((mut selection, _, _, _)) =
                                selection_q.get_mut(selection_entity)
                            {
                                selection.focused = false;
                            }
                        }

                        // Update placeholder
                        if let Some(placeholder_entity) = focused_selection_placeholder_entity {
                            if let Ok(mut placeholder_text) = text_q.get_mut(placeholder_entity.0) {
                                placeholder_text.sections[0].value = selected_item_text;
                            }
                        }
                    }
                    _ => *bg_color = BackgroundColor(ITEM_NORMAL_BG_COLOR),
                }
                break;
            }
        }
    }
}
