pub mod tests;

use bevy::prelude::*;
use crate::utils::{process_spacing_built_in_class, insert_id_and_class};
use crate::widgets::{
    DefaultWidgetEntity, FamiqBuilder,
    WidgetStyle, ExternalStyleHasChanged
};

/// Marker component identifyijng Famiq Image widget.
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
        process_spacing_built_in_class(&mut node, &class);

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

        insert_id_and_class(root_node, image_entity, &id, &class);
        image_entity
    }
}

/// Builder for creating image widget.
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

    /// Method to add class to image.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to image.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// set custom size for image
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Spawn image into UI World.
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

/// API to create `FaImageBuilder`
pub fn fa_image<'a>(builder: &'a mut FamiqBuilder, path: &str) -> FaImageBuilder<'a> {
    let image_handle = builder.asset_server.load(path);
    FaImageBuilder::new(
        image_handle,
        builder.ui_root_node.reborrow()
    )
}

/// a system to check if Image internal system(s) can run.
///
/// True only if image widget is created.
pub fn can_run_image_systems(image_q: Query<&IsFamiqImage>) -> bool {
    !image_q.is_empty()
}
