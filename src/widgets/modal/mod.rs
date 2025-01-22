pub mod helper;

use crate::widgets::{FamiqWidgetId, FamiqWidgetClasses, DefaultWidgetEntity, FamiqWidgetBuilder};
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

    fn _build_modal_background(
        id: &str,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        container_entity: Entity
    ) -> Entity {
        let entity = root_node
            .commands()
            .spawn((
                default_modal_background_node(),
                BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                BorderRadius::default(),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ZIndex::default(),
                GlobalZIndex(5),
                Visibility::Hidden,
                IsFamiqModalBackground,
                FaModalState(false),
                FamiqWidgetId(id.to_string()),
                FocusPolicy::Block,
                FaModalContainerEntity(container_entity)
            ))
            .id();

        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        root_node.add_child(entity);
        entity
    }

    pub fn new(
        id: &str,
        class: Option<String>,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let container = Self::_build_modal_container(id, root_node, items);
        let background = Self::_build_modal_background(id, class, root_node, container);

        utils::entity_add_child(root_node, container, background);
        container
    }

    pub fn hide_or_display_modal_system(
        mut modal_bg_q: Query<(&mut Visibility, &FaModalState, &FaModalContainerEntity)>,
        mut modal_container_q: Query<(&mut AnimationProgress, &mut Transform), With<IsFamiqModalContainer>>,
        time: Res<Time>
    ) {
        let delta = time.delta_secs() * 6.0;

        for (mut visibility, modal_state, container_entity) in modal_bg_q.iter_mut() {
            if let Ok((mut progress, mut transform)) = modal_container_q.get_mut(container_entity.0) {
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

pub struct FaModalBuilder<'a> {
    pub id: String,
    pub class: Option<String>,
    pub children: Option<Vec<Entity>>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaModalBuilder<'a> {
    pub fn new(id: String, root_node: EntityCommands<'a>) -> Self {
        Self {
            id,
            class: None,
            children: Some(Vec::new()),
            root_node
        }
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn children(mut self, children: Vec<Entity>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaModal::new(
            self.id.as_str(),
            self.class.clone(),
            self.children.as_ref().unwrap(),
            &mut self.root_node
        )
    }
}

pub fn fa_modal<'a>(builder: &'a mut FamiqWidgetBuilder, id: &str) -> FaModalBuilder<'a> {
    FaModalBuilder::new(
        id.to_string(),
        builder.ui_root_node.reborrow()
    )
}
