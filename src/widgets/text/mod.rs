use crate::event_writer::*;
use crate::plugin::{CursorType, CursorIcons};
use crate::widgets::*;
use crate::utils::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

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
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        size: TextSize,
        has_observer: bool,
        use_get_text_color: bool,
        text_layout: TextLayout
    ) -> Entity {
        let txt = Text::new(text);

        let mut txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
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

        let txt_color = match use_get_text_color {
            true => TextColor(get_text_color(&attributes.color)),
            false => TextColor(get_color(&attributes.color))
        };
        let mut temp_cmd = root_node.commands();
        let mut entity_cmd = temp_cmd
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                text_layout.clone(),
                DefaultTextConfig::new(txt, txt_font, txt_color, text_layout),
                IsFamiqText,
                ReactiveText(text.to_string()),
                WidgetType::Text,
                ReactiveWidget
            ));

        if has_observer {
            entity_cmd
                .observe(FaText::handle_on_mouse_over)
                .observe(FaText::handle_on_mouse_out)
                .observe(FaText::handle_on_mouse_down)
                .observe(FaText::handle_on_mouse_up);
        }
        let entity = entity_cmd.id();
        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        size: TextSize,
        has_node: bool,
        has_observer: bool,
        use_get_text_color: bool,
        text_layout: TextLayout
    ) -> Entity {
        let text = Self::_build_text(
            attributes,
            text,
            root_node,
            size,
            has_observer,
            use_get_text_color,
            text_layout
        );
        if !attributes.bind_keys.is_empty() {
            root_node
                .commands().entity(text).insert(ReactiveKeys(attributes.bind_keys.to_owned()));
        }
        if has_node {
            let mut style_components = BaseStyleComponents::default();
            style_components.node = attributes.node.clone();
            root_node.commands().entity(text).insert((
                MainWidget,
                style_components.clone(),
                DefaultWidgetConfig::from(style_components),
            ));
        }
        text
    }

    fn handle_on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        if let Ok(id) = text_q.get(trigger.entity()) {
            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Text);
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Text, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        if let Ok(id) = text_q.get(trigger.entity()) {
            _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Text, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut writer: EventWriter<FaMouseEvent>,
        text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
    ) {
        if let Ok(id) = text_q.get(trigger.entity()) {
            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Text, trigger.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Text, trigger.entity(), id);
            }
        }
        trigger.propagate(false);
    }

    fn handle_on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut writer: EventWriter<FaMouseEvent>,
        text_q: Query<Option<&WidgetId>,  With<IsFamiqText>>,
    ) {
        if let Ok(id) = text_q.get(trigger.entity()) {
            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Text, trigger.entity(), id);
            }
        }
        trigger.propagate(false);
    }
}

/// Builder for creating `FaText` entities with customizable options.
pub struct FaTextBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub value: String,
    pub root_node: EntityCommands<'a>,
    pub(crate) has_observer: bool,
    pub(crate) has_node: bool,
    pub(crate) use_get_text_color: bool,
    pub(crate) text_layout: TextLayout
}

impl<'a> FaTextBuilder<'a> {
    pub fn new(value: String, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            value,
            root_node,
            has_observer: true,
            has_node: true,
            use_get_text_color: false,
            text_layout: TextLayout::new_with_justify(JustifyText::Center)
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
        self._process_built_in_size_class();
        self._node();
        let size = self._process_built_in_text_size_class();
        FaText::new(
            &self.attributes,
            self.value.as_str(),
            &mut self.root_node,
            size,
            self.has_node,
            self.has_observer,
            self.use_get_text_color,
            self.text_layout
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
pub fn fa_text_builder<'a>(builder: &'a mut FamiqBuilder, value: &str) -> FaTextBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaTextBuilder::new(
        value.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

#[macro_export]
macro_rules! fa_text {
    (
        text: $text:expr
        $(, $key:ident : $value:tt )* $(,)?
    ) => {{
        let builder = builder_mut();
        let all_reactive_keys: &mut Vec<&str> = &mut Vec::new();
        all_reactive_keys.extend(extract_reactive_key($text));

        let mut text_builder = fa_text_builder(builder, $text);
        $(
            $crate::fa_text_attributes!(text_builder, $key : $value);
        )*
        text_builder.build()
    }};
}

#[macro_export]
macro_rules! fa_text_attributes {
    ($text:ident, use_get_text_color: $use_get_text_color:expr) => {{
        $text.use_get_text_color = $use_get_text_color;
    }};

    ($text:ident, has_observer: $has_observer:expr) => {{
        $text.has_observer = $has_observer;
    }};

    ($text:ident, has_node: $has_node:expr) => {{
        $text.has_node = $has_node;
    }};

    ($text:ident, text_layout: $text_layout:expr) => {{
        $text.text_layout = $text_layout;
    }};

    // common attributes
    ($text:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($text, $key : $value);
    }};
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FaTextFields {
    pub text: String,
    pub common: CommonMacroFields
}

impl FaTextFields {
    pub fn new_with_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            common: CommonMacroFields::default()
        }
    }
}

pub struct FaTextContext<'a> {
    pub builder: &'a mut FaTextBuilder<'a>,
    pub fields: &'a mut FaTextFields,
    pub reactive_data: &'a HashMap<String, RVal>,
    pub all_reactive_keys: &'a mut Vec<&'a str>
}

#[macro_export]
macro_rules! test_text {
    ( text: $text:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let builder = builder_mut();
        let reactive_data = builder.reactive_data.data.clone();
        let mut fields = FaTextFields::new_with_text($text);

        let reactive_keys = extract_reactive_key($text);

        let mut all_r_keys: Vec<&str> = Vec::new();
        all_r_keys.extend_from_slice(&reactive_keys);

        let parsed_text = replace_text_with_reactive_keys(
            $text,
            &reactive_keys,
            &reactive_data
        );
        let ctx = FaTextContext {
            builder: &mut fa_text_builder(builder, &parsed_text),
            fields: &mut fields,
            reactive_data: &reactive_data,
            all_reactive_keys: &mut all_r_keys,
        };
        $(
            $crate::test_text_attributes!(ctx, $key : $value);
        )*
        ctx.all_reactive_keys.sort();
        ctx.all_reactive_keys.dedup();

        let entity = ctx.builder.build();
        let serialized_fields = serde_json::to_string(&ctx.fields).unwrap();
        let keys: Vec<String> = ctx.all_reactive_keys.iter().map(|k| k.to_string()).collect();

        builder.ui_root_node.commands().queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(keys, entity, serialized_fields.clone()));
        });
        entity
    }};
}
#[macro_export]
macro_rules! test_text_attributes {
    ($ctx:ident, use_get_text_color: $value:expr) => {{
        $ctx.builder.use_get_text_color = $value;
    }};
    ($ctx:ident, has_observer: $value:expr) => {{
        $ctx.builder.has_observer = $value;
    }};
    ($ctx:ident, has_node: $value:expr) => {{
        $ctx.builder.has_node = $value;
    }};
    ($ctx:ident, text_layout: $value:expr) => {{
        $ctx.builder.text_layout = $value;
    }};

    // common attributes
    ($ctx:ident, $key:ident : $value:expr) => {{
        $crate::test_common_attributes!($ctx, $key : $value);
    }};
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
        mut famiq_res: ResMut<FamiqResource>,
        mut fa_query: FaQuery
    ) {
        let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
        inject_builder(&mut builder);
        fa_text!(text: "Test Text", id: "#test-text");
    }

    #[test]
    fn test_create_default_text() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.add_systems(Startup, setup_test_default_text);
        app.update();

        let txt_q = app.world_mut()
            .query::<(&WidgetId, &Text, &IsFamiqText)>()
            .get_single(app.world());

        let id = txt_q.as_ref().unwrap().0;
        let txt = txt_q.as_ref().unwrap().1;

        assert_eq!("#test-text".to_string(), id.0);
        assert_eq!("Test Text".to_string(), txt.0);
    }
}
