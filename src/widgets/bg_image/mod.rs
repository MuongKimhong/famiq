use bevy::prelude::*;
use crate::widgets::FamiqWidgetBuilder;

/// Marker component indicating that an entity is a Famiq background image.
#[derive(Component)]
pub struct IsFamiqBgImage;

/// Creates a background image that covers the entire window.
/// The background is set with a global z-index of -1, making it appear behind all other UI elements in the UI world.
pub struct FaBgImage;

impl<'a> FaBgImage {
    /// Creates a new background image entity.
    ///
    /// # Parameters
    /// - `independent`: Determines if the background image is independent of the root node.
    ///     - If `true`, the background image won't be a child of the root node, so it won't be despawned when the root node is despawned.
    ///     - If `false`, the background image will be a child of the root node.
    /// - `root_node`: A mutable reference to the `EntityCommands` for the root node where this background image will be attached.
    /// - `image_handle`: A handle to the image asset to use as the background.
    ///
    /// # Returns
    /// - The `Entity` of the created background image.
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
    /// Indicates whether the background image is independent of the root node.
    pub independent: Option<bool>,
    /// Handle to the image asset for the background.
    pub image_handle: Handle<Image>,
    /// The `EntityCommands` for the root node where the background image might be attached.
    pub root_node: EntityCommands<'a>,
}

impl<'a> FaBgImageBuilder<'a> {
    /// Creates a new `FaBgImageBuilder`.
    ///
    /// # Parameters
    /// - `image_handle`: A handle to the image asset to use as the background.
    /// - `root_node`: The `EntityCommands` for the root node where this background image might be attached.
    ///
    /// # Returns
    /// - A new instance of `FaBgImageBuilder`.
    pub fn new(image_handle: Handle<Image>, root_node: EntityCommands<'a>) -> Self {
        Self {
            independent: Some(false),
            image_handle,
            root_node
        }
    }

    /// Sets the background image to be independent of the root node.
    ///
    /// When set, the background image will not be despawned when the root node is despawned.
    ///
    /// # Returns
    /// - The updated `FaBgImageBuilder` instance.
    pub fn independent(mut self) -> Self {
        self.independent = Some(true);
        self
    }

    /// Builds and spawns the background image entity.
    ///
    /// # Returns
    /// - The `Entity` of the created background image.
    pub fn build(&mut self) -> Entity {
        FaBgImage::new(
            self.independent.unwrap(),
            &mut self.root_node,
            self.image_handle.clone()
        )
    }
}

/// API to create a `FaBgImageBuilder` for a background image.
///
/// This function loads the specified image asset and prepares a builder for creating the background image.
///
/// # Parameters
/// - `builder`: A mutable reference to a `FamiqWidgetBuilder`.
/// - `path`: The file path to the image asset to use as the background.
///
/// # Returns
/// - A `FaBgImageBuilder` preconfigured with the loaded image handle and the root node from the `FamiqWidgetBuilder`.
pub fn fa_bg_image<'a>(builder: &'a mut FamiqWidgetBuilder, path: &str) -> FaBgImageBuilder<'a> {
    let image_handle = builder.asset_server.load(path);
    FaBgImageBuilder::new(
        image_handle,
        builder.ui_root_node.reborrow()
    )
}
