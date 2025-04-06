use crate::event_writer::*;
use crate::plugin::{CursorType, CursorIcons};
use crate::widgets::*;
use crate::utils::{_change_cursor_icon, get_color, insert_id_and_class, process_spacing_built_in_class};
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
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands,
        size: TextSize
    ) -> Entity {
        let txt = Text::new(text);

        let mut txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: 16.0,
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
                IsFamiqMainWidget,
                style_components.clone(),
                DefaultWidgetEntity::from(style_components),
                ReactiveText(text.to_string())
            ))
            .observe(FaText::handle_on_mouse_over)
            .observe(FaText::handle_on_mouse_out)
            .observe(FaText::handle_on_mouse_down)
            .observe(FaText::handle_on_mouse_up)
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
        let text = Self::_build_text(attributes, text, root_node, size);

        if !attributes.bind_keys.is_empty() {
            root_node
                .commands().entity(text).insert(ReactiveKeys(attributes.bind_keys.to_owned()));
        }
        text
    }

    fn handle_on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        text_q: Query<Option<&FamiqWidgetId>,  With<IsFamiqText>>,
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
        text_q: Query<Option<&FamiqWidgetId>,  With<IsFamiqText>>,
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
        text_q: Query<Option<&FamiqWidgetId>,  With<IsFamiqText>>,
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
        text_q: Query<Option<&FamiqWidgetId>,  With<IsFamiqText>>,
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
        $builder:expr,
        text: $text:expr
        $(, $($rest:tt)+)?
    ) => {{
        let mut text = fa_text_builder($builder, $text);
        $(
            $crate::fa_text_attributes!(text, $($rest)+);
        )?
        text.build()
    }};
}

#[macro_export]
macro_rules! fa_text_attributes {
    ($text:ident, id: $id:expr $(, $($rest:tt)+)?) => {{
        $text = $text.id($id);
        $(
            $crate::fa_text_attributes!($text, $($rest)+);
        )?
    }};

    ($text:ident, class: $class:expr $(, $($rest:tt)+)?) => {{
        $text = $text.class($class);
        $(
            $crate::fa_text_attributes!($text, $($rest)+);
        )?
    }};

    ($text:ident, color: $color:expr $(, $($rest:tt)+)?) => {{
        $text = $text.color($color);
        $(
            $crate::fa_text_attributes!($text, $($rest)+);
        )?
    }};

    ($text:ident, display: $display:expr $(, $($rest:tt)+)?) => {{
        $text = $text.display($display);
        $(
            $crate::fa_text_attributes!($text, $($rest)+);
        )?
    }};

    ($text:ident, bind: $bind:expr $(, $($rest:tt)+)?) => {{
        $text = $text.bind($bind);
        $(
            $crate::fa_text_attributes!($text, $($rest)+);
        )?
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
        fa_text!(&mut builder, text: "Test Text", id: "#test-text");
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
}
