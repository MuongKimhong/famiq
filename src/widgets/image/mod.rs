use bevy::prelude::*;
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetBuilder, WidgetStyle, ExternalStyleHasChanged
};

#[derive(Component)]
pub struct IsFamiqImage;

pub struct FaImage;

impl<'a> FaImage {
    pub fn new(
        id: Option<String>,
        class: Option<String>,
        width: Option<Val>,
        height: Option<Val>,
        root_node: &'a mut EntityCommands,
        image_handle: Handle<Image>
    ) -> Entity {
        let mut node = Node::default();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        if let Some(w) = width {
            node.width = w;
        }
        if let Some(h) = height {
            node.height = h;
        }
        let image_entity = root_node.commands().spawn((
            ImageNode::new(image_handle),
            node.clone(),
            bg_color.clone(),
            border_radius.clone(),
            border_color.clone(),
            z_index.clone(),
            visibility.clone(),
            IsFamiqImage,
            DefaultWidgetEntity::new(
                node,
                border_color,
                border_radius,
                bg_color,
                z_index,
                visibility
            ),
            Interaction::default(),
            WidgetStyle::default(),
            ExternalStyleHasChanged(false)
        )).id();

        if let Some(id) = id {
            root_node.commands().entity(image_entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(image_entity).insert(FamiqWidgetClasses(class));
        }
        image_entity
    }
}

pub struct FaImageBuilder<'a> {
    pub id: Option<String>,
    pub image_handle: Handle<Image>,
    pub class: Option<String>,
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaImageBuilder<'a> {
    pub fn new(image_handle: Handle<Image>, root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            width: None,
            height: None,
            image_handle,
            root_node
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

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaImage::new(
            self.id.clone(),
            self.class.clone(),
            self.width.clone(),
            self.height.clone(),
            &mut self.root_node,
            self.image_handle.clone()
        )
    }
}

pub fn fa_image<'a>(builder: &'a mut FamiqWidgetBuilder, path: &str) -> FaImageBuilder<'a> {
    let image_handle = builder.asset_server.load(path);
    FaImageBuilder::new(
        image_handle,
        builder.ui_root_node.reborrow()
    )
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::widgets::FamiqWidgetResource;
    use crate::utils::{get_embedded_asset_path, create_test_app};
    use super::*;

    fn setup_test_default_image(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
        fa_image(&mut builder, path.as_str()).id("#test-image").build();
    }

    fn setup_test_image_with_class(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
        fa_image(&mut builder, path.as_str())
            .class("test-class-one")
            .build();
    }

    fn setup_test_image_with_custom_size(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        let path = get_embedded_asset_path("embedded_assets/logo.jpeg").to_string();
        fa_image(&mut builder, path.as_str())
            .size(Val::Px(200.0), Val::Px(200.0))
            .build();
    }

    #[test]
    fn test_create_default_image() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_image);
        app.update();

        let img_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqImage)>().get_single(app.world());
        assert!(img_q.is_ok(), "There should be only 1 Image");

        let img_id = img_q.unwrap().0;
        assert_eq!("#test-image".to_string(), img_id.0);
    }

    #[test]
    fn test_create_image_with_class() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_image_with_class);
        app.update();

        let img_q = app.world_mut().query::<(&FamiqWidgetClasses, &IsFamiqImage)>().get_single(app.world());
        assert!(img_q.is_ok(), "There should be only 1 Image");
        assert_eq!("test-class-one".to_string(), img_q.unwrap().0.0);
    }

    #[test]
    fn test_create_image_with_custom_size() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_image_with_custom_size);
        app.update();

        let img_q = app.world_mut().query::<(&Node, &IsFamiqImage)>().get_single(app.world());
        let img_node = img_q.unwrap().0;
        assert_eq!(Val::Px(200.0), img_node.width);
        assert_eq!(Val::Px(200.0), img_node.height);
    }
}
