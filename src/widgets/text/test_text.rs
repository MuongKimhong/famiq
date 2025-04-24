use bevy::prelude::*;
use macros::set_widget_attributes;
use super::*;

#[set_widget_attributes]
pub struct TextBuilder {
    pub value: String
}

impl TextBuilder {
    pub fn new(value: String, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            value,
            attributes
        }
    }

    fn on_mouse_over(
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

    fn on_mouse_out(
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

    fn on_mouse_down(
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

    fn on_mouse_up(
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

impl SetupWidget for TextBuilder {
    fn components(&mut self) -> impl Bundle {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_text_container_node();
        process_spacing_built_in_class(&mut style_components.node, &self.attributes.class);
        (
            MainWidget,
            IsFamiqText,
            WidgetType::Text,
            ReactiveWidget,
            style_components.clone(),
            DefaultWidgetConfig::from(style_components)
        )
    }

    fn build(&mut self, commands: &mut Commands) -> Entity {
        let mut base_text = FaBaseText::new_with_attributes(
            &self.value,
            &self.attributes
        );
        base_text.use_get_color = true;
        let text_entity = base_text.build(commands);

        commands
            .entity(text_entity)
            .insert(self.components())
            .observe(TextBuilder::on_mouse_up)
            .observe(TextBuilder::on_mouse_down)
            .observe(TextBuilder::on_mouse_over)
            .observe(TextBuilder::on_mouse_out);

        insert_class_id(commands, text_entity, &self.attributes.id, &self.attributes.class);
        text_entity
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TextFields {
    pub text: String,
    pub common: CommonMacroFields
}

impl TextFields {
    pub fn new_with_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            common: CommonMacroFields::default()
        }
    }
}

pub struct TextContext<'a> {
    pub builder: &'a mut TextBuilder,
    pub fields: &'a mut TextFields,
    pub reactive_data: &'a HashMap<String, RVal>,
    pub all_reactive_keys: &'a mut Vec<&'a str>
}

#[macro_export]
macro_rules! text {
    ( text: $text:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let reactive_data = famiq_builder.reactive_data.data.clone();
        let mut fields = TextFields::new_with_text($text);

        let reactive_keys = extract_reactive_key($text);

        let mut all_r_keys: Vec<&str> = Vec::new();
        all_r_keys.extend_from_slice(&reactive_keys);

        let parsed_text = replace_text_with_reactive_keys(
            $text,
            &reactive_keys,
            &reactive_data
        );
        let ctx = TextContext {
            builder: &mut TextBuilder::new(parsed_text, &famiq_builder.get_font_handle()),
            fields: &mut fields,
            reactive_data: &reactive_data,
            all_reactive_keys: &mut all_r_keys,
        };
        $(
            $crate::text_attributes!(ctx, $key : $value);
        )*
        ctx.all_reactive_keys.sort();
        ctx.all_reactive_keys.dedup();

        let entity = ctx.builder.build(&mut famiq_builder.ui_root_node.commands());
        let serialized_fields = serde_json::to_string(&ctx.fields).unwrap();
        let keys: Vec<String> = ctx.all_reactive_keys.iter().map(|k| k.to_string()).collect();

        famiq_builder.ui_root_node.commands().queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(keys, entity, serialized_fields.clone()));
        });
        entity
    }};
}

#[macro_export]
macro_rules! text_attributes {
    // common attributes
    ($ctx:ident, $key:ident : $value:expr) => {{
        $crate::test_common_attributes!($ctx, $key : $value);
    }};
}
