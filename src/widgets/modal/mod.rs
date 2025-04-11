pub mod helper;
pub mod tests;

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

/// Component that keep tracking of modal show/hide animation.
#[derive(Component)]
pub struct AnimationProgress(pub f32);

/// Use to define show/hide state for modal
/// by id or entity.
#[derive(Resource, Default, Debug)]
pub struct FaModalState {
    pub entity_states: HashMap<Entity, bool>,
    pub state_changed: bool
}

impl FaModalState {
    fn _update_or_insert_entity(&mut self, entity: Entity, new_state: bool) {
        self.entity_states.entry(entity).or_insert(false);
        self.entity_states.insert(entity, new_state);
    }

    fn _hide_all(&mut self) {
        self.entity_states.values_mut().for_each(|v| *v = false);
    }

    /// Show modal by entity (Only one can be `true`)
    pub(crate) fn show_by_entity(&mut self, entity: Entity) {
        self._hide_all();
        self._update_or_insert_entity(entity, true);
        self.state_changed = true;
    }

    /// Hide modal by entity
    pub(crate) fn hide_by_entity(&mut self, entity: Entity) {
        self._update_or_insert_entity(entity, false);
        self.state_changed = true;
    }

    pub fn get_state_by_entity(&self, entity: Entity) -> Option<&bool> {
        self.entity_states.get(&entity)
    }
}

pub struct FaModal;

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
        container_entity: Entity
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.visibility = Visibility::Visible;
        style_components.border_color = BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.8));

        if !clear_bg {
            style_components.background_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8));
        }

        let entity = root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqModalBackground,
                IsFamiqMainWidget,
                IsFamiqContainableWidget,
                FocusPolicy::Block,
                FaModalContainerEntity(container_entity),
                GlobalZIndex(5)
            ))
            .id();

        utils::insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        root_node.add_child(entity);

        if attributes.model_key.is_some() {
            root_node
                .commands()
                .entity(entity)
                .insert(ReactiveModelKey(attributes.model_key.clone().unwrap()));
        }
        entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        clear_bg: bool,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let container = Self::_build_modal_container(root_node, items);
        let background = Self::_build_modal_background(attributes, clear_bg, root_node, container);

        utils::entity_add_child(root_node, container, background);
        background
    }

    // Internal system to hide or display via `FaModalState` resource.
    pub(crate) fn hide_or_display_modal_system(
        mut modal_bg_q: Query<(&mut Node, Entity, &FaModalContainerEntity)>,
        mut modal_container_q: Query<(&mut AnimationProgress, &mut Transform), With<IsFamiqModalContainer>>,
        time: Res<Time>,
        mut modal_res: ResMut<FaModalState>,
    ) {
        if modal_res.state_changed {
            let delta = time.delta_secs() * 8.0;

            for (mut node, entity, container_entity) in modal_bg_q.iter_mut() {
                let is_visible = modal_res
                    .get_state_by_entity(entity)
                    .copied()
                    .unwrap_or(false);

                if let Ok((mut progress, mut transform)) = modal_container_q.get_mut(container_entity.0) {
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
pub fn fa_modal_builder<'a>(builder: &'a mut FamiqBuilder) -> FaModalBuilder<'a> {
    FaModalBuilder::new(
        builder.ui_root_node.reborrow(),
    )
}

#[macro_export]
macro_rules! fa_modal {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let builder = builder_mut();
        #[allow(unused_mut)]
        let mut children_vec: Vec<Entity> = Vec::new();
        $(
            $crate::extract_children!(children_vec, builder, $key : $value);
        )*

        let mut modal = fa_modal_builder(builder);

        $(
            $crate::fa_modal_attributes!(modal, $key : $value);
        )*

        modal = modal.children(children_vec);
        modal.build()
    }};
}

#[macro_export]
macro_rules! fa_modal_attributes {
    // skip children
    ($modal:ident, children: $children_vec:tt) => {{}};

    ($modal:ident, model: $model:expr) => {{
        $modal = $modal.model($model);
    }};

    ($modal:ident, clear_bg: $clear_bg:expr) => {{
        $modal.clear_bg = $clear_bg;
    }};

    // common attributes
    ($modal:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($modal, $key : $value);
    }};
}

/// Determines if modal internal system(s) can run.
///
/// True only if there is a modal widget created.
pub fn can_run_modal_systems(modal_q: Query<&IsFamiqModalBackground>) -> bool {
    !modal_q.is_empty()
}
