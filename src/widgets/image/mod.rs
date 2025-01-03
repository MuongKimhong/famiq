use bevy::prelude::*;
use crate::widgets::{DefaultWidgetEntity, FamiqWidgetId};

#[derive(Component)]
pub struct IsFamiqImage;

pub struct FaImage;

impl<'a> FaImage {
    pub fn new(
        id: &str,
        path: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
    ) -> Entity {
        let node = Node::default();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        root_node.commands().spawn((
            ImageNode::new(asset_server.load(path)),
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
        )).id()
    }
}
