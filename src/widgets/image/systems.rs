use bevy::prelude::*;
use super::*;

pub(crate) fn on_mouse_over(
    mut trigger: Trigger<Pointer<Over>>,
    mut writer: EventWriter<FaMouseEvent>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    image_q: Query<(Option<&WidgetId>, &GlobalTransform, Option<&TooltipEntity>), With<IsFamiqImage>>
) {
    if let Ok((id, transform, tooltip_entity)) = image_q.get(trigger.entity()) {
        show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Image, trigger.entity(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut trigger: Trigger<Pointer<Out>>,
    mut writer: EventWriter<FaMouseEvent>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    image_q: Query<(Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqImage>>
) {
    if let Ok((id, tooltip_entity)) = image_q.get(trigger.entity()) {
        hide_tooltip(tooltip_entity, &mut tooltip_q);
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Image, trigger.entity(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut trigger: Trigger<Pointer<Down>>,
    mut writer: EventWriter<FaMouseEvent>,
    image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
) {
    if let Ok(id) = image_q.get(trigger.entity()) {
        if trigger.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Image, trigger.entity(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Image, trigger.entity(), id);
        }
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut trigger: Trigger<Pointer<Up>>,
    mut writer: EventWriter<FaMouseEvent>,
    image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
) {
    if let Ok(id) = image_q.get(trigger.entity()) {
        FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Image, trigger.entity(), id);
    }
    trigger.propagate(false);
}
