use bevy::prelude::*;
use crate::widgets::FamiqBuilder;

#[derive(Resource, Default)]
pub struct FaBgImageResource {
    pub image_path: String,

    /// only one bg image widget is allowed
    widget_created: bool,
    widget_entity: Option<Entity>
}

impl FaBgImageResource {
    pub fn change_image(&mut self, new_path: &str) {
        self.image_path = new_path.to_string();
    }
}

/// Marker component indicating that an entity is a Famiq background image.
#[derive(Component)]
pub struct IsFamiqBgImage;

#[derive(Component)]
pub struct BgImagePath(pub String);

/// Creates a background image that covers the entire window.
/// The background is set with a global z-index of -1, making it appear behind all other UI elements in the UI world.
pub struct FaBgImage;

impl<'a> FaBgImage {
    pub fn new(root_node: &'a mut EntityCommands, path: &str) -> Entity {
        root_node
            .commands()
            .spawn((
                IsFamiqBgImage,
                BgImagePath(path.to_string())
            ))
            .id()
    }

    pub fn detect_new_bg_image_system(
        mut commands: Commands,
        mut bg_resource: ResMut<FaBgImageResource>,
        asset_server: Res<AssetServer>,
        bg_q: Query<(Entity, &BgImagePath), Added<IsFamiqBgImage>>,
        window: Single<&Window>
    ) {
        // only one widget is allowed, if multiple detected, despawn

        if bg_resource.widget_created {
            return;
        }
        let mut first_entity = None;
        let size = window.physical_size();

        for (entity, path) in bg_q.iter() {
            if first_entity.is_none() {
                commands
                    .entity(entity)
                    .insert((
                        Sprite {
                            image: asset_server.load(&path.0),
                            custom_size: Some(Vec2::new(size.x as f32, size.y as f32)),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0))
                    ));
                bg_resource.image_path = path.0.clone();
                bg_resource.widget_entity = Some(entity);
                bg_resource.widget_created = true;
                first_entity = Some(entity);
            }
            else {
                commands.entity(entity).despawn();
            }
        }
    }

    pub fn handle_image_changed(
        bg_res: Res<FaBgImageResource>,
        asset_server: Res<AssetServer>,
        mut fa_bg_q: Query<&mut Sprite, With<IsFamiqBgImage>>
    ) {
        if bg_res.is_changed() && !bg_res.is_added() {
            if let Ok(mut sprite) = fa_bg_q.get_single_mut() {
                sprite.image = asset_server.load(&bg_res.image_path);
            }
        }
    }
}

/// Builder for creating a `FaBgImage` entity with optional configurations.
pub struct FaBgImageBuilder<'a> {
    pub root_node: EntityCommands<'a>,
    pub path: String
}

impl<'a> FaBgImageBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>, path: &str) -> Self {
        Self {
            root_node,
            path: path.to_string()
        }
    }

    /// Spawn background image into UI World.
    pub fn build(&mut self) -> Entity {
        FaBgImage::new(&mut self.root_node, &self.path)
    }
}

/// API to create a `FaBgImageBuilder` for a background image.
pub fn fa_bg_image<'a>(builder: &'a mut FamiqBuilder, path: &str) -> FaBgImageBuilder<'a> {
    FaBgImageBuilder::new(builder.ui_root_node.reborrow(), path)
}

pub fn can_run_bg_image_systems(bg_q: Query<&IsFamiqBgImage>) -> bool {
    !bg_q.is_empty()
}
