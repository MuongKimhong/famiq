pub mod helper;
pub mod tests;

use crate::widgets::{
    FamiqWidgetId, DefaultWidgetEntity,
    FamiqBuilder, BaseStyleComponents
};
use crate::utils;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;
use helper::*;

/// Marker component for identifying the modal background.
#[derive(Component)]
pub struct IsFamiqModalBackground;

/// Marker component for identifying the modal container that hold all the items provided.
#[derive(Component)]
pub struct IsFamiqModalContainer;

/// Component associating a modal background with its container entity.
#[derive(Component)]
pub struct FaModalContainerEntity(pub Entity);

/// Component that keep tracking of modal show/hide animation.
#[derive(Component)]
pub struct AnimationProgress(pub f32);

/// Use to define show/hide state for modal
/// by id or entity.
#[derive(Resource, Default, Debug)]
pub struct FaModalState {
    pub id_states: HashMap<String, bool>,
    pub entity_states: HashMap<Entity, bool>,
    pub state_changed: bool
}

impl FaModalState {
    /// Private: Updates or inserts an ID state
    fn _update_or_insert_id(&mut self, id: &str, new_state: bool) {
        self.id_states.entry(id.to_string()).or_insert(false);
        self.id_states.insert(id.to_string(), new_state);
    }

    /// Private: Updates or inserts an Entity state
    fn _update_or_insert_entity(&mut self, entity: Entity, new_state: bool) {
        self.entity_states.entry(entity).or_insert(false);
        self.entity_states.insert(entity, new_state);
    }

    /// Private: Set all modal states to false
    fn _hide_all(&mut self) {
        self.id_states.values_mut().for_each(|v| *v = false);
        self.entity_states.values_mut().for_each(|v| *v = false);
    }

    /// Show modal by ID (Only one can be `true`)
    pub fn show_by_id(&mut self, id: &str) {
        self._hide_all();
        self._update_or_insert_id(id, true);
        self.state_changed = true;
    }

    /// Show modal by Entity (Only one can be `true`)
    pub fn show_by_entity(&mut self, entity: Entity) {
        self._hide_all();
        self._update_or_insert_entity(entity, true);
        self.state_changed = true;
    }

    /// Hide modal by ID
    pub fn hide_by_id(&mut self, id: &str) {
        self._update_or_insert_id(id, false);
        self.state_changed = true;
    }

    /// Hide modal by Entity
    pub fn hide_by_entity(&mut self, entity: Entity) {
        self._update_or_insert_entity(entity, false);
        self.state_changed = true;
    }

    pub fn get_state_by_id(&self, id: &str) -> Option<&bool> {
        self.id_states.get(id)
    }

    pub fn get_state_by_entity(&self, entity: Entity) -> Option<&bool> {
        self.entity_states.get(&entity)
    }
}

pub struct FaModal;

// Doesn't need container
impl<'a> FaModal {
    fn _build_modal_container(
        root_node: &'a mut EntityCommands,
        items: &Vec<Entity>
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_modal_container_node();

        let container_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                DefaultWidgetEntity::from(style_components),
                IsFamiqModalContainer,
                FocusPolicy::Block,
                AnimationProgress(0.0)
            ))
            .id();

        utils::entity_add_children(root_node, items, container_entity);
        container_entity
    }

    fn _build_modal_background(
        id: Option<String>,
        class: Option<String>,
        clear_bg: bool,
        root_node: &'a mut EntityCommands,
        container_entity: Entity
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_modal_background_node();
        style_components.visibility = Visibility::Hidden;
        style_components.border_color = BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.6));

        if !clear_bg {
            style_components.background_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6));
        }

        let entity = root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqModalBackground,
                FocusPolicy::Block,
                FaModalContainerEntity(container_entity),
                GlobalZIndex(5)
            ))
            .id();

        utils::insert_id_and_class(root_node, entity, &id, &class);
        root_node.add_child(entity);
        entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        clear_bg: bool,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let container = Self::_build_modal_container(root_node, items);
        let background = Self::_build_modal_background(id, class, clear_bg, root_node, container);

        utils::entity_add_child(root_node, container, background);
        background
    }

    pub fn detect_new_modal_system(
        mut modal_res: ResMut<FaModalState>,
        mut modal_container_q: Query<&mut Transform, With<IsFamiqModalContainer>>,
        modal_bg_q: Query<
            (Entity, Option<&FamiqWidgetId>, &FaModalContainerEntity),
            Added<IsFamiqModalBackground>
        >,
    ) {
        for (entity, id, container_entity) in modal_bg_q.iter() {
            if let Some(id) = id {
                modal_res._update_or_insert_id(&id.0, false);
            }
            modal_res._update_or_insert_entity(entity, false);

            if let Ok(mut transform) = modal_container_q.get_mut(container_entity.0) {
                transform.scale = Vec3::splat(0.0);
            }
        }
    }

    /// Internal system to hide or display via `FaModalState` resource.
    pub fn hide_or_display_modal_system(
        mut modal_bg_q: Query<(&mut Visibility, Entity, &FamiqWidgetId, &FaModalContainerEntity)>,
        mut modal_container_q: Query<(&mut AnimationProgress, &mut Transform), With<IsFamiqModalContainer>>,
        time: Res<Time>,
        mut modal_res: ResMut<FaModalState>,
    ) {
        if modal_res.state_changed {
            let delta = time.delta_secs() * 6.0;

            for (mut visibility, modal_entity, modal_id, container_entity) in modal_bg_q.iter_mut() {
                let is_visible = modal_res
                    .get_state_by_id(&modal_id.0)
                    .copied()
                    .or_else(|| modal_res.get_state_by_entity(modal_entity).copied())
                    .unwrap_or(false);

                // Try to get the corresponding modal container
                if let Ok((mut progress, mut transform)) = modal_container_q.get_mut(container_entity.0) {
                    let old_progress = progress.0;

                    if is_visible {
                        *visibility = Visibility::Visible;
                        progress.0 = (progress.0 + delta).min(1.0);
                    } else {
                        *visibility = Visibility::Hidden;
                        progress.0 = (progress.0 - delta).max(0.0);
                    }
                    transform.scale = Vec3::splat(progress.0); // Uniform scaling

                    if old_progress != progress.0 && (progress.0 == 1.0 || progress.0 == 0.0) {
                        modal_res.state_changed = false;
                    }
                }
            }
        }
    }
}

/// Builder for creating modal widgets.
pub struct FaModalBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub clear_bg: bool,
    pub children: Option<Vec<Entity>>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaModalBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            clear_bg: false,
            children: Some(Vec::new()),
            root_node
        }
    }

    /// Method to add class to modal.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to modal.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Method to make modal background full transparent
    pub fn clear_bg(mut self) -> Self {
        self.clear_bg = true;
        self
    }

    /// Sets the child entities for the modal.
    ///
    /// # Parameters
    /// - `children`: An iterable collection of entities to add as children.
    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = Some(children.into_iter().collect());
        self
    }

    /// Spawn modal into UI World.
    pub fn build(&mut self) -> Entity {
        FaModal::new(
            self.id.clone(),
            self.class.clone(),
            self.clear_bg,
            self.children.as_ref().unwrap(),
            &mut self.root_node
        )
    }
}

/// API to create `FaModalBuilder`
pub fn fa_modal<'a>(builder: &'a mut FamiqBuilder) -> FaModalBuilder<'a> {
    FaModalBuilder::new(
        builder.ui_root_node.reborrow(),
    )
}

/// Determines if modal internal system(s) can run.
///
/// True only if there is a modal widget created.
pub fn can_run_modal_systems(modal_q: Query<&IsFamiqModalBackground>) -> bool {
    !modal_q.is_empty()
}
