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

    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = Some(children.into_iter().collect());
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

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::widgets::button::fa_button;
    use crate::widgets::FamiqWidgetResource;
    use super::*;

    fn setup_test_default_container(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_container(&mut builder).id("#test-container").build();
    }

    fn setup_test_container_with_class(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_container(&mut builder)
            .id("#test-container")
            .class("test-class-one test-class-two")
            .build();
    }

    fn setup_test_container_with_children(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);

        let test_btn_1 = fa_button(&mut builder, "Button 1").build();
        let test_btn_2 = fa_button(&mut builder, "Button 2").build();

        fa_container(&mut builder)
            .id("#test-container")
            .children(vec![test_btn_1, test_btn_2])
            .build();
    }

    #[test]
    fn test_create_default_container() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_container);
        app.update();

        let container_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqContainer)>().get_single(app.world());
        assert!(container_q.is_ok(), "There should be only 1 container");

        let container_id = container_q.unwrap().0;
        assert_eq!("#test-container".to_string(), container_id.0);
    }

    #[test]
    fn test_create_container_with_class() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_container_with_class);
        app.update();

        let container_q = app.world_mut().query::<(&FamiqWidgetClasses, &IsFamiqContainer)>().get_single(app.world());
        assert!(container_q.is_ok(), "There should be only 1 container");

        let container_class = container_q.unwrap().0;
        assert_eq!("test-class-one test-class-two".to_string(), container_class.0);
    }

    #[test]
    fn test_create_container_with_children() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_container_with_children);
        app.update();

        let container_q = app.world_mut()
            .query::<(&Children, &IsFamiqContainer)>()
            .get_single(app.world());

        assert_eq!(2 as usize, container_q.unwrap().0.len());
    }
}
