pub mod helper;

use crate::widgets::{FamiqWidgetId, DefaultWidgetEntity};
use crate::utils;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use helper::*;

#[derive(Component)]
pub struct IsFamiqModalBackground;

#[derive(Component)]
pub struct IsFamiqModalContainer;

#[derive(Component)]
pub struct FaModalState(pub bool);

#[derive(Component)]
pub struct FaModalContainerEntity(pub Entity);

#[derive(Component)]
pub struct AnimationProgress(pub f32);

pub struct FaModal;

// Doesn't need container
impl<'a> FaModal {
    fn _build_modal_container(id: &str, root_node: &'a mut EntityCommands, items: &Vec<Entity>) -> Entity {
        let node = default_modal_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        let container_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(format!("{id}_modal_container")),
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility
                ),
                IsFamiqModalContainer,
                FocusPolicy::Block,
                AnimationProgress(0.0)
            ))
            .id();

        utils::entity_add_children(root_node, items, container_entity);
        container_entity
    }

    fn _build_modal_background(id: &str, root_node: &'a mut EntityCommands, container_entity: Entity) -> Entity {
        root_node
            .commands()
            .spawn((
                default_modal_background_node(),
                BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                BorderRadius::default(),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ZIndex(20),
                Visibility::Hidden,
                IsFamiqModalBackground,
                FaModalState(false),
                FamiqWidgetId(id.to_string()),
                FocusPolicy::Block,
                FaModalContainerEntity(container_entity)
            ))
            .id()
    }

    pub fn new(id: &str, items: &Vec<Entity>, root_node: &'a mut EntityCommands) -> Entity {
        let container = Self::_build_modal_container(id, root_node, items);
        let background = Self::_build_modal_background(id, root_node, container);

        utils::entity_add_child(root_node, container, background);
        container
    }

    pub fn hide_or_display_modal_system(
        mut modal_bg_q: Query<(&mut Visibility, &FaModalState, &FaModalContainerEntity)>,
        mut modal_container_q: Query<(&mut AnimationProgress, &mut Transform, &IsFamiqModalContainer)>,
        time: Res<Time>
    ) {
        let delta = time.delta_secs() * 6.0;

        for (mut visibility, modal_state, container_entity) in modal_bg_q.iter_mut() {
            if let Ok((mut progress, mut transform, _)) = modal_container_q.get_mut(container_entity.0) {
                if modal_state.0 {
                    *visibility = Visibility::Visible;
                    progress.0 = (progress.0 + delta).min(1.0);
                }
                else {
                    *visibility = Visibility::Hidden;
                    progress.0 = (progress.0 - delta).max(0.0);
                }
                transform.scale = Vec3::splat(progress.0); // Uniform scaling
            }
        }
    }
}
