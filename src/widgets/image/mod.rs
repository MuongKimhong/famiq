pub mod tests;

use bevy::prelude::*;
use crate::utils::{process_spacing_built_in_class, insert_id_and_class};
use crate::widgets::{DefaultWidgetEntity, WidgetStyle, ExternalStyleHasChanged};

use super::FamiqWidgetClasses;

/// Marker component identifyijng Famiq Image widget.
#[derive(Component)]
pub struct IsFamiqImage;

/// Image path
#[derive(Component)]
pub struct FaImagePath(pub String);

#[derive(Component)]
pub struct FaImageSize {
    pub width: Val,
    pub height: Val
}

pub struct FaImage;

impl FaImage {
    pub fn _detect_fa_image_creation_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        image_q: Query<
            (Entity, &FaImagePath, Option<&FamiqWidgetClasses>, Option<&FaImageSize>),
            Added<IsFamiqImage>
        >
    ) {
        for (entity, path, class, size) in image_q.iter() {
            let class_ref = class.map(|s| s.0.clone());
            let image_handle: Handle<Image> = asset_server.load(&path.0);

            let mut node = Node::default();
            process_spacing_built_in_class(&mut node, &class_ref);

            if let Some(size) = size {
                node.width = size.width.clone();
                node.height = size.height.clone();
            }
            commands
                .entity(entity)
                .insert((
                    ImageNode::new(image_handle),
                    node.clone(),
                    BackgroundColor::default(),
                    BorderRadius::default(),
                    BorderColor::default(),
                    ZIndex::default(),
                    Visibility::Inherited,
                    DefaultWidgetEntity::new(
                        node,
                        BorderColor::default(),
                        BorderRadius::default(),
                        BackgroundColor::default(),
                        ZIndex::default(),
                        Visibility::Inherited
                    ),
                    Interaction::default(),
                    WidgetStyle::default(),
                    ExternalStyleHasChanged(false)
                ));
        }
    }
}

/// Builder for creating image widget.
pub struct FaImageBuilder<'w, 's> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub path: String,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> FaImageBuilder<'w, 's> {
    pub fn new(path: &str, commands: Commands<'w, 's>) -> Self {
        Self {
            id: None,
            class: None,
            width: None,
            height: None,
            path: path.to_string(),
            commands
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
        let entity = self.commands.spawn((
            IsFamiqImage,
            FaImagePath(self.path.clone())
        ))
        .id();
        insert_id_and_class(&mut self.commands, entity, &self.id, &self.class);

        if self.width.is_some() && self.height.is_some() {
            self.commands.entity(entity).insert(FaImageSize {
                width: self.width.unwrap(),
                height: self.height.unwrap()
            });
        }
        entity
    }
}

/// API to create `FaImageBuilder`
pub fn fa_image<'w, 's>(commands: &'w mut Commands, path: &str) -> FaImageBuilder<'w, 's>
where
    'w: 's
{
    FaImageBuilder::new(path, commands.reborrow())
}
