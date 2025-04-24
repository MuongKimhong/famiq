use bevy::prelude::*;
use super::*;

pub(crate) fn on_mouse_over(
    mut trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
    trigger.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
    trigger.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut trigger: Trigger<Pointer<Down>>,
    checkbox_q: Query<Option<&ReactiveModelKey>, With<IsFamiqCheckbox>>,
    mut item_box_q: Query<(&mut CheckBoxChoiceTicked, &mut BackgroundColor)>,
    item_wrapper_q: Query<(&CheckBoxItemBoxEntity, &CheckBoxItemText, &CheckBoxMainContainerEntity)>,
    mut fa_query: FaQuery,
) {
    if let Ok((box_entity, item_text, main_entity)) = item_wrapper_q.get(trigger.entity()) {
        if let Ok(model_key) = checkbox_q.get(main_entity.0) {
            if let Some(key) = model_key {
                if let Some(value) = fa_query.get_data_mut(&key.0) {
                    match value {
                        RVal::List(v) => {
                            if let Ok((mut box_ticked, mut bg_color)) = item_box_q.get_mut(box_entity.0) {
                                if v.contains(&item_text.0) {
                                    v.retain(|value| *value != item_text.0);
                                    bg_color.0 = Color::NONE;
                                } else {
                                    v.push(item_text.0.clone());
                                    bg_color.0 = PRIMARY_DARK_COLOR;
                                }
                                box_ticked.0 = !box_ticked.0;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    trigger.propagate(false);
}
