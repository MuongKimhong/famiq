pub mod tests;
pub mod base_container;
pub mod systems;

pub(crate) use base_container::*;
use systems::*;

use bevy::prelude::*;
use macros::set_widget_attributes;

use crate::widgets::*;
use crate::event_writer::*;

/// Marker component for identifying a Famiq container.
#[derive(Component)]
pub struct IsFamiqContainer;

pub fn default_container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart, // Align children at the top
        height: Val::Auto,
        border: UiRect::all(Val::Px(1.)),
        ..default()
    }
}

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ContainerBuilder {
    pub children: Vec<Entity>,
    pub all_reactive_keys: Vec<String>,
    pub root_node: Entity
}

impl ContainerBuilder {
    pub fn new(root_node: Entity) -> Self {
        Self {
            all_reactive_keys: Vec::new(),
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            children: Vec::new(),
            root_node
        }
    }

    pub(crate) fn prepare_atts(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_container_node();

        if self.cloned_attrs.color == WidgetColor::Default {
            self.cloned_attrs.color = WidgetColor::Transparent;
        }
        self.cloned_attrs.default_visibility = Visibility::Visible;
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
    }
}

impl SetupWidget for ContainerBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqContainer, IsFamiqContainableWidget, MainWidget, ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.prepare_atts(r_data);
        let mut base_container = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let container_entity = base_container.build(r_data, commands);

        commands
            .entity(container_entity)
            .insert(self.components())
            .add_children(&self.children)
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        commands.entity(self.root_node).add_child(container_entity);

        insert_class_id(commands, container_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                container_entity,
                WidgetBuilder {
                    builder: BuilderType::Container(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        container_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        self.prepare_atts(r_data);
        let mut base_container = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        base_container.rebuild(r_data, old_entity, world);

        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Container(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

/// Macro for creating a container.
#[macro_export]
macro_rules! container {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();

        #[allow(unused_mut)]
        let mut children_vec: Vec<Entity> = Vec::new();
        $(
            $crate::extract_children!(children_vec, $key : $value);
        )*
        let root_entity = famiq_builder.resource.root_node_entity.unwrap();
        let c_builder = &mut ContainerBuilder::new(root_entity);

        $(
            $crate::container_attributes!(c_builder, $key : $value);
        )*
        c_builder.children = children_vec.clone();
        let entity = c_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        );
        // famiq_builder.containable_children.insert(entity, children_vec);
        entity
    }};
}

#[macro_export]
macro_rules! container_attributes {
    // skip children
    ($c_builder:ident, children: $children_vec:tt) => {{}};

    ($c_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($c_builder, $key : $value);
    }};
}

pub fn can_run_container_systems(q: Query<&IsFamiqContainer>) -> bool {
    !q.is_empty()
}
