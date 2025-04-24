use bevy::prelude::*;
use super::*;

pub(crate) fn on_mouse_over(
    mut trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut writer: EventWriter<FaMouseEvent>,
    text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    if let Ok(id) = text_q.get(trigger.entity()) {
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Text, trigger.entity(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    mut writer: EventWriter<FaMouseEvent>,
    text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    if let Ok(id) = text_q.get(trigger.entity()) {
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Text, trigger.entity(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut trigger: Trigger<Pointer<Down>>,
    mut writer: EventWriter<FaMouseEvent>,
    text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
) {
    if let Ok(id) = text_q.get(trigger.entity()) {
        if trigger.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Text, trigger.entity(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Text, trigger.entity(), id);
        }
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut trigger: Trigger<Pointer<Up>>,
    mut writer: EventWriter<FaMouseEvent>,
    text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
) {
    if let Ok(id) = text_q.get(trigger.entity()) {
        if trigger.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Text, trigger.entity(), id);
        }
    }
    trigger.propagate(false);
}
