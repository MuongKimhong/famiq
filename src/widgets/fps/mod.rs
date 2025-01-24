pub mod helper;

use crate::utils::entity_add_child;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity,
    FamiqWidgetId, FamiqWidgetBuilder, FamiqWidgetResource,
    WidgetStyle, ExternalStyleHasChanged
};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

use super::color::{GREEN_COLOR, WHITE_COLOR, WARNING_COLOR, DANGER_COLOR};

const DEFAULT_FPS_TEXT_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct IsFamiqFPSTextLabel;

#[derive(Component)]
pub struct IsFamiqFPSTextCount;

#[derive(Component)]
pub struct IsFamiqFPSTextContainer;

// whether fps change color. green > 100, orange < 100, red < 60
#[derive(Component)]
pub struct CanChangeColor(pub bool);

pub struct FaFpsText;

// Doesn't need container
impl<'a> FaFpsText {
    fn _build_container(id: &Option<String>, right_side: bool, root_node: &'a mut EntityCommands) -> Entity {
        let mut node = default_fps_text_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        if right_side {
            node.left = Val::Auto;
            node.right = Val::Px(6.0);
        }

        let entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqFPSTextContainer,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                GlobalZIndex(6),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(format!("{id}_fps_text_container")));
        }
        root_node.add_child(entity);
        entity
    }

    fn _build_text(
        id: Option<String>,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        change_color: bool
    ) -> Entity {
        let label_txt = Text::new("FPS:");
        let label_txt_font = TextFont {
            font: font_handle,
            font_size: DEFAULT_FPS_TEXT_SIZE,
            ..default()
        };
        let label_txt_color = TextColor(WHITE_COLOR);
        let label_txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let count_txt = TextSpan::default();
        let count_txt_font = label_txt_font.clone();
        let count_txt_color = TextColor(GREEN_COLOR);

        let label_txt_entity = root_node
            .commands()
            .spawn((
                label_txt.clone(),
                label_txt_font.clone(),
                label_txt_color.clone(),
                label_txt_layout.clone(),
                DefaultTextEntity::new(
                    label_txt,
                    label_txt_font,
                    label_txt_color,
                    label_txt_layout,
                ),
                IsFamiqFPSTextLabel,
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(label_txt_entity).insert(FamiqWidgetId(id));
        }

        let count_txt_entity = root_node
            .commands()
            .spawn((
                count_txt,
                count_txt_font,
                count_txt_color,
                IsFamiqFPSTextCount,
                CanChangeColor(change_color)
            ))
            .id();

        entity_add_child(root_node, count_txt_entity, label_txt_entity);
        label_txt_entity
    }

    pub fn new(
        id: Option<String>,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        change_color: bool,
        right_side: bool,
    ) -> Entity {
        let container_entity = Self::_build_container(&id, right_side, root_node);
        let text_entity = Self::_build_text(id, root_node, font_handle, change_color);

        entity_add_child(root_node, text_entity, container_entity);
        text_entity
    }

    pub fn update_fps_count_system(
        diagnostics: Res<DiagnosticsStore>,
        mut text_q: Query<(&mut TextSpan, &mut TextColor, &CanChangeColor, &IsFamiqFPSTextCount)>
    ) {
        for (mut text, mut color, change_color, _) in text_q.iter_mut() {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    text.0 = format!("{value:.2}");

                    if change_color.0 {
                        if value > 100.0 {
                            color.0 = GREEN_COLOR;
                        }
                        else if value > 60.0 && value < 100.0 {
                            color.0 = WARNING_COLOR;
                        }
                        else {
                            color.0 = DANGER_COLOR;
                        }
                    }
                    else {
                        color.0 = WHITE_COLOR;
                    }
                }
            }
        }
    }
}

pub struct FaFpsTextBuilder<'a> {
    pub id: Option<String>,
    pub change_color: Option<bool>,
    pub right_side: Option<bool>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaFpsTextBuilder<'a> {
    pub fn new(font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            root_node,
            font_handle,
            change_color: Some(false),
            right_side: Some(false)
        }
    }

    pub fn change_color(mut self) -> Self {
        self.change_color = Some(true);
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn right_side(mut self) -> Self {
        self.right_side = Some(true);
        self
    }

    pub fn build(&mut self) -> Entity {
        FaFpsText::new(
            self.id.clone(),
            &mut self.root_node,
            self.font_handle.clone(),
            self.change_color.unwrap(),
            self.right_side.unwrap()
        )
    }
}

pub fn fa_fps<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaFpsTextBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    builder.resource.can_run_systems.fps = true;

    FaFpsTextBuilder::new(
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_fps_systems(builder_res: Res<FamiqWidgetResource>) -> bool {
    builder_res.can_run_systems.fps
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::utils::create_test_app;
    use super::*;

    fn setup_test_default_fps(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_fps(&mut builder)
            .id("#test-fps")
            .build();
    }

    fn setup_test_fps_with_change_color(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_fps(&mut builder)
            .change_color()
            .build();
    }

    fn setup_test_fps_with_right_side(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_fps(&mut builder)
            .right_side()
            .build();
    }

    #[test]
    fn test_create_default_fps() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_fps);
        app.update();

        let fps_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqFPSTextLabel)>().get_single(app.world());
        assert!(fps_q.is_ok(), "There should be only 1 fps widget");

        let fps_id = fps_q.unwrap().0;
        assert_eq!("#test-fps".to_string(), fps_id.0);
    }

    #[test]
    fn test_create_fps_with_change_color() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_fps_with_change_color);
        app.update();

        let fps_q = app.world_mut().query::<(&CanChangeColor, &IsFamiqFPSTextCount)>().get_single(app.world());

        let fps_can_change_color_flag = fps_q.unwrap().0;
        assert_eq!(true, fps_can_change_color_flag.0);
    }

    #[test]
    fn test_create_fps_with_right_side() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_fps_with_right_side);
        app.update();

        let fps_q = app.world_mut().query::<(&Node, &IsFamiqFPSTextContainer)>().get_single(app.world());

        let fps_node = fps_q.unwrap().0;

        // when right_side is true, right is Val::Px(6.0) and left is Val::Auto by default
        assert_eq!(Val::Px(6.0), fps_node.right);
        assert_eq!(Val::Auto, fps_node.left);
    }
}
