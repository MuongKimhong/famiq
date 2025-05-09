use bevy::prelude::*;
use super::*;

pub fn detect_dialog_reactive_model_change(
    reactive_data: Res<RData>,
    dialog_q: Query<(Entity, Option<&ReactiveModelKey>), With<IsFamiqDialog>>,
    mut dialog_state: ResMut<FaDialogState>,
) {
    if reactive_data.is_changed() {
        for (entity, model_key) in dialog_q.iter() {
            if model_key.is_none() {
                continue;
            }
            let key = model_key.unwrap();
            let r_value = reactive_data.data.get(&key.0);

            if r_value.is_none() {
                continue;
            }
            match r_value.unwrap() {
                RVal::Bool(state) => {
                    if *state {
                        dialog_state.show_by_entity(entity);
                    } else {
                        dialog_state.hide_by_entity(entity);
                    }
                }
                _ => {}
            }
        }
    }
}

// Internal system to hide or display via `FaModalState` resource.
pub fn hide_or_display_dialog_system(
    mut dialog_q: Query<(&mut Node, &mut AnimationProgress, &Children, Entity)>,
    mut children_q: Query<&mut Transform>,
    mut dialog_res: ResMut<FaDialogState>,
    time: Res<Time>,
) {
    if dialog_res.state_changed {
        let delta = time.delta_secs() * 7.0;

        for (mut node, mut progress, children, entity) in dialog_q.iter_mut() {
            for child in children.iter() {
                if let Ok(mut transform) = children_q.get_mut(child) {
                    let is_visible = dialog_res
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
                        dialog_res.state_changed = false;

                        if progress.0 == 0.0 {
                            node.display = Display::None;
                        }
                    }
                }
            }
        }
    }
}
