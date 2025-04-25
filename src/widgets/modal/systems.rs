use bevy::prelude::*;
use super::*;

pub fn detect_modal_reactive_model_change(
    reactive_data: Res<RData>,
    modal_q: Query<(Entity, Option<&ReactiveModelKey>), With<IsFamiqModal>>,
    mut modal_state: ResMut<FaModalState>,
) {
    if reactive_data.is_changed() && !reactive_data.is_added() {
        for (entity, model_key) in modal_q.iter() {
            if let Some(key) = model_key {
                if let Some(r_value) = reactive_data.data.get(&key.0) {
                    match r_value {
                        RVal::Bool(state) => {
                            if *state {
                                modal_state.show_by_entity(entity);
                            } else {
                                modal_state.hide_by_entity(entity);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// Internal system to hide or display via `FaModalState` resource.
pub fn hide_or_display_modal_system(
    mut modal_bg_q: Query<(&mut Node, &mut AnimationProgress, &Children, Entity)>,
    mut children_q: Query<&mut Transform>,
    mut modal_res: ResMut<FaModalState>,
    time: Res<Time>,
) {
    if modal_res.state_changed {
        let delta = time.delta_secs() * 7.0;

        for (mut node, mut progress, children, entity) in modal_bg_q.iter_mut() {
            for child in children.iter() {
                if let Ok(mut transform) = children_q.get_mut(child) {
                    let is_visible = modal_res
                        .get_state_by_entity(entity)
                        .copied()
                        .unwrap_or(false);

                    let old_progress = progress.0;

                    if is_visible {
                        node.display = Display::default();
                        progress.0 = (progress.0 + delta).min(1.0);
                    } else {
                        progress.0 = (progress.0 - delta).max(0.0);
                    }
                    transform.scale = Vec3::splat(progress.0); // Uniform scaling

                    if old_progress != progress.0 && (progress.0 == 1.0 || progress.0 == 0.0) {
                        modal_res.state_changed = false;

                        if progress.0 == 0.0 {
                            node.display = Display::None;
                        }
                    }
                }
            }
        }
    }
}
