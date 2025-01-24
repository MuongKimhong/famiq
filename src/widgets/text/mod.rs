use super::color::WHITE_COLOR;
use crate::widgets::{
    DefaultTextEntity, FamiqWidgetId, FamiqWidgetClasses,
    FamiqWidgetBuilder, WidgetStyle, ExternalStyleHasChanged
};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqText;

// Needs container
pub struct FaText;

impl<'a> FaText {
    pub fn new(
        id: Option<String>,
        value: &str,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>
    ) -> Entity {
        let txt = Text::new(value);
        let txt_font = TextFont {
            font: font_handle,
            ..default()
        };
        let txt_color = TextColor(WHITE_COLOR);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                Interaction::default(),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false),
                IsFamiqText
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        entity
    }
}

pub struct FaTextBuilder<'a> {
    pub id: Option<String>,
    pub value: String,
    pub class: Option<String>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaTextBuilder<'a> {
    pub fn new(value: String, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            value,
            class: None,
            font_handle,
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

    pub fn build(&mut self) -> Entity {
        FaText::new(
            self.id.clone(),
            self.value.as_str(),
            self.class.clone(),
            &mut self.root_node,
            self.font_handle.clone(),
        )
    }
}

pub fn fa_text<'a>(builder: &'a mut FamiqWidgetBuilder, value: &str) -> FaTextBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaTextBuilder::new(
        value.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::utils::create_test_app;
    use crate::widgets::FamiqWidgetResource;
    use super::*;

    fn setup_test_default_text(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_text(&mut builder, "Test Text").id("#test-text").build();
    }

    #[test]
    fn test_create_default_text() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_text);
        app.update();

        let txt_q = app.world_mut()
            .query::<(&FamiqWidgetId, &Text, &IsFamiqText)>()
            .get_single(app.world());

        let id = txt_q.as_ref().unwrap().0;
        let txt = txt_q.as_ref().unwrap().1;

        assert_eq!("#test-text".to_string(), id.0);
        assert_eq!("Test Text".to_string(), txt.0);
    }
}
