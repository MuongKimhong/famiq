use bevy::prelude::*;
use super::*;

pub(crate) fn on_mouse_over(
    mut trigger: Trigger<Pointer<Over>>,
    mut writer: EventWriter<FaMouseEvent>,
    q: Query<Option<&WidgetId>, With<MainWidget>>
) {
    if let Ok(id) = q.get(trigger.target()) {
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Container, trigger.target(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut trigger: Trigger<Pointer<Out>>,
    mut writer: EventWriter<FaMouseEvent>,
    q: Query<Option<&WidgetId>, With<MainWidget>>
) {
    if let Ok(id) = q.get(trigger.target()) {
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Container, trigger.target(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    mut writer: EventWriter<FaMouseEvent>,
    q: Query<Option<&WidgetId>, With<MainWidget>>
) {
    if let Ok(id) = q.get(trigger.target()) {
        if trigger.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Container, trigger.target(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Container, trigger.target(), id);
        }
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut trigger: Trigger<Pointer<Released>>,
    mut writer: EventWriter<FaMouseEvent>,
    q: Query<Option<&WidgetId>, With<MainWidget>>
) {
    if let Ok(id) = q.get(trigger.target()) {
        FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Container, trigger.target(), id);
    }
    trigger.propagate(false);
}
