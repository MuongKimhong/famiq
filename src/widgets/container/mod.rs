pub mod helper;
pub mod tests;

use bevy::prelude::*;

use crate::utils::{self, entity_add_children, entity_add_child};
use crate::widgets::{DefaultWidgetEntity, WidgetStyle, ExternalStyleHasChanged, IsFaWidgetRoot};
use helper::default_container_node;

use super::FamiqWidgetClasses;

/// Marker component for identifying a Famiq container.
#[derive(Component)]
pub struct IsFamiqContainer;

#[derive(Component)]
pub struct FaContainerChildren(pub Vec<Entity>);

/// Represents a Famiq container widget.
/// Think of it as a Div element in HTML.
pub struct FaContainer;

// Doesn't need container
impl FaContainer {
    pub fn _detect_fa_container_creation_system(
        mut commands: Commands,
        root_q: Query<Entity, With<IsFaWidgetRoot>>,
        container_q: Query<(Entity, &FaContainerChildren, Option<&FamiqWidgetClasses>), Added<IsFamiqContainer>>
    ) {
        for (entity, children, class) in container_q.iter() {
            let class_ref = class.map(|s| s.0.clone());
            let mut node = default_container_node();
            utils::process_spacing_built_in_class(&mut node, &class_ref);

            let bg_color = BackgroundColor::default();
            let border_color = BorderColor::default();
            let border_radius = BorderRadius::default();
            let z_index = ZIndex::default();
            let visibility = Visibility::Visible;
            commands
                .entity(entity)
                .insert((
                    node.clone(),
                    bg_color.clone(),
                    border_color.clone(),
                    border_radius.clone(),
                    z_index.clone(),
                    visibility.clone(),
                    DefaultWidgetEntity::new(
                        node,
                        border_color,
                        border_radius,
                        bg_color,
                        z_index,
                        visibility,
                    ),
                    Interaction::default(),
                    WidgetStyle::default(),
                    ExternalStyleHasChanged(false)
                ));

            entity_add_children(&mut commands, &children.0, entity);

            if let Ok(root_entity) = root_q.get_single() {
                entity_add_child(&mut commands, entity, root_entity);
            }
        }
    }
}

/// Builder for creating `FaContainer` entities with customizable options.
pub struct FaContainerBuilder<'w, 's> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: Vec<Entity>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> FaContainerBuilder<'w, 's> {
    pub fn new(commands: Commands<'w, 's>) -> Self {
        Self {
            id: None,
            class: None,
            children: Vec::new(),
            commands
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
        let entity = self.commands.spawn((
            IsFamiqContainer,
            FaContainerChildren(self.children.clone())
        ))
        .id();
        utils::insert_id_and_class(&mut self.commands, entity, &self.id, &self.class);
        entity
    }
}

/// API to create `FaContainerBuilder`
pub fn fa_container<'w, 's>(commands: &'w mut Commands) -> FaContainerBuilder<'w, 's>
where
    'w: 's
{
    FaContainerBuilder::new(commands.reborrow())
}
