use super::color::WHITE_COLOR;
use crate::widgets::{
    DefaultTextEntity, FamiqWidgetId, DefaultWidgetEntity,
    FamiqBuilder, BaseStyleComponents
};
use crate::utils::{process_spacing_built_in_class, insert_id_and_class};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::hashbrown::HashSet;

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

    /// entity-value pair
    pub entity_value: HashMap<Entity, String>,

    /// Tracks which text IDs changed
    pub changed_texts: HashSet<String>,

    /// Tracks which entities' text changed
    pub changed_entities: HashSet<Entity>,
}

impl FaTextResource {
    /// Update fa_text's value by id
    pub fn update_value_by_id(&mut self, id: &str, new_value: &str) {
        if let Some(existing) = self.text_value.get(id) {
            if existing == new_value {
                return;
            }
        }
        self.text_value.insert(id.to_string(), new_value.to_string());
        self.changed_texts.insert(id.to_string()); // Mark as changed
    }

    /// Update fa_text's value by entity
    pub fn update_value_by_entity(&mut self, entity: Entity, new_value: &str) {
        if let Some(existing) = self.entity_value.get(&entity) {
            if existing == new_value {
                return;
            }
        }
        self.entity_value.insert(entity, new_value.to_string());
        self.changed_entities.insert(entity); // Mark as changed
    }

    /// Get `fa_text` value by id, return empty string if id doesn't exist.
    pub fn get_value_by_id(&self, id: &str) -> String {
        if let Some(v) = self.text_value.get(id) {
            return v.clone();
        }
        String::new()
    }

    /// Get `fa_text` value by entity, return emtpty string if entity doesn't exist.
    pub fn get_value_by_entity(&self, entity: Entity) -> String {
        if let Some(v) = self.entity_value.get(&entity) {
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

        let mut node = _default_text_container_node();
        process_spacing_built_in_class(&mut node, &class);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = node;

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

        insert_id_and_class(root_node, entity, id, class);
        entity
    }

    pub fn new(
        id: Option<String>,
        text: &str,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: TextSize
    ) -> Entity {
        Self::_build_text(&id, &class, text, root_node, font_handle, size)
    }

    /// Internal system that reads `FaTextResource` and update the corresponding text widget's value
    pub fn update_text_value_system(
        mut text_q: Query<(&mut Text, Entity, Option<&FamiqWidgetId>), With<IsFamiqText>>,
        mut text_res: ResMut<FaTextResource>
    ) {
        if text_res.is_changed() {
            for (mut text, entity, id) in text_q.iter_mut() {
                // Check by id
                if let Some(id) = id {
                    if text_res.changed_texts.contains(&id.0) {
                        if let Some(value) = text_res.text_value.get(&id.0) {
                            text.0 = value.clone();
                        }
                    }
                }

                // Check by entity
                if text_res.changed_entities.contains(&entity) {
                    if let Some(value) = text_res.entity_value.get(&entity) {
                        text.0 = value.clone();
                    }
                }
            }

            // Clear changed lists after updates
            text_res.changed_texts.clear();
            text_res.changed_entities.clear();
        }
    }

    /// Internal system to insert text value into `FaTextResource` after created.
    pub fn detect_new_text_widget_system(
        text_q: Query<(Entity, &Text, Option<&FamiqWidgetId>), Added<IsFamiqText>>,
        mut text_res: ResMut<FaTextResource>
    ) {
        for (entity, text, id) in text_q.iter() {

            if let Some(id) = id {
                if !text_res.text_value.contains_key(id.0.as_str()) {
                    text_res.text_value.insert(id.0.clone(), text.0.clone());
                }
            }

            if !text_res.entity_value.contains_key(&entity) {
                text_res.entity_value.insert(entity, text.0.clone());
            }
        }
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
pub fn fa_text<'a>(builder: &'a mut FamiqBuilder, value: &str) -> FaTextBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
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
    use crate::widgets::FamiqResource;
    use super::*;

    #[derive(Resource)]
    struct TestResource(Entity);

    fn setup_test_default_text(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqResource>,
    ) {
        let mut builder = FamiqBuilder::new(&mut commands, &mut builder_res, &asset_server);
        let text = fa_text(&mut builder, "Test Text").id("#test-text").build();
        commands.insert_resource(TestResource(text));
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

        text_res.update_value_by_id("#test-text", "New test text Hello World");

        app.update(); // update again so update_text_value_system run again

        let txt_q = app.world_mut()
            .query::<(&Text, &IsFamiqText)>()
            .get_single(app.world());

        let txt = txt_q.as_ref().unwrap().0;

        // original text is "Test Text"
        assert_eq!("New test text Hello World".to_string(), txt.0);
    }

    #[test]
    fn test_update_text_value_by_entity() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FaTextResource::default());
        app.add_systems(Startup, setup_test_default_text);
        app.add_systems(Update, FaText::update_text_value_system); // internal system that handle updating the text
        app.update();

        let text_entity = app.world_mut().resource::<TestResource>().0;
        let mut text_res = app.world_mut().resource_mut::<FaTextResource>();
        text_res.update_value_by_entity(text_entity, "New test text Hello World");

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

        let value = text_res.get_value_by_id("#random-id");

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

        let value = text_res.get_value_by_id("#test-text");

        assert_eq!(String::from("Test Text"), value);
    }
}
