pub mod tests;

use bevy::prelude::*;
use crate::utils::{process_spacing_built_in_class, insert_id_and_class};
use crate::widgets::*;

/// Marker component identifyijng Famiq Image widget.
#[derive(Component)]
pub struct IsFamiqImage;

pub struct FaImage;

impl<'a> FaImage {
    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();

        let image_entity = root_node
            .commands()
            .spawn((
                ImageNode::new(attributes.image_handle.clone().unwrap()),
                style_components.clone(),
                IsFamiqImage,
                DefaultWidgetEntity::from(style_components)
            ))
            .id();

        insert_id_and_class(root_node, image_entity, &attributes.id, &attributes.class);
        image_entity
    }
}

/// Builder for creating image widget.
pub struct FaImageBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaImageBuilder<'a> {
    pub fn new(image_handle: Handle<Image>, root_node: EntityCommands<'a>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.image_handle = Some(image_handle);
        Self {
            attributes,
            root_node
        }
    }

    /// set custom size for image
    pub fn set_size(mut self, width: Val, height: Val) -> Self {
        self.attributes.node.width = width;
        self.attributes.node.height = height;
        self
    }

    /// Spawn image into UI World.
    pub fn build(&mut self) -> Entity {
        self._node();
        FaImage::new(&self.attributes, &mut self.root_node)
    }
}

impl<'a> SetWidgetAttributes for FaImageBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
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
