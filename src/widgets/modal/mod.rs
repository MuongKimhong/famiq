pub mod helper;
pub mod tests;

use crate::resources::*;
use crate::widgets::*;
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

#[derive(Component)]
pub struct FaModalChildren(pub Vec<Entity>);

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

#[derive(Default)]
pub struct IsFamiqModalResource;
pub type FaModalResource = ContainableResource<IsFamiqModalResource>;

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
                AnimationProgress(0.0),
                Transform::from_scale(Vec3::splat(0.0))
            ))
            .id();

        utils::entity_add_children(root_node, items, container_entity);
        container_entity
    }

    fn _build_modal_background(
        attributes: &WidgetAttributes,
        clear_bg: bool,
        root_node: &'a mut EntityCommands,
        container_entity: Entity,
        items: &Vec<Entity>
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
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
                FaModalChildren(items.clone()),
                GlobalZIndex(5)
            ))
            .id();

        utils::insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        root_node.add_child(entity);
        entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        clear_bg: bool,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let container = Self::_build_modal_container(root_node, items);
        let background = Self::_build_modal_background(attributes, clear_bg, root_node, container, items);

        utils::entity_add_child(root_node, container, background);
        background
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

    pub fn detect_new_modal_system(
        mut commands: Commands,
        mut modal_res: ResMut<FaModalResource>,
        modal_q: Query<(Entity, Option<&FamiqWidgetId>, &FaModalChildren), Added<IsFamiqModalBackground>>
    ) {
        for (entity, id, children) in modal_q.iter() {
            if let Some(_id) = id {
                if modal_res.containers.get(&_id.0).is_none() {
                    modal_res.containers.insert(_id.0.clone(), ContainableData {
                        entity: Some(entity),
                        children: children.0.clone()
                    });
                    commands.entity(entity).remove::<FaModalChildren>();
                }
            }
        }
    }

    pub(crate) fn detect_modal_resource_change(
        mut commands: Commands,
        modal_res: Res<FaModalResource>,
        modal_q: Query<&FaModalContainerEntity>,
        mut child_q: Query<(
            &mut Node,
            &mut DefaultWidgetEntity,
            Option<&FamiqWidgetId>,
            Option<&FamiqWidgetClasses>
        )>,
        mut styles: ResMut<StylesKeyValueResource>
    ) {
        if modal_res.is_changed() && !modal_res.is_added() {
            if modal_res.changed_container.is_none() {
                return;
            }

            let changed_modal = modal_res.changed_container.unwrap();

            let modal_container_entity = match modal_q.get(changed_modal) {
                Ok(v) => v.0,
                Err(_) => return,
            };

            match modal_res.method_called {
                ContainableMethodCall::AddChildren => {
                    commands
                        .entity(modal_container_entity)
                        .add_children(&modal_res.to_use_children);
                }
                ContainableMethodCall::InsertChildren => {
                    commands
                        .entity(modal_container_entity)
                        .insert_children(modal_res.insert_index, &modal_res.to_use_children);
                }
                ContainableMethodCall::RemoveChildren => {
                    commands
                        .entity(modal_container_entity)
                        .remove_children(&modal_res.to_use_children);

                    for child in modal_res.to_use_children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            }

            let mut changed_json_style_keys: Vec<String> = Vec::new();
            for child in modal_res.to_use_children.iter() {
                if let Ok((mut node, mut default_widget, id, class)) = child_q.get_mut(*child) {
                    if node.display == Display::None {
                        node.display = Display::Flex;
                        default_widget.node.display = Display::Flex;
                    }
                    if let Some(id) = id {
                        changed_json_style_keys.push(id.0.clone());
                    }
                    if let Some(classes) = class {
                        let classes_split: Vec<&str> = classes.0.split_whitespace().collect();
                        for class_name in classes_split {
                            let formatted = format!(".{class_name}");
                            if !changed_json_style_keys.contains(&formatted) {
                                changed_json_style_keys.push(formatted);
                            }
                        }
                    }
                }
            }
            styles.changed_keys= changed_json_style_keys;
        }
    }
}

/// Builder for creating modal widgets.
pub struct FaModalBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub clear_bg: bool,
    pub children: Vec<Entity>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaModalBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            clear_bg: false,
            children: Vec::new(),
            root_node
        }
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
        self.children = children.into_iter().collect();
        self
    }

    /// Spawn modal into UI World.
    pub fn build(&mut self) -> Entity {
        self._node();
        FaModal::new(
            &self.attributes,
            self.clear_bg,
            &self.children,
            &mut self.root_node
        )
    }
}

impl<'a> SetWidgetAttributes for FaModalBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_modal_background_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
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
