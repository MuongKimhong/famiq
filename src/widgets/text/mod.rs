use crate::event_writer::FaInteractionEvent;
use crate::plugin::{CursorType, CursorIcons};
use crate::widgets::*;
use crate::utils::{_change_cursor_icon, get_color, insert_id_and_class, process_spacing_built_in_class};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::hashbrown::{HashSet, HashMap};

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

/// Resource use to store & update fa_text's value by id
#[derive(Resource, Default, Debug)]
pub struct FaTextResource {
    /// id-value
    pub text_value: HashMap<String, String>,

    /// Tracks which text IDs changed
    pub changed_texts: HashSet<String>
}

impl FaTextResource {
    /// Update fa_text's value by id
    pub fn update_value(&mut self, id: &str, new_value: &str) {
        if let Some(existing) = self.text_value.get(id) {
            if existing == new_value {
                return;
            }
        }
        self.text_value.insert(String::from(id), String::from(new_value));
        self.changed_texts.insert(String::from(id));
    }

    /// Get `fa_text` value by id, return empty string if id doesn't exist.
    pub fn get_value(&self, id: &str) -> String {
        if let Some(v) = self.text_value.get(id) {
            return v.clone();
        }
        String::new()
    }
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
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        size: TextSize
    ) -> Entity {
        let txt = Text::new(text);
        let mut txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
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

        let txt_color = TextColor(get_color(&attributes.color));
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqText,
                style_components.clone(),
                DefaultWidgetEntity::from(style_components)
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        size: TextSize
    ) -> Entity {
        Self::_build_text(attributes, text, root_node, size)
    }

    /// Internal system that reads `FaTextResource` and update the corresponding text widget's value
    pub fn update_text_value_system(
        mut text_q: Query<(&mut Text, Option<&FamiqWidgetId>), With<IsFamiqText>>,
        mut text_res: ResMut<FaTextResource>
    ) {
        if text_res.is_changed() {
            for (mut text, id) in text_q.iter_mut() {
                // Check by id
                if let Some(id) = id {
                    if text_res.changed_texts.contains(&id.0) {
                        if let Some(value) = text_res.text_value.get(&id.0) {
                            text.0 = value.clone();
                        }
                    }
                }
            }
            text_res.changed_texts.clear();
        }
    }

    /// Internal system to insert text value into `FaTextResource` after created.
    pub fn detect_new_text_widget_system(
        text_q: Query<(&Text, Option<&FamiqWidgetId>), Added<IsFamiqText>>,
        mut text_res: ResMut<FaTextResource>
    ) {
        for (text, id) in text_q.iter() {

            if let Some(id) = id {
                if !text_res.text_value.contains_key(id.0.as_str()) {
                    text_res.text_value.insert(id.0.clone(), text.0.clone());
                }
            }
        }
    }

    pub fn handle_text_interaction_system(
        mut events: EventReader<FaInteractionEvent>,
        window: Single<Entity, With<Window>>,
        mut commands: Commands,
        cursor_icons: Res<CursorIcons>,
    ) {
        for e in events.read() {
            if e.widget == WidgetType::Text {
                match e.interaction {
                    Interaction::None => {
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
                    },
                    _ => {
                        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
                    }
                }
            }
        }
    }
}

/// Builder for creating `FaText` entities with customizable options.
pub struct FaTextBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub value: String,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaTextBuilder<'a> {
    pub fn new(value: String, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            value,
            root_node
        }
    }

    fn _process_built_in_text_size_class(&self) -> TextSize {
        let mut use_size = TextSize::Default;

        if let Some(class) = self.attributes.class.as_ref() {
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

    /// Spawn text into UI World.
    pub fn build(&mut self) -> Entity {
        self._process_built_in_color_class();
        self._node();
        let size = self._process_built_in_text_size_class();
        FaText::new(
            &self.attributes,
            self.value.as_str(),
            &mut self.root_node,
            size
        )
    }
}

impl<'a> SetWidgetAttributes for FaTextBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = _default_text_container_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaTextBuilder`.
pub fn fa_text<'a>(builder: &'a mut FamiqBuilder, value: &str) -> FaTextBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaTextBuilder::new(
        value.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_text_systems(text_q: Query<&IsFamiqText>) -> bool {
    !text_q.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::utils::create_test_app;
    use crate::widgets::FamiqResource;
    use super::*;

    fn setup_test_default_text(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut builder_res: ResMut<FamiqResource>,
    ) {
        let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_text(&mut builder, "Test Text").id("#test-text").build();
    }

    #[test]
    fn test_create_default_text() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
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

    #[test]
    fn test_update_text_value_by_id() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FaTextResource::default());
        app.add_systems(Startup, setup_test_default_text);
        app.add_systems(Update, FaText::update_text_value_system); // internal system that handle updating the text
        app.update();

        let mut text_res = app.world_mut().resource_mut::<FaTextResource>();

        text_res.update_value("#test-text", "New test text Hello World");

        app.update(); // update again so update_text_value_system run again

        let txt_q = app.world_mut()
            .query::<(&Text, &IsFamiqText)>()
            .get_single(app.world());

        let txt = txt_q.as_ref().unwrap().0;

        // original text is "Test Text"
        assert_eq!("New test text Hello World".to_string(), txt.0);
    }

    #[test]
    fn test_get_value_by_non_exist_id() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FaTextResource::default());
        app.add_systems(Startup, setup_test_default_text);
        app.add_systems(Update, FaText::update_text_value_system); // internal system that handle updating the text
        app.update();

        let text_res = app.world_mut().resource::<FaTextResource>();

        let value = text_res.get_value("#random-id");

        assert_eq!(String::new(), value);
    }

    #[test]
    fn test_get_value_by_id() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FaTextResource::default());
        app.add_systems(Startup, setup_test_default_text);
        app.add_systems(Update, FaText::update_text_value_system); // internal system that handle updating the text
        app.update();

        let text_res = app.world_mut().resource::<FaTextResource>();

        let value = text_res.get_value("#test-text");

        assert_eq!(String::from("Test Text"), value);
    }
}
