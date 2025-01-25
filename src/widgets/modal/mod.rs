pub mod helper;

use crate::widgets::{
    FamiqWidgetId, FamiqWidgetClasses, FamiqWidgetResource,
    DefaultWidgetEntity, FamiqWidgetBuilder, WidgetStyle, ExternalStyleHasChanged
};
use crate::utils;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use helper::*;

#[derive(Component)]
pub struct IsFamiqModalBackground;

#[derive(Component)]
pub struct IsFamiqModalContainer;

#[derive(Component)]
pub struct FaModalState(pub bool);

#[derive(Component)]
pub struct FaModalContainerEntity(pub Entity);

#[derive(Component)]
pub struct AnimationProgress(pub f32);

pub struct FaModal;

// Doesn't need container
impl<'a> FaModal {
    fn _build_modal_container(
        id: &Option<String>,
        root_node: &'a mut EntityCommands,
        items: &Vec<Entity>
    ) -> Entity {
        let node = default_modal_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        let container_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility
                ),
                IsFamiqModalContainer,
                FocusPolicy::Block,
                AnimationProgress(0.0)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(container_entity).insert(FamiqWidgetId(format!("{id}_modal_container")));
        }

        utils::entity_add_children(root_node, items, container_entity);
        container_entity
    }

    fn _build_modal_background(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        container_entity: Entity
    ) -> Entity {
        let entity = root_node
            .commands()
            .spawn((
                default_modal_background_node(),
                BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                BorderRadius::default(),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ZIndex::default(),
                GlobalZIndex(5),
                Visibility::Hidden,
                IsFamiqModalBackground,
                FaModalState(false),
                FocusPolicy::Block,
                FaModalContainerEntity(container_entity),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        root_node.add_child(entity);
        entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let container = Self::_build_modal_container(&id, root_node, items);
        let background = Self::_build_modal_background(id, class, root_node, container);

        utils::entity_add_child(root_node, container, background);
        container
    }

    pub fn hide_or_display_modal_system(
        mut modal_bg_q: Query<(&mut Visibility, &FaModalState, &FaModalContainerEntity)>,
        mut modal_container_q: Query<(&mut AnimationProgress, &mut Transform), With<IsFamiqModalContainer>>,
        time: Res<Time>
    ) {
        let delta = time.delta_secs() * 6.0;

        for (mut visibility, modal_state, container_entity) in modal_bg_q.iter_mut() {
            if let Ok((mut progress, mut transform)) = modal_container_q.get_mut(container_entity.0) {
                if modal_state.0 {
                    *visibility = Visibility::Visible;
                    progress.0 = (progress.0 + delta).min(1.0);
                }
                else {
                    *visibility = Visibility::Hidden;
                    progress.0 = (progress.0 - delta).max(0.0);
                }
                transform.scale = Vec3::splat(progress.0); // Uniform scaling
            }
        }
    }
}

pub struct FaModalBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: Option<Vec<Entity>>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaModalBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            children: Some(Vec::new()),
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

    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = Some(children.into_iter().collect());
        self
    }

    pub fn build(&mut self) -> Entity {
        FaModal::new(
            self.id.clone(),
            self.class.clone(),
            self.children.as_ref().unwrap(),
            &mut self.root_node
        )
    }
}

pub fn fa_modal<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaModalBuilder<'a> {
    builder.resource.can_run_systems.modal = true;
    FaModalBuilder::new(builder.ui_root_node.reborrow())
}

pub fn can_run_modal_systems(builder_res: Res<FamiqWidgetResource>) -> bool {
    builder_res.can_run_systems.modal
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::widgets::text::fa_text;
    use super::*;

    fn setup_test_default_modal(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_modal(&mut builder).id("#test-modal").build();
    }

    fn setup_test_modal_with_children(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        let txt_one = fa_text(&mut builder, "Text one").build();
        let txt_two = fa_text(&mut builder, "Text two").build();

        fa_modal(&mut builder)
            .children(vec![txt_one, txt_two])
            .build();
    }

    #[test]
    fn test_create_default_modal() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_modal);
        app.update();

        let modal_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqModalBackground)>().get_single(app.world());
        assert!(modal_q.is_ok(), "There should be only 1 listview");

        let modal_id = modal_q.unwrap().0;
        assert_eq!("#test-modal".to_string(), modal_id.0);
    }

    #[test]
    fn test_create_modal_with_children() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_modal_with_children);
        app.update();

        let modal_q = app.world_mut().query::<(&Children, &IsFamiqModalContainer)>().get_single(app.world());
        assert_eq!(2 as usize, modal_q.unwrap().0.len());
    }
}
