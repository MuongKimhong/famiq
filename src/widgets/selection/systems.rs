use crate::widgets::FamiqResource;
use super::*;
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

pub(crate) fn on_mouse_over(
    mut trigger: Trigger<Pointer<Over>>,
    mut selector_q: Query<
        (&mut BoxShadow, &BorderColor, Option<&WidgetId>, &GlobalTransform, Option<&TooltipEntity>),
        With<IsFamiqSelectionSelector>
    >,
    mut commands: Commands,
    mut writer: EventWriter<FaMouseEvent>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    if let Ok((mut box_shadow, border_color, id, transform, tooltip_entity)) = selector_q.get_mut(trigger.target()) {
        box_shadow.0[0].color = border_color.0.clone();
        show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Selection, trigger.target(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut trigger: Trigger<Pointer<Out>>,
    mut selector_q: Query<
        (&mut BoxShadow, Option<&WidgetId>, Option<&TooltipEntity>),
        With<IsFamiqSelectionSelector>
    >,
    mut commands: Commands,
    mut writer: EventWriter<FaMouseEvent>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    if let Ok((mut box_shadow, id, tooltip_entity)) = selector_q.get_mut(trigger.target()) {
        box_shadow.0[0].color = Color::NONE;
        hide_tooltip(tooltip_entity, &mut tooltip_q);
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Selection, trigger.target(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
    mut writer: EventWriter<FaMouseEvent>,
    mut famiq_res: ResMut<FamiqResource>
) {
    if let Ok(id) = selector_q.get_mut(trigger.target()) {
        // currently true, set back to false
        if let Some(state) = famiq_res.get_widget_focus_state(&trigger.target()) {
            if state {
                famiq_res.update_or_insert_focus_state(trigger.target(), false);
                return;
            }
        }
        // currently false, set back to true
        famiq_res.update_all_focus_states(false);
        famiq_res.update_or_insert_focus_state(trigger.target(), true);

        if trigger.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Selection, trigger.target(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Selection, trigger.target(), id);
        }
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut trigger: Trigger<Pointer<Released>>,
    mut selector_q: Query<Option<&WidgetId>, With<IsFamiqSelectionSelector>>,
    mut writer: EventWriter<FaMouseEvent>,
) {
    if let Ok(id) = selector_q.get_mut(trigger.target()) {
        FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Selection, trigger.target(), id);
    }
    trigger.propagate(false);
}
