pub mod helper;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
// use crate::widgets::{DefaultWidgetBundle, FaWidgetBundle, FamiqWidgetId};
use crate::widgets::FamiqWidgetId;
use helper::default_container_node;

#[derive(Component)]
pub struct IsFamiqContainer;

pub struct FaContainer;

// containable
impl<'a> FaContainer {
    pub fn new(id: &str, root_node: &'a mut EntityCommands, children: &Vec<Entity>) -> Entity {
        let container_entity = root_node
            .commands()
            .spawn((
                default_container_node(),
                FamiqWidgetId(id.to_string()),
                IsFamiqContainer,
            ))
            .id();

        root_node.add_child(container_entity);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }
}
