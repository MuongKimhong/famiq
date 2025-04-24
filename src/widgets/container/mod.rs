pub mod tests;
pub mod base_container;
pub use base_container::*;

use bevy::prelude::*;
use macros::set_widget_attributes;

// use crate::extract_children;
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
    pub root_node: Entity
}

impl ContainerBuilder {
    pub fn new(root_node: Entity) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            children: Vec::new(),
            root_node
        }
    }

    fn on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&WidgetId>, With<MainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Container, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&WidgetId>, With<MainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Container, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&WidgetId>, With<MainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Container, trigger.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Container, trigger.entity(), id);
            }
        }
        trigger.propagate(false);
    }

    fn on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&WidgetId>, With<MainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Container, trigger.entity(), id);
        }
        trigger.propagate(false);
    }
}

impl SetupWidget for ContainerBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqContainer, IsFamiqContainableWidget, MainWidget, ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_container_node();

        if self.cloned_attrs.color == WidgetColor::Default {
            self.cloned_attrs.color = WidgetColor::Transparent;
        }
        self.cloned_attrs.default_visibility = Visibility::Visible;
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut base_container = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let container_entity = base_container.build(r_data, commands);

        commands
            .entity(container_entity)
            .insert(self.components())
            .add_children(&self.children)
            .observe(ContainerBuilder::on_mouse_up)
            .observe(ContainerBuilder::on_mouse_down)
            .observe(ContainerBuilder::on_mouse_over)
            .observe(ContainerBuilder::on_mouse_out);

        commands.entity(self.root_node).add_child(container_entity);

        insert_class_id(commands, container_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                all_reactive_keys,
                container_entity,
                WidgetBuilder {
                    builder: BuilderType::Container(cloned_builder)
                }
            ));
        });

        container_entity
    }

    fn build_with_world(&mut self, r_data: &HashMap<String, RVal>, world: &mut World) -> Option<Entity> {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_container_node();

        if self.cloned_attrs.color == WidgetColor::Default {
            self.cloned_attrs.color = WidgetColor::Transparent;
        }
        self.cloned_attrs.default_visibility = Visibility::Visible;
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let mut base_container = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let container_entity = base_container.build_with_world(r_data, world);


        world
            .entity_mut(container_entity.unwrap())
            .insert(self.components())
            .add_children(&self.children)
            .observe(ContainerBuilder::on_mouse_up)
            .observe(ContainerBuilder::on_mouse_down)
            .observe(ContainerBuilder::on_mouse_over)
            .observe(ContainerBuilder::on_mouse_out);

        world.entity_mut(self.root_node).add_child(container_entity.unwrap());

        insert_class_id_world(world, container_entity.unwrap(), &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            all_reactive_keys,
            container_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::Container(cloned_builder)
            }
        ));

        container_entity
    }
}

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
        famiq_builder.containable_children.insert(entity, children_vec);
        entity
    }};
}

#[macro_export]
macro_rules! container_attributes {
    // skip children
    ($c_builder:ident, children: $children_vec:tt) => {{}};

    // common attributes
    ($c_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($c_builder, $key : $value);
    }};
}

pub fn can_run_container_systems(q: Query<&IsFamiqContainer>) -> bool {
    !q.is_empty()
}
