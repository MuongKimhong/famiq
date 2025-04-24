pub mod components;
pub mod tests;
pub use components::*;

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
    pub entity: Option<Entity>,
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
pub struct ScrollBuilder {
    pub children: Vec<Entity>,
    pub root_node: Entity,
    pub scroll_height: f32
}

impl ScrollBuilder {
    pub fn new(root_node: Entity) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            children: Vec::new(),
            scroll_height: 15.0,
            root_node
        }
    }

    pub(crate) fn build_move_panel(
        &self,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut panel = FaBaseContainer::new();
        panel.cloned_attrs.overrided_border_color = Some(Color::NONE);
        panel.cloned_attrs.overrided_background_color = Some(Color::NONE);
        panel.cloned_attrs.node = default_move_panel_node();

        let panel_entity = panel.build(r_data, commands);
        commands
            .entity(panel_entity)
            .insert((IsFamiqScrollMovePanel, ScrollList::new(self.scroll_height)));

        for (_index, child_entity) in self.children.iter().enumerate() {
            commands.entity(child_entity.clone()).insert(IsFamiqScrollItem);
        }
        commands.entity(panel_entity).add_children(&self.children);
        panel_entity
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

    /// System to track hover interactions on ListView widgets.
    pub(crate) fn on_hover_system(
        scroll_q: Query<(Entity, &Interaction), (With<IsFamiqScroll>, Changed<Interaction>)>,
        mut can_be_scrolled: ResMut<CanBeScrolled>
    ) {
        for (entity, interaction) in scroll_q.iter() {
            if *interaction == Interaction::Hovered {
                can_be_scrolled.entity = Some(entity);
                break;
            }
        }
    }

    /// Internal system to handle scrolling interactions on ListView widgets.
    pub fn on_scroll_system(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut scroll_q: Query<
            (&mut Node, &ComputedNode, &ScrollMovePanelEntity, Option<&WidgetId>),
            Without<ScrollList>
        >,
        mut panel_q: Query<(&mut Node, &ComputedNode, &mut ScrollList, &mut DefaultWidgetConfig)>,
        mut mouse_event_writer: EventWriter<FaMouseEvent>,
        can_be_scrolled: ResMut<CanBeScrolled>,
    ) {
        for e in mouse_wheel_events.read() {
            if let Some(hovered) = can_be_scrolled.entity {

                // get hovered listview
                if let Ok((_, scroll_c_node, panel_entity, scroll_id)) = scroll_q.get_mut(hovered) {

                    // get panel
                    if let Ok((mut panel_node, panel_c_node, mut scroll_list, mut default_style)) = panel_q.get_mut(panel_entity.0) {

                        let dy = match e.unit {
                            MouseScrollUnit::Line => e.y * scroll_list.scroll_height,
                            MouseScrollUnit::Pixel => e.y,
                        };
                        let max_scroll = Self::calculate_max_scroll(panel_c_node, scroll_c_node);

                        scroll_list.position = (scroll_list.position + dy).clamp(-max_scroll, 0.0);

                        panel_node.top = Val::Px(scroll_list.position);
                        default_style.node.top = Val::Px(scroll_list.position);

                        FaMouseEvent::send_event(
                            &mut mouse_event_writer,
                            EventType::Scroll,
                            WidgetType::Scroll,
                            hovered,
                            scroll_id
                        );
                    }
                }
            }
        }
    }

    pub fn detect_new_scroll_system(
        listview_q: Query<&ScrollMovePanelEntity, Added<IsFamiqScroll>>,
        mut panel_q: Query<&mut Node, With<IsFamiqScrollMovePanel>>
    ) {
        for panel_entity in listview_q.iter() {
            if let Ok(mut panel_node) = panel_q.get_mut(panel_entity.0) {
                panel_node.padding = UiRect::all(Val::Px(0.0));
            }
        }
    }
}

impl SetupWidget for ScrollBuilder {
    fn components(&mut self) -> impl Bundle {
        (IsFamiqScroll, MainWidget, IsFamiqContainableWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_scroll_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;

        if self.cloned_attrs.color == WidgetColor::Default {
            self.cloned_attrs.color = WidgetColor::Transparent;
        }
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

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
        scroll_entity
    }

    fn build_with_world(
        &mut self,
        _reactive_data: &HashMap<String, RVal>,
        _world: &mut World
    ) -> Option<Entity> {
        None
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
