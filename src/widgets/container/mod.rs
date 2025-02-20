pub mod helper;
pub mod tests;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::widgets::{DefaultWidgetEntity, FamiqBuilder, FamiqWidgetId};
use super::BaseStyleComponents;
use helper::default_container_node;

/// Marker component for identifying a Famiq container.
#[derive(Component)]
pub struct IsFamiqContainer;

#[derive(Component)]
pub struct FaContainerChildren(pub Vec<Entity>);

/// Represents a Famiq container widget.
/// Think of it as a Div element in HTML.
pub struct FaContainer;

// Doesn't need container
impl<'a> FaContainer {
    pub fn new(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        children: &Vec<Entity>
    ) -> Entity {
        let mut node = default_container_node();
        utils::process_spacing_built_in_class(&mut node, &class);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = node;
        style_components.visibility = Visibility::Visible;

        let container_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqContainer,
                DefaultWidgetEntity::from(style_components),
                FaContainerChildren(children.clone()),
            ))
            .id();

        root_node.add_child(container_entity);
        utils::insert_id_and_class(root_node, container_entity, &id, &class);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }

    /// Add one child
    pub fn add_child(
        id: &str,
        commands: &mut Commands,
        containable_q: &Query<(Entity, Option<&FamiqWidgetId>), With<IsFamiqContainer>>,
        child: Entity
    ) {
        for (entity, _id) in containable_q.iter() {
            if let Some(_id) = _id {
                if &_id.0 == id {
                    commands.entity(entity).add_child(child);
                    break;
                }
            }
        }
    }

    /// Add multiple children
    pub fn add_children<I: IntoIterator<Item = Entity>>(
        id: &str,
        commands: &mut Commands,
        containable_q: &Query<(Entity, Option<&FamiqWidgetId>), With<IsFamiqContainer>>,
        children: I
    ) {
        for (entity, _id) in containable_q.iter() {
            if let Some(_id) = _id {
                if &_id.0 == id {
                    let children_vec: Vec<Entity> = children.into_iter().collect();
                    commands.entity(entity).add_children(&children_vec);
                    break;
                }
            }
        }
    }

    /// Insert children at given index
    pub fn insert_children<I: IntoIterator<Item = Entity>>(
        id: &str,
        commands: &mut Commands,
        containable_q: &Query<(Entity, Option<&FamiqWidgetId>), With<IsFamiqContainer>>,
        children: I,
        index: usize
    ) {
        for (entity, _id) in containable_q.iter() {
            if let Some(_id) = _id {
                if &_id.0 == id {
                    let children_vec: Vec<Entity> = children.into_iter().collect();
                    commands.entity(entity).insert_children(index, &children_vec);
                    break;
                }
            }
        }
    }

    pub fn remove_children<I: IntoIterator<Item = Entity>>(
        commands: &mut Commands,
        children: I,
    ) {
        for child in children {
            commands.entity(child).despawn_recursive();
        }
    }
}

/// Builder for creating `FaContainer` entities with customizable options.
pub struct FaContainerBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: Vec<Entity>,
    pub root_note: EntityCommands<'a>
}

impl<'a> FaContainerBuilder<'a> {
    pub fn new(root_note: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            children: Vec::new(),
            root_note
        }
    }

    /// Method to add class to container
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to container
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Sets the child entities for the container.
    ///
    /// # Parameters
    /// - `children`: An iterable collection of entities to add as children.
    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Spawn container into UI World
    pub fn build(&mut self) -> Entity {
        FaContainer::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_note.reborrow(),
            &self.children
        )
    }
}

/// API to create `FaContainerBuilder`
pub fn fa_container<'a>(builder: &'a mut FamiqBuilder) -> FaContainerBuilder<'a> {
    FaContainerBuilder::new(
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_container_systems(q: Query<&IsFamiqContainer>) -> bool {
    !q.is_empty()
}
