pub mod tests;
pub mod systems;

use bevy::prelude::*;
use famiq_macros::set_widget_attributes;

use crate::widgets::container::base_container::*;
use crate::event_writer::*;
use crate::utils::*;
use crate::widgets::*;

use systems::*;

/// Marker component identifyijng Famiq Image widget.
#[derive(Component)]
pub struct IsFamiqImage;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ImageBuilder {
    pub path: String,
    pub all_reactive_keys: Vec<String>
}

impl ImageBuilder {
    pub fn new(path: String) -> Self {
        Self {
            path,
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            all_reactive_keys: Vec::new(),
        }
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) -> String {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);

        let reactive_keys = get_reactive_key(&self.path);
        let parsed_path = replace_reactive_keys(&self.path, &reactive_keys, r_data);
        self.all_reactive_keys.extend_from_slice(&reactive_keys);
        parsed_path
    }
}

impl SetupWidget for ImageBuilder {
    fn components(&mut self) -> impl Bundle {
        ( IsFamiqImage, MainWidget, ReactiveWidget )
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        let parsed_path = self.prepare_attrs(r_data);
        let mut image = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let image_entity = image.build(r_data, commands);

        commands
            .entity(image_entity)
            .insert(self.components())
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, image_entity);
        }
        insert_class_id(commands, image_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            let image_handle: Handle<Image> = w.resource::<AssetServer>().load(parsed_path);
            w.entity_mut(image_entity).insert(ImageNode::new(image_handle));
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                image_entity,
                WidgetBuilder {
                    builder: BuilderType::Image(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        image_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        let parsed_path = self.prepare_attrs(r_data);
        let mut image = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        image.rebuild(r_data, old_entity, world);

        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let image_handle: Handle<Image> = world.resource::<AssetServer>().load(parsed_path);
        world.entity_mut(old_entity).insert(ImageNode::new(image_handle));

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Image(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

/// Macro for creating an image.
#[macro_export]
macro_rules! image {
    ( path: $path:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let i_builder = &mut ImageBuilder::new($path.to_string());
        $(
            $crate::image_attributes!(i_builder, $key : $value);
        )*
        i_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! image_attributes {
    ($i_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($i_builder, $key : $value);
    }};
}

/// a system to check if Image internal system(s) can run.
///
/// True only if image widget is created.
pub fn can_run_image_systems(image_q: Query<&IsFamiqImage>) -> bool {
    !image_q.is_empty()
}
