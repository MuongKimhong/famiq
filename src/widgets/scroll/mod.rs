pub mod components;
pub mod systems;
pub mod tests;
pub(crate) use components::*;
pub(crate) use systems::*;

use crate::widgets::container::base_container::*;
use crate::event_writer::*;
use crate::widgets::*;
use crate::utils::*;

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use macros::set_widget_attributes;

// TODO: make scroll widget reactive

/// only entity inside this resource can be scrolled
#[derive(Resource)]
pub struct CanBeScrolled {
    pub entity: Option<Entity>
}

pub fn default_move_panel_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        padding: UiRect {
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(2.0),
            bottom: Val::Px(2.0)
        },
        ..default()
    }
}

pub fn default_scroll_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        height: Val::Percent(50.0),
        overflow: Overflow::scroll_y(),
        padding: UiRect::all(Val::Px(0.0)),
        border: UiRect::all(Val::Px(1.)),
        ..default()
    }
}


#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ScrollBuilder {
    pub all_reactive_keys: Vec<String>,
    pub children: Vec<Entity>,
    pub root_node: Entity,
    pub scroll_height: f32,
    pub panel_entity: Option<Entity>
}

impl ScrollBuilder {
    pub fn new(root_node: Entity) -> Self {
        Self {
            all_reactive_keys: Vec::new(),
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            children: Vec::new(),
            scroll_height: 15.0,
            panel_entity: None,
            root_node
        }
    }

    pub(crate) fn build_move_panel(&mut self, commands: &mut Commands, r_data: &HashMap<String, RVal>) -> Entity {
        let mut panel = FaBaseContainer::new();
        panel.cloned_attrs.overrided_border_color = Some(Color::NONE);
        panel.cloned_attrs.overrided_background_color = Some(Color::NONE);
        panel.cloned_attrs.node = default_move_panel_node();

        let panel_entity = panel.build(r_data, commands);
        commands
            .entity(panel_entity)
            .insert((IsFamiqScrollMovePanel, ScrollList::new(self.scroll_height)));

        commands.entity(panel_entity).add_children(&self.children);
        self.panel_entity = Some(panel_entity);
        panel_entity
    }

    pub fn rebuild_panel(&mut self, r_data: &HashMap<String, RVal>, world: &mut World) {
        let mut panel = FaBaseContainer::new();
        panel.cloned_attrs.overrided_border_color = Some(Color::NONE);
        panel.cloned_attrs.overrided_background_color = Some(Color::NONE);
        panel.cloned_attrs.node = default_move_panel_node();
        panel.rebuild(r_data, self.panel_entity.unwrap(), world);
        world
            .entity_mut(self.panel_entity.unwrap())
            .insert(ScrollList::new(self.scroll_height));
    }

    pub(crate) fn calculate_max_scroll(
        panel_node: &ComputedNode,
        scroll_node: &ComputedNode,
    ) -> f32 {
        let panel_height = panel_node.size().y * panel_node.inverse_scale_factor();
        let container_height = scroll_node.size().y * scroll_node.inverse_scale_factor();

        let max_scroll = panel_height - container_height;
        max_scroll.max(0.0)
    }
}

impl SetupWidget for ScrollBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqScroll, MainWidget, IsFamiqContainableWidget, ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_scroll_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;

        if self.cloned_attrs.color == WidgetColor::Default {
            self.cloned_attrs.color = WidgetColor::Transparent;
        }
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);

        let mut scroll = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let scroll_entity = scroll.build(r_data, commands);
        let panel_entity = self.build_move_panel(commands, r_data);

        commands
            .entity(scroll_entity)
            .add_child(panel_entity)
            .insert(self.components())
            .insert(ScrollMovePanelEntity(panel_entity));

        commands.entity(self.root_node).add_child(scroll_entity);

        insert_class_id(commands, scroll_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                scroll_entity,
                WidgetBuilder {
                    builder: BuilderType::Scroll(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        scroll_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_scroll_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;

        if self.cloned_attrs.color == WidgetColor::Default {
            self.cloned_attrs.color = WidgetColor::Transparent;
        }
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);

        let mut scroll = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        scroll.rebuild(r_data, old_entity, world);
        self.rebuild_panel(r_data, world);

        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Scroll(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

#[macro_export]
macro_rules! scroll {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();

        #[allow(unused_mut)]
        let mut children_vec: Vec<Entity> = Vec::new();
        $(
            $crate::extract_children!(children_vec, $key : $value);
        )*

        let root_entity = famiq_builder.resource.root_node_entity.unwrap();
        let s_builder = &mut ScrollBuilder::new(root_entity);
        $(
            $crate::scroll_attributes!(s_builder, $key : $value);
        )*
        s_builder.children = children_vec;
        s_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! scroll_attributes {
    // skip children
    ($s_builder:ident, children: $children_vec:tt) => {{}};

    ($s_builder:ident, scroll_height: $scroll_height:expr) => {{
        $s_builder.scroll_height = $scroll_height;
    }};
    ($s_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($s_builder, $key : $value);
    }};
}


/// Determines if scroll internal system(s) can run.
///
/// True only if there is a scroll widget created.
pub fn can_run_scroll_systems(scroll_q: Query<&IsFamiqScroll>) -> bool {
    !scroll_q.is_empty()
}
