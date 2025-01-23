pub mod helper;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::utils;
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetBuilder, WidgetStyle, ExternalStyleHasChanged
};
use helper::default_container_node;

#[derive(Component)]
pub struct IsFamiqContainer;

pub struct FaContainer;

// Doesn't need container
impl<'a> FaContainer {
    pub fn new(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        children: &Vec<Entity>
    ) -> Entity {
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

        if let Some(id) = id {
            root_node.commands().entity(container_entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(container_entity).insert(FamiqWidgetClasses(class));
        }
        root_node.add_child(container_entity);
        utils::entity_add_children(root_node, children, container_entity);
        container_entity
    }
}

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

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn children(mut self, children: Vec<Entity>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaContainer::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_note.reborrow(),
            self.children.as_ref().unwrap()
        )
    }
}

pub fn fa_container<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaContainerBuilder<'a> {
    FaContainerBuilder::new(
        builder.ui_root_node.reborrow()
    )
}
