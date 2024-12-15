pub mod helper;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::widgets::{DefaultWidgetBundle, FaWidgetBundle, FamiqWidgetId};
use helper::*;

#[derive(Component)]
pub struct IsFamiqContainer;

pub struct FaContainer;

// containable
impl<'a> FaContainer {
    pub fn new(
        id: &str,
        root_node: &'a mut EntityCommands,
        children: &Vec<Entity>,
        custom_bundle: Option<FaWidgetBundle>,
    ) -> Entity {
        let container_bundle = match custom_bundle {
            Some(v) => v,
            None => FaWidgetBundle {
                style: get_default_container_style(),
                ..default()
            },
        };
        let container_entity = root_node
            .commands()
            .spawn((
                container_bundle.clone(),
                FamiqWidgetId(id.to_string()),
                IsFamiqContainer,
                DefaultWidgetBundle(container_bundle),
            ))
            .id();

        root_node.add_child(container_entity);
        utils::entity_push_children(root_node, children, container_entity);
        container_entity
    }
}
