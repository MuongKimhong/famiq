use super::color::WHITE_COLOR;
use crate::widgets::{
    DefaultTextEntity, FamiqWidgetId, FamiqWidgetClasses, DefaultWidgetEntity,
    FamiqWidgetBuilder, WidgetStyle, ExternalStyleHasChanged
};
use crate::utils::{entity_add_child, process_spacing_built_in_class};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

/// Marker component for identifying Famiq text widgets.
#[derive(Component)]
pub struct IsFamiqText;

/// Marker component for identifying Famiq text container.
#[derive(Component)]
pub struct IsFamiqTextContainer;

pub enum TextSize {
    Default,
    TitleH1,
    TitleH2,
    TitleH3,
    TitleH4,
    TitleH5,
    TitleH6,
}

/// Represents a Famiq text widget for displaying styled text.
pub struct FaText;

fn _default_text_container_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

impl<'a> FaText {
    fn _build_text(
        id: &Option<String>,
        class: &Option<String>,
        text: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: TextSize
    ) -> Entity {
        let txt = Text::new(text);
        let mut txt_font = TextFont {
            font: font_handle,
            ..default()
        };
        match size {
            TextSize::TitleH1 => txt_font.font_size = 40.0,
            TextSize::TitleH2 => txt_font.font_size = 32.0,
            TextSize::TitleH3 => txt_font.font_size = 28.0,
            TextSize::TitleH4 => txt_font.font_size = 24.0,
            TextSize::TitleH5 => txt_font.font_size = 20.0,
            TextSize::TitleH6 => txt_font.font_size = 16.0,
            _ => {}
        }

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
            root_node.commands().entity(entity).insert(FamiqWidgetId(id.to_owned()));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class.to_owned()));
        }
        entity
    }

    fn _build_container(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let mut node = _default_text_container_node();
        process_spacing_built_in_class(&mut node, &class);

        let border_color = BorderColor::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;
        let border_radius = BorderRadius::default();

        let container_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                bg_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqTextContainer,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(container_entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(container_entity).insert(FamiqWidgetClasses(class));
        }
        container_entity
    }

    pub fn new(
        id: Option<String>,
        text: &str,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: TextSize
    ) -> Entity {
        let txt_entity = Self::_build_text(&id, &class, text, root_node, font_handle, size);
        let container = Self::_build_container(id, class, root_node);
        entity_add_child(root_node, txt_entity, container);
        container
    }
}

/// Builder for creating `FaText` entities with customizable options.
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

    fn _process_built_in_size_class(&self) -> TextSize {
        let mut use_size = TextSize::Default;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "h1" => use_size = TextSize::TitleH1,
                    "h2" => use_size = TextSize::TitleH2,
                    "h3" => use_size = TextSize::TitleH3,
                    "h4" => use_size = TextSize::TitleH4,
                    "h5" => use_size = TextSize::TitleH5,
                    _ => {}
                }
            }
        }
        use_size
    }

    /// Method to add class to text.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to text.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Spawn text into UI World.
    pub fn build(&mut self) -> Entity {
        let size = self._process_built_in_size_class();
        FaText::new(
            self.id.clone(),
            self.value.as_str(),
            self.class.clone(),
            &mut self.root_node,
            self.font_handle.clone(),
            size
        )
    }
}

/// API to create `FaTextBuilder`.
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
