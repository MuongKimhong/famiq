use bevy::prelude::*;
use super::*;

pub(crate) fn on_mouse_over(
    mut over: Trigger<Pointer<Over>>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    mut commands: Commands,
    mut writer: EventWriter<FaMouseEvent>,
    button_q: Query<
        (&GlobalTransform, Option<&TooltipEntity>, Option<&WidgetId>),
        With<IsFamiqButton>
    >,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);

    if let Ok((transform, tooltip_entity, id)) = button_q.get(over.target()) {
        show_tooltip(
            tooltip_entity,
            &mut tooltip_q,
            transform.translation()
        );
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Button, over.target(), id);
    }
    over.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut down: Trigger<Pointer<Pressed>>,
    mut famiq_res: ResMut<FamiqResource>,
    mut button_q: Query<
        (&mut BackgroundColor, &mut ButtonColorBeforePressed, Option<&WidgetId>),
        With<IsFamiqButton>
    >,
    mut writer: EventWriter<FaMouseEvent>
) {
    if let Ok((mut bg_color, mut before_pressed_color, id)) = button_q.get_mut(down.target()) {
        before_pressed_color.0 = Some(bg_color.0);
        famiq_res.update_all_focus_states(false);
        famiq_res.update_or_insert_focus_state(down.target(), true);

        if let Some(darkened_color) = darken_color(20.0, &bg_color.0) {
            bg_color.0 = darkened_color;
        }
        if down.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Button, down.target(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Button, down.target(), id);
        }
    }
    down.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut up: Trigger<Pointer<Released>>,
    mut button_q: Query<(&mut BackgroundColor, &ButtonColorBeforePressed, Option<&WidgetId>), With<IsFamiqButton>>,
    mut writer: EventWriter<FaMouseEvent>
) {
    if let Ok((mut bg_color, before_pressed_color, id)) = button_q.get_mut(up.target()) {
        if let Some(color) = before_pressed_color.0 {
            bg_color.0 = color;
        }
        FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Button, up.target(), id);
    }
    up.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut out: Trigger<Pointer<Out>>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    mut button_q: Query<
        (Option<&TooltipEntity>, Option<&WidgetId>, &mut BackgroundColor, &ButtonColorBeforePressed),
        With<IsFamiqButton>
    >,
    mut commands: Commands,
    mut writer: EventWriter<FaMouseEvent>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>,
) {
    _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);

    if let Ok((tooltip_entity, id, mut bg_color, before_pressed_color)) = button_q.get_mut(out.target()) {
        if let Some(color) = before_pressed_color.0 {
            bg_color.0 = color;
        }
        hide_tooltip(tooltip_entity, &mut tooltip_q);
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Button, out.target(), id);
    }
    out.propagate(false);
}
