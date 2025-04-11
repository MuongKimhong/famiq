pub mod tests;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

// use crate::extract_children;
use crate::utils;
use crate::widgets::*;
use crate::event_writer::*;
use super::BaseStyleComponents;

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
        border: UiRect::all(Val::Px(10.)),
        ..default()
    }
}

/// Represents a Famiq container widget.
/// Think of it as a Div element in HTML.
pub struct FaContainer;

// Doesn't need container
impl<'a> FaContainer {
    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        children: &Vec<Entity>,
        has_observer: bool
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.visibility = Visibility::Visible;

        let mut temp_cmd = root_node.commands();

        let mut container = temp_cmd
            .spawn((
                style_components.clone(),
                IsFamiqMainWidget,
                DefaultWidgetEntity::from(style_components)
            ));

        if has_observer {
            container
                .insert((IsFamiqContainer, IsFamiqContainableWidget))
                .observe(FaContainer::on_mouse_over)
                .observe(FaContainer::on_mouse_out)
                .observe(FaContainer::on_mouse_down)
                .observe(FaContainer::on_mouse_up);
        }
        let entity = container.id();
        root_node.add_child(entity);
        utils::insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        utils::entity_add_children(root_node, children, entity);
        entity
    }

    fn on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&FamiqWidgetId>, With<IsFamiqMainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Container, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&FamiqWidgetId>, With<IsFamiqMainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Container, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut writer: EventWriter<FaMouseEvent>,
        q: Query<Option<&FamiqWidgetId>, With<IsFamiqMainWidget>>
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
        q: Query<Option<&FamiqWidgetId>, With<IsFamiqMainWidget>>
    ) {
        if let Ok(id) = q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Container, trigger.entity(), id);
        }
        trigger.propagate(false);
    }
}

/// Builder for creating `FaContainer` entities with customizable options.
pub struct FaContainerBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub children: Vec<Entity>,
    pub root_note: EntityCommands<'a>,
    pub(crate) has_observer: bool,
}

impl<'a> FaContainerBuilder<'a> {
    pub fn new(root_note: EntityCommands<'a>) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            children: Vec::new(),
            has_observer: true,
            root_note
        }
    }

    /// Sets the child entities for the container.
    ///
    /// # Parameters
    /// - `children`: An iterable collection of entities to add as children.
    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Spawn container into UI World
    pub fn build(&mut self) -> Entity {
        self._node();
        FaContainer::new(
            &self.attributes,
            &mut self.root_note.reborrow(),
            &self.children,
            self.has_observer
        )
    }
}

impl<'a> SetWidgetAttributes for FaContainerBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_container_node();

        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        utils::process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaContainerBuilder`
pub fn fa_container_builder<'a>(builder: &'a mut FamiqBuilder) -> FaContainerBuilder<'a> {
    FaContainerBuilder::new(
        builder.ui_root_node.reborrow()
    )
}

#[macro_export]
macro_rules! fa_container {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{

        let builder = builder_mut();

        #[allow(unused_mut)]
        let mut children_vec: Vec<Entity> = Vec::new();
        $(
            $crate::extract_children!(children_vec, builder, $key : $value);
        )*

        let mut container = fa_container_builder(builder);

        $(
            $crate::fa_container_attributes!(container, $key : $value);
        )*

        container = container.children(children_vec);
        container.build()
    }};
}

#[macro_export]
macro_rules! fa_container_attributes {
    ($container:ident, has_observer: $has_observer:expr) => {{
        $container.has_observer = $has_observer;
    }};
    // skip children
    ($container:ident, children: $children_vec:tt) => {{}};

    // common attributes
    ($container:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($container, $key : $value);
    }};
}

pub fn can_run_container_systems(q: Query<&IsFamiqContainer>) -> bool {
    !q.is_empty()
}
