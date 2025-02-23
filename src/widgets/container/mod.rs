pub mod helper;
pub mod tests;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::widgets::*;
use crate::resources::*;
use super::BaseStyleComponents;
use helper::*;

#[derive(Default)]
pub struct IsFamiqContainerResource;
pub type FaContainerResource = ContainableResource<IsFamiqContainerResource>;

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
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        children: &Vec<Entity>
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.visibility = Visibility::Visible;

        let container_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqContainer,
                DefaultWidgetEntity::from(style_components),
                FaContainerChildren(children.to_owned())
            ))
            .id();

        root_node.add_child(container_entity);
        utils::insert_id_and_class(root_node, container_entity, &attributes.id, &attributes.class);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }

    pub fn detect_new_container_system(
        mut commands: Commands,
        mut container_res: ResMut<FaContainerResource>,
        container_q: Query<(Entity, Option<&FamiqWidgetId>, &FaContainerChildren), Added<IsFamiqContainer>>
    ) {
        for (entity, id, children) in container_q.iter() {
            if let Some(_id) = id {
                if container_res.containers.get(&_id.0).is_none() {
                    container_res.containers.insert(_id.0.clone(), ContainableData {
                        entity: Some(entity),
                        children: children.0.clone()
                    });
                    commands.entity(entity).remove::<FaContainerChildren>();
                }
            }
        }
    }

    pub(crate) fn detect_container_resource_change(
        mut commands: Commands,
        container_res: Res<FaContainerResource>,
        mut child_q: Query<
            (
                &mut Node,
                &mut DefaultWidgetEntity,
                Option<&FamiqWidgetId>,
                Option<&FamiqWidgetClasses>,
            )
        >,
        mut styles: ResMut<StylesKeyValueResource>
    ) {
        if container_res.is_changed() && !container_res.is_added() {
            if let Some(changed_container) = container_res.changed_container {

                match container_res.method_called {
                    ContainableMethodCall::AddChildren => {
                        commands
                            .entity(changed_container)
                            .add_children(&container_res.to_use_children);
                    }
                    ContainableMethodCall::InsertChildren => {
                        commands
                            .entity(changed_container)
                            .insert_children(container_res.insert_index, &container_res.to_use_children);
                    }
                    ContainableMethodCall::RemoveChildren => {
                        commands
                            .entity(changed_container)
                            .remove_children(&container_res.to_use_children);

                        for child in container_res.to_use_children.iter() {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                }
                let mut changed_json_style_keys: Vec<String> = Vec::new();
                for child in container_res.to_use_children.iter() {
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
                styles.changed_keys = changed_json_style_keys;
            }
        }
    }
}

/// Builder for creating `FaContainer` entities with customizable options.
pub struct FaContainerBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub children: Vec<Entity>,
    pub root_note: EntityCommands<'a>
}

impl<'a> FaContainerBuilder<'a> {
    pub fn new(root_note: EntityCommands<'a>) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            children: Vec::new(),
            root_note
        }
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
        self._node();
        FaContainer::new(
            &self.attributes,
            &mut self.root_note.reborrow(),
            &self.children
        )
    }
}

impl<'a> SetWidgetAttributes for FaContainerBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_container_node();

        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }

        utils::process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
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
