pub mod helper;

use crate::utils;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity,
    FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetResource, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged
};
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use helper::*;

#[derive(Component)]
pub struct IsFamiqButton;

#[derive(Component)]
pub struct IsFamiqButtonText;

#[derive(Component)]
pub struct ButtonTextEntity(pub Entity);

#[derive(Component)]
pub struct ButtonTextContainerEntity(pub Entity);

#[derive(Component)]
pub struct FaButtonText(pub String);

pub enum BtnColor {
    Default,
    Primary,
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark
}

pub enum BtnSize {
    Small,
    Normal,
    Large,
}

pub enum BtnShape {
    Default,
    Round,
    Rectangle
}

pub struct FaButton;

// Needs container
impl<'a> FaButton {
    fn _build_text(
        id: &Option<String>,
        text: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        color: &BtnColor,
        size: &BtnSize,
    ) -> Entity {
        let txt = Text::new(text);
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(size),
            ..default()
        };
        let txt_color = TextColor(get_text_color(color));
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                FaButtonText(text.to_string()),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqButtonText
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(format!("{id}_btn_text")));
        }
        entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        text: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        color: BtnColor,
        size: BtnSize,
        shape: BtnShape
    ) -> Entity {
        let txt_entity = Self::_build_text(&id, text, root_node, font_handle, &color, &size);

        let node = default_button_node();
        let border_color = get_button_border_color(&color);
        let bg_color = get_button_background_color(&color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;
        let mut border_radius =  BorderRadius::all(Val::Px(6.0));

        match shape {
            BtnShape::Round => border_radius = BorderRadius::all(Val::Percent(50.0)),
            BtnShape::Rectangle => border_radius = BorderRadius::all(Val::Px(0.0)),
            _ => ()
        }
        let btn_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                bg_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqButton,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                ButtonTextEntity(txt_entity),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(btn_entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(btn_entity).insert(FamiqWidgetClasses(class));
        }
        utils::entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }

    pub fn handle_button_on_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        mut button_q: Query<(&IsFamiqButton, &DefaultWidgetEntity, &mut BackgroundColor, &mut BorderColor)>,
        mut builder_res: ResMut<FamiqWidgetResource>
    ) {
        for e in events.read() {
            if let Ok((_, default_style, mut bg_color, mut bd_color)) = button_q.get_mut(e.entity) {
                match e.interaction {
                    Interaction::Hovered => {
                        // darken by 10%
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                        darken_bg_and_bg_color(10.0, &mut bg_color, &mut bd_color);
                    },
                    Interaction::Pressed => {
                        // darken by 15%
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                        darken_bg_and_bg_color(15.0, &mut bg_color, &mut bd_color);

                        builder_res.update_all_focus_states(false);
                        builder_res.update_or_insert_focus_state(e.entity, true);
                    },
                    Interaction::None => {
                        set_default_bg_and_bd_color(default_style, &mut bg_color, &mut bd_color);
                    },
                }
            }
        }
    }
}

pub struct FaButtonBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub text: String,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>,
}

impl<'a> FaButtonBuilder<'a> {
    pub fn new(
        text: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>,
    ) -> Self {
        Self {
            id: None,
            class: None,
            text,
            font_handle,
            root_node,
        }
    }

    fn _process_built_in_classes(&self) -> (BtnColor, BtnSize, BtnShape) {
        let mut use_color = BtnColor::Default;
        let mut use_size = BtnSize::Normal;
        let mut use_shape = BtnShape::Default;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    // Check for colors
                    "is-primary" => use_color = BtnColor::Primary,
                    "is-primary-dark" => use_color = BtnColor::PrimaryDark,
                    "is-secondary" => use_color = BtnColor::Secondary,
                    "is-danger" => use_color = BtnColor::Danger,
                    "is-danger-dark" => use_color = BtnColor::DangerDark,
                    "is-success" => use_color = BtnColor::Success,
                    "is-success-dark" => use_color = BtnColor::SuccessDark,
                    "is-warning" => use_color = BtnColor::Warning,
                    "is-warning-dark" => use_color = BtnColor::WarningDark,
                    "is-info" => use_color = BtnColor::Info,
                    "is-info-dark" => use_color = BtnColor::InfoDark,

                    // Check for sizes
                    "is-small" => use_size = BtnSize::Small,
                    "is-large" => use_size = BtnSize::Large,
                    "is-normal" => use_size = BtnSize::Normal,

                    // check for shapes
                    "is-round" => use_shape = BtnShape::Round,
                    "is-rectangle" => use_shape = BtnShape::Rectangle,

                        _ => (),
                }
            }
        }
        (use_color, use_size, use_shape)
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn build(&mut self) -> Entity {
        let (color, size, shape) = self._process_built_in_classes();
        FaButton::new(
            self.id.clone(),
            self.class.clone(),
            self.text.as_str(),
            &mut self.root_node,
            self.font_handle.clone(),
            color,
            size,
            shape
        )
    }
}

pub fn fa_button<'a>(builder: &'a mut FamiqWidgetBuilder, text: &str) -> FaButtonBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    builder.resource.can_run_systems.button = true;

    FaButtonBuilder::new(
        text.to_string(),
        font_handle,
        builder.ui_root_node.reborrow(),
    )
}

pub fn can_run_button_systems(builder_res: Res<FamiqWidgetResource>) -> bool {
    builder_res.can_run_systems.button
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use bevy::asset::AssetPlugin;
    use super::*;

    fn create_test_app() -> App {
        let mut app = App::new();
        // Note the use of `MinimalPlugins` instead of `DefaultPlugins`, as described above.
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Font>();
        // Spawning a fake window allows testing systems that require a window.
        app.world_mut().spawn(Window::default());
        app
    }

    fn setup_test_default_button(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_button(&mut builder, "Press me").id("#test-btn").build();
    }

    fn setup_test_button_with_built_in_class(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_button(&mut builder, "Press me")
            .id("#test-btn")
            .class("is-primary is-large is-round")
            .build();
    }

    #[test]
    fn test_create_default_button() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_button);
        app.update();

        let btn_q = app.world_mut().query::<(&FamiqWidgetId, &IsFamiqButton)>().get_single(app.world());
        assert!(btn_q.is_ok(), "There should be only 1 button");

        let btn_id = btn_q.unwrap().0;
        assert_eq!(
            "#test-btn".to_string(),
            btn_id.0
        );

        let btn_text_q = app.world_mut().query::<(&Text, &IsFamiqButtonText)>()
                        .get_single(app.world());

        assert_eq!(
            "Press me".to_string(),
            btn_text_q.unwrap().0.0
        );
    }

    #[test]
    fn test_create_button_with_built_in_class() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_button_with_built_in_class);
        app.update();

        let btn_q = app.world_mut().query::<(&FamiqWidgetClasses, &IsFamiqButton)>().get_single(app.world());
        assert_eq!(
            "is-primary is-large is-round".to_string(),
            btn_q.unwrap().0.0
        );
    }
}
