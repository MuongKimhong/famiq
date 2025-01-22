use super::color::WHITE_COLOR;
use crate::widgets::{DefaultTextEntity, FamiqWidgetId, FamiqWidgetClasses, FamiqWidgetBuilder};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct IsFamiqText;

// Needs container
pub struct FaText;

impl<'a> FaText {
    pub fn new(
        id: &str,
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
                FamiqWidgetId(id.to_string()),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                Interaction::default()
            ))
            .id();

        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        entity
    }
}

pub struct FaTextBuilder<'a> {
    pub id: String,
    pub value: String,
    pub class: Option<String>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaTextBuilder<'a> {
    pub fn new(id: String, value: String, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        Self {
            id,
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

    pub fn build(&mut self) -> Entity {
        FaText::new(
            self.id.as_str(),
            self.value.as_str(),
            self.class.clone(),
            &mut self.root_node,
            self.font_handle.clone(),
        )
    }
}

pub fn fa_text<'a>(builder: &'a mut FamiqWidgetBuilder, id: &str, value: &str) -> FaTextBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaTextBuilder::new(
        id.to_string(),
        value.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}
