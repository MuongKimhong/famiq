use bevy::prelude::*;
use super::*;

/// Internal system to detect new progress bars bing created.
pub fn detect_new_progress_bar(
    mut commands: Commands,
    mut progress_materials: ResMut<Assets<ProgressBarMaterial>>,
    mut bar_q: Query<
        (&ComputedNode, &ProgressValueEntity, Option<&ReactiveModelKey>),
        Or<(Added<IsFamiqProgressBar>, Changed<ComputedNode>)>
    >,
    reactive_data: Res<RData>,
    mut value_q: Query<(&mut Node, &ProgressValueColor, &mut ProgressValuePercentage)>,
) {
    bar_q.iter_mut().for_each(|(computed_node, value_entity, model_key)| {
        if let Some(key) = model_key {
            if let Some(r_value) = reactive_data.data.get(&key.0) {
                if let Ok((mut node, value_color, mut percent)) = value_q.get_mut(value_entity.0) {
                    match r_value {
                        RVal::FNum(v) => {
                            percent.0 = Some(v.to_owned());
                            node.width = Val::Percent(v.to_owned());
                        }
                        RVal::None => {
                            percent.0 = None;
                            node.width = Val::Percent(100.0);
                        }
                        _ => {}
                    }
                    if let Color::Srgba(value) = value_color.0 {
                        let u_blend = if percent.0.is_some() {
                            0.0
                        } else {
                            1.0
                        };
                        commands
                            .entity(value_entity.0)
                            .insert(
                                MaterialNode(progress_materials.add(ProgressBarMaterial {
                                    u_time: 0.0,
                                    u_color: Vec3::new(value.red, value.green, value.blue),
                                    u_blend,
                                    u_size: computed_node.size()
                                }))
                            );
                    }
                }
            }
        }
    });
}

pub fn update_progress_bar_material_u_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    query: Query<(&MaterialNode<ProgressBarMaterial>, &ProgressValuePercentage)>
) {
    query.iter().for_each(|(handle, percentage)| {
        if let Some(material) = materials.get_mut(handle) {
            if percentage.0.is_none() {
                material.u_time = -time.elapsed_secs();
                material.u_blend = 1.0;
            } else {
                material.u_time = 0.0;
                material.u_blend = 0.0;
            }
        }
    });
}

pub fn detect_reactive_model_change(
    reactive_data: Res<RData>,
    bar_q: Query<(&ProgressValueEntity, Option<&ReactiveModelKey>)>,
    mut value_q: Query<(&mut ProgressValuePercentage, &mut Node)>
) {
    if reactive_data.is_changed() && !reactive_data.is_added() {
        for (value_entity, model_key) in bar_q.iter() {
            if let Some(key) = model_key {
                if let Some(r_value) = reactive_data.data.get(&key.0) {

                    if let Ok((mut percent, mut node)) = value_q.get_mut(value_entity.0) {
                        match r_value {
                            RVal::FNum(v) => {
                                percent.0 = Some(v.to_owned());
                                node.width = Val::Percent(v.to_owned());
                            }
                            RVal::None => {
                                percent.0 = None;
                                node.width = Val::Percent(100.0);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn on_mouse_over(
    mut trigger: Trigger<Pointer<Over>>,
    mut writer: EventWriter<FaMouseEvent>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    bar_q: Query<(&GlobalTransform, Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqProgressBar>>
) {
    if let Ok((transform, id, tooltip_entity)) = bar_q.get(trigger.entity()) {
        show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
        FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::ProgressBar, trigger.entity(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_out(
    mut trigger: Trigger<Pointer<Out>>,
    mut writer: EventWriter<FaMouseEvent>,
    mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    bar_q: Query<(Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqProgressBar>>
) {
    if let Ok((id, tooltip_entity)) = bar_q.get(trigger.entity()) {
        hide_tooltip(tooltip_entity, &mut tooltip_q);
        FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::ProgressBar, trigger.entity(), id);
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_down(
    mut trigger: Trigger<Pointer<Down>>,
    mut writer: EventWriter<FaMouseEvent>,
    bar_q: Query<Option<&WidgetId>, With<IsFamiqProgressBar>>
) {
    if let Ok(id) = bar_q.get(trigger.entity()) {
        if trigger.event().button == PointerButton::Secondary {
            FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::ProgressBar, trigger.entity(), id);
        } else {
            FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::ProgressBar, trigger.entity(), id);
        }
    }
    trigger.propagate(false);
}

pub(crate) fn on_mouse_up(
    mut trigger: Trigger<Pointer<Up>>,
    mut writer: EventWriter<FaMouseEvent>,
    bar_q: Query<Option<&WidgetId>, With<IsFamiqProgressBar>>
) {
    if let Ok(id) = bar_q.get(trigger.entity()) {
        FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::ProgressBar, trigger.entity(), id);
    }
    trigger.propagate(false);
}
