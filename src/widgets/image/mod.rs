use bevy::prelude::*;
use crate::widgets::style_parse::parse_val;
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetBuilder
};

#[derive(Component)]
pub struct IsFamiqImage;

pub struct FaImage;

impl<'a> FaImage {
    pub fn new(
        id: &str,
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
            FamiqWidgetId(id.to_string()),
            DefaultWidgetEntity::new(
                node,
                border_color,
                border_radius,
                bg_color,
                z_index,
                visibility
            ),
            Interaction::default()
        )).id();

        if let Some(class) = class {
            root_node.commands().entity(image_entity).insert(FamiqWidgetClasses(class));
        }
        image_entity
    }
}

pub struct FaImageBuilder<'a> {
    pub id: String,
    pub image_handle: Handle<Image>,
    pub class: Option<String>,
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaImageBuilder<'a> {
    pub fn new(
        id: String,
        image_handle: Handle<Image>,
        root_node: EntityCommands<'a>
    ) -> Self {
        Self {
            id,
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

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaImage::new(
            self.id.as_str(),
            self.class.clone(),
            self.width.clone(),
            self.height.clone(),
            &mut self.root_node,
            self.image_handle.clone()
        )
    }
}

pub fn fa_image<'a>(builder: &'a mut FamiqWidgetBuilder, id: &str, path: &str) -> FaImageBuilder<'a> {
    let image_handle = builder.asset_server.load(path);
    FaImageBuilder::new(
        id.to_string(),
        image_handle,
        builder.ui_root_node.reborrow()
    )
}
