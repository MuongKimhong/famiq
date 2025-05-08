pub mod base_text;
pub mod systems;
pub(crate) use base_text::*;
use systems::*;

use crate::event_writer::*;
use crate::plugin::{CursorType, CursorIcons};
use crate::widgets::*;
use crate::utils::*;
use famiq_macros::set_widget_attributes;
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

fn default_text_container_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct TextBuilder {
    pub value: String,
    pub all_reactive_keys: Vec<String>
}

impl TextBuilder {
    pub fn new(value: String, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            value,
            attributes,
            cloned_attrs: WidgetAttributes::default(),
            all_reactive_keys: Vec::new()
        }
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) -> String {
        self.cloned_attrs = self.attributes.clone();
        let reactive_keys = get_reactive_key(&self.value);
        let parsed_text = replace_reactive_keys(&self.value, &reactive_keys, r_data);
        self.all_reactive_keys.extend_from_slice(&reactive_keys);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
        parsed_text
    }
}

impl SetupWidget for TextBuilder {
    fn components(&mut self) -> impl Bundle {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_text_container_node();
        self._process_built_in_spacing_class();
        (
            MainWidget,
            IsFamiqText,
            ReactiveWidget,
            style_components.clone(),
            DefaultWidgetConfig::from(style_components)
        )
    }

    fn build(
        &mut self,
        r_data: &HashMap<String, RVal>,
        commands: &mut Commands
    ) -> Entity {
        let parsed_text = self.prepare_attrs(r_data);
        let mut base_text = FaBaseText::new_with_attributes(&parsed_text, &self.cloned_attrs);
        base_text.use_get_color = true;
        let text_entity = base_text.build(r_data, commands);
        commands
            .entity(text_entity)
            .insert(self.components())
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        insert_class_id(commands, text_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                text_entity,
                WidgetBuilder {
                    builder: BuilderType::Text(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        text_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        let parsed_text = self.prepare_attrs(r_data);
        let mut base_text = FaBaseText::new_with_attributes(&parsed_text, &self.cloned_attrs);
        base_text.use_get_color = true;
        base_text.rebuild(r_data, old_entity, world);

        world
            .entity_mut(old_entity)
            .insert(self.components());

        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Text(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

/// Macro for creating a text.
#[macro_export]
macro_rules! text {
    ( text: $text:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let text_builder = &mut TextBuilder::new($text.to_string(), &famiq_builder.get_font_handle());
        $(
            $crate::text_attributes!(text_builder, $key: $value);
        )*
        text_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! text_attributes {
    ($text_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($text_builder, $key : $value);
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
        FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
        text!(text: "Test Text", id: "#test-text");
    }

    #[test]
    fn test_create_default_text() {
        let mut app = create_test_app();
        app.add_plugins(FamiqPlugin);
        app.add_systems(Startup, setup_test_default_text);
        app.update();

        let txt_q = app.world_mut()
            .query::<(&WidgetId, &Text, &IsFamiqText)>()
            .single(app.world());

        let id = txt_q.as_ref().unwrap().0;
        let txt = txt_q.as_ref().unwrap().1;

        assert_eq!("#test-text".to_string(), id.0);
        assert_eq!("Test Text".to_string(), txt.0);
    }
}
