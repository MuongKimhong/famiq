pub mod helper;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::event_writer::FaInteractionEvent;
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetResource, WidgetType, FamiqWidgetBuilder
};
use helper::default_container_node;

#[derive(Component)]
pub struct IsFamiqContainer;

pub struct FaContainer;

// Doesn't need container
impl<'a> FaContainer {
    pub fn new(id: &str, class: Option<String>, root_node: &'a mut EntityCommands, children: &Vec<Entity>) -> Entity {
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

        if let Some(class) = class {
            root_node.commands().entity(container_entity).insert(FamiqWidgetClasses(class));
        }

        root_node.add_child(container_entity);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }

    pub fn handle_container_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut builder_res: ResMut<FamiqWidgetResource>
    ) {
        for e in events.read() {
            if e.widget == WidgetType::Container && e.interaction == Interaction::Pressed {
                builder_res.update_all_focus_states(false);
                builder_res.update_or_insert_focus_state(e.entity, true);
            }
        }
    }
}

pub struct FaContainerBuilder<'a> {
    pub id: String,
    pub class: Option<String>,
    pub children: Option<Vec<Entity>>,
    pub root_note: EntityCommands<'a>
}

impl<'a> FaContainerBuilder<'a> {
    pub fn new(id: String, root_note: EntityCommands<'a>) -> Self {
        Self {
            id,
            class: None,
            children: Some(Vec::new()),
            root_note
        }
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn children(mut self, children: Vec<Entity>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaContainer::new(
            self.id.as_str(),
            self.class.clone(),
            &mut self.root_note.reborrow(),
            self.children.as_ref().unwrap()
        )
    }
}

pub fn fa_container<'a>(builder: &'a mut FamiqWidgetBuilder, id: &str) -> FaContainerBuilder<'a> {
    FaContainerBuilder::new(
        id.to_string(),
        builder.ui_root_node.reborrow()
    )
}
