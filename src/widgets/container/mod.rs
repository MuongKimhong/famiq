pub mod helper;
pub mod tests;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged
};
use helper::default_container_node;

/// Marker component for identifying a Famiq container.
#[derive(Component)]
pub struct IsFamiqContainer;

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

        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let container_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                bg_color.clone(),
                border_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqContainer,
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
            ))
            .id();

        root_node.add_child(container_entity);
        utils::insert_id_and_class(root_node, container_entity, &id, &class);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }
}

/// Builder for creating `FaContainer` entities with customizable options.
pub struct FaContainerBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: Option<Vec<Entity>>,
    pub root_note: EntityCommands<'a>
}

impl<'a> FaContainerBuilder<'a> {
    pub fn new(root_note: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            children: Some(Vec::new()),
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
        self.children = Some(children.into_iter().collect());
        self
    }

    /// Spawn container into UI World
    pub fn build(&mut self) -> Entity {
        FaContainer::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_note.reborrow(),
            self.children.as_ref().unwrap()
        )
    }
}

/// API to create `FaContainerBuilder`
pub fn fa_container<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaContainerBuilder<'a> {
    FaContainerBuilder::new(
        builder.ui_root_node.reborrow()
    )
}
