pub mod helper;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::event_writer::FaInteractionEvent;
use crate::widgets::{DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses, FamiqWidgetBuilderResource, WidgetType};
use helper::default_container_node;

#[derive(Component)]
pub struct IsFamiqContainer;

pub struct FaContainer;

// Doesn't need container
impl<'a> FaContainer {
    pub fn new(id: &str, classes: &str, root_node: &'a mut EntityCommands, children: &Vec<Entity>) -> Entity {
        let node = default_container_node();
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
                FamiqWidgetId(id.to_string()),
                FamiqWidgetClasses(classes.to_string()),
                IsFamiqContainer,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default()
            ))
            .id();

        root_node.add_child(container_entity);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }

    pub fn handle_container_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut builder_res: ResMut<FamiqWidgetBuilderResource>
    ) {
        for e in events.read() {
            if e.widget == WidgetType::Container && e.interaction == Interaction::Pressed {
                builder_res.update_all_focus_states(false);
                builder_res.update_or_insert_focus_state(e.entity, true);
            }
        }
    }
}
