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
                ExternalStyleHasChanged(false)
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
