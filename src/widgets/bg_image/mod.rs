use bevy::prelude::*;

use super::IsFaWidgetRoot;

/// Marker component indicating that an entity is a Famiq background image.
#[derive(Component)]
pub struct IsFamiqBgImage;

#[derive(Component)]
pub struct FaBgImagePath(pub String);

/// Creates a background image that covers the entire window.
/// The background is set with a global z-index of -1, making it appear behind all other UI elements in the UI world.
pub struct FaBgImage;

impl FaBgImage {
    pub fn detect_bg_image_creation_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        bg_q: Query<(Entity, &FaBgImagePath), Added<IsFamiqBgImage>>,
        root_q: Query<Entity, With<IsFaWidgetRoot>>
    ) {
        for (entity, path) in bg_q.iter() {
            let image_handle: Handle<Image> = asset_server.load(&path.0);
            commands
                .entity(entity)
                .insert((
                    ImageNode::new(image_handle),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ZIndex::default(),
                    Visibility::Visible,
                    GlobalZIndex(-1)
                ));

            if let Ok(root_entity) = root_q.get_single() {
                commands.entity(root_entity).add_child(entity);
            }
        }
    }
}

/// Builder for creating a `FaBgImage` entity with optional configurations.
pub struct FaBgImageBuilder<'w, 's> {
    pub path: String,
    pub commands: Commands<'w, 's>,
}

impl<'w, 's> FaBgImageBuilder<'w, 's> {
    pub fn new(path: &str, commands: Commands<'w, 's>) -> Self {
        Self {
            path: path.to_string(),
            commands
        }
    }

    /// Spawn background image into UI World.
    pub fn build(&mut self) -> Entity {
        self.commands.spawn((
            IsFamiqBgImage,
            FaBgImagePath(self.path.clone())
        ))
        .id()
    }
}

/// API to create a `FaBgImageBuilder` for a background image.
pub fn fa_bg_image<'w, 's>(commands: &'w mut Commands, path: &str) -> FaBgImageBuilder<'w, 's>
where
    'w: 's
{
    FaBgImageBuilder::new(path, commands.reborrow())
}
