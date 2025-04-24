use bevy::prelude::*;
use super::*;

pub(crate) fn on_mouse_over(
    mut over: Trigger<Pointer<Over>>,
    circular_q: Query<(&GlobalTransform, Option<&TooltipEntity>, Option<&WidgetId>), With<IsFamiqCircular>>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    mut writer: EventWriter<FaMouseEvent>
) {
    if let Ok((transform, tooltip_entity, id)) = circular_q.get(over.entity()) {
        show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Circular, over.entity(), id);
    }
    over.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut out: Trigger<Pointer<Out>>,
    mut circular_q: Query<(Option<&TooltipEntity>, Option<&WidgetId>), With<IsFamiqCircular>>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    mut writer: EventWriter<FaMouseEvent>
) {
    if let Ok((tooltip_entity, id)) = circular_q.get_mut(out.entity()) {
        hide_tooltip(tooltip_entity, &mut tooltip_q);
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Circular, out.entity(), id);
    }
    out.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut down: Trigger<Pointer<Down>>,
    mut circular_q: Query<Option<&WidgetId>, With<IsFamiqCircular>>,
    mut writer: EventWriter<FaMouseEvent>
) {
    if let Ok(id) = circular_q.get_mut(down.entity()) {
        if down.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Circular, down.entity(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Circular, down.entity(), id);
        }
    }
    down.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut up: Trigger<Pointer<Up>>,
    mut circular_q: Query<Option<&WidgetId>, With<IsFamiqCircular>>,
    mut writer: EventWriter<FaMouseEvent>
) {
    if let Ok(id) = circular_q.get_mut(up.entity()) {
        FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Circular, up.entity(), id);
    }
    up.propagate(false);
}

/// Internal system to detect new circular bing created.
pub fn detect_new_circular(
    mut commands: Commands,
    mut circular_material: ResMut<Assets<CircularMaterial>>,
    circular_q: Query<(Entity, &SpinnerColor), Added<IsFamiqCircular>>,
) {
    circular_q.iter().for_each(|(entity, color)| {
        if let Color::Srgba(value) = color.0 {
            commands
                .entity(entity)
                .insert(
                    MaterialNode(circular_material.add(CircularMaterial {
                        u_time: 0.0,
                        u_color: Vec3::new(value.red, value.green, value.blue)
                    }))
                );
        }
    });
}

pub fn update_circular_material_u_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<CircularMaterial>>,
    query: Query<&MaterialNode<CircularMaterial>>
) {
    query.iter().for_each(|handle| {
        if let Some(material) = materials.get_mut(handle) {
            material.u_time = -time.elapsed_secs();
        }
    });
}
