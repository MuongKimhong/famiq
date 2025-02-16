use bevy::prelude::*;
use crate::widgets::FamiqBuilder;

/// Marker component indicating that an entity is a Famiq background image.
#[derive(Component)]
pub struct IsFamiqBgImage;

/// Creates a background image that covers the entire window.
/// The background is set with a global z-index of -1, making it appear behind all other UI elements in the UI world.
pub struct FaBgImage;

impl<'a> FaBgImage {
    pub fn new(
        independent: bool,
        root_node: &'a mut EntityCommands,
        image_handle: Handle<Image>
    ) -> Entity {
        let node = Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(0.0)),
            ..default()
        };
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let entity = root_node.commands().spawn((
            ImageNode::new(image_handle),
            node,
            z_index,
            visibility,
            IsFamiqBgImage,
            Interaction::default(),
            GlobalZIndex(-1)
        )).id();

        if !independent {
            root_node.add_child(entity);
        }
        entity
    }
}

/// Builder for creating a `FaBgImage` entity with optional configurations.
pub struct FaBgImageBuilder<'a> {
    pub independent: Option<bool>,
    pub image_handle: Handle<Image>,
    pub root_node: EntityCommands<'a>,
}

impl<'a> FaBgImageBuilder<'a> {
    pub fn new(image_handle: Handle<Image>, root_node: EntityCommands<'a>) -> Self {
        Self {
            independent: Some(false),
            image_handle,
            root_node
        }
    }

    /// Method to set the background image to be independent of the root node.
    ///
    /// When set, the background image will not be despawned when the root node is despawned.
    pub fn independent(mut self) -> Self {
        self.independent = Some(true);
        self
    }

    /// Spawn background image into UI World.
    pub fn build(&mut self) -> Entity {
        FaBgImage::new(
            self.independent.unwrap(),
            &mut self.root_node,
            self.image_handle.clone()
        )
    }
}

/// API to create a `FaBgImageBuilder` for a background image.
pub fn fa_bg_image<'a>(builder: &'a mut FamiqBuilder, path: &str) -> FaBgImageBuilder<'a> {
    let image_handle = builder.asset_server.load(path);
    FaBgImageBuilder::new(
        image_handle,
        builder.ui_root_node.reborrow()
    )
}
