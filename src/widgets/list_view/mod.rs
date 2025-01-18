pub mod helper;

// use crate::event_writer::FaInteractionEvent;
use crate::utils;
use crate::widgets::{DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetClasses, WidgetType};
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use helper::*;

#[derive(Component, Debug)]
pub struct ListView {
    pub focused: bool,
    pub items: Vec<Entity>,
}

impl ListView {
    pub fn new(items: &Vec<Entity>) -> Self {
        Self {
            focused: false,
            items: items.clone(),
        }
    }
}

#[derive(Component)]
pub struct IsFamiqListView;

#[derive(Component)]
pub struct IsFamiqListViewItem;

#[derive(Component)]
pub struct IsFamiqListViewMovePanel;

#[derive(Component)]
pub struct ListViewMovePanelEntity(pub Entity);

#[derive(Component, Default)]
pub struct ScrollList {
    pub position: f32,
}

// only listview with entity inside this resource can be scrolled
#[derive(Resource)]
pub struct CanBeScrolledListView {
    pub entity: Option<Entity>,
}

pub struct FaListView;

// Doesn't need container
impl<'a> FaListView {
    fn _build_move_panel(
        id: &str,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let node = default_move_panel_node();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        let move_panel_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                bg_color.clone(),
                border_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(format!("{id}_move_panel")),
                IsFamiqListViewMovePanel,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                ScrollList::default()
            ))
            .id();

        // insert IsFamiqListViewItem component into user provided items's entities
        for (_index, item_entity) in items.iter().enumerate() {
            let cloned = item_entity.clone();
            root_node
                .commands()
                .entity(cloned)
                .insert((IsFamiqListViewItem,));
        }
        utils::entity_add_children(root_node, items, move_panel_entity);
        move_panel_entity
    }

    fn _build_listview(
        id: &str,
        classes: &str,
        root_node: &'a mut EntityCommands,
        panel_entity: Entity,
    ) -> Entity {
        let node = default_listview_node();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let listview_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(id.to_string()),
                FamiqWidgetClasses(classes.to_string()),
                IsFamiqListView,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                ListViewMovePanelEntity(panel_entity),
            ))
            .id();
        listview_entity
    }

    pub fn new(id: &str, classes: &str, root_node: &'a mut EntityCommands, items: &Vec<Entity>) -> Entity {
        let move_panel = Self::_build_move_panel(id, items, root_node);
        let listview = Self::_build_listview(id, classes, root_node, move_panel);

        utils::entity_add_child(root_node, move_panel, listview);
        listview
    }

    // (top & bottom)
    fn _calculate_listview_padding_y(listview_style: &Node) -> Option<(f32, f32)> {
        let padding_top = utils::extract_val(listview_style.padding.top);
        let padding_bottom = utils::extract_val(listview_style.padding.bottom);

        if padding_top == None || padding_bottom == None {
            return None;
        }
        Some((padding_top.unwrap(), padding_bottom.unwrap()))
    }

    fn _calculate_max_scroll(
        panel_node: &ComputedNode,
        listview_node: &ComputedNode,
        // listview_style: &Node,
    ) -> f32 {
        let panel_height = panel_node.size().y;
        let container_height = listview_node.size().y;

        let max_scroll = panel_height - container_height;
        max_scroll.max(0.0)
    }

    pub fn on_hover_system(
        mut interaction_events: EventReader<FaInteractionEvent>,
        mut can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
    ) {
        for e in interaction_events.read() {
            if e.widget == WidgetType::ListView && e.interaction == Interaction::Hovered {
                can_be_scrolled_listview.entity = Some(e.entity);
                break;
            }
        }
    }

    pub fn on_scroll_system(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut listview_q: Query<(&mut Node, &ComputedNode, &ListViewMovePanelEntity), Without<ScrollList>>,
        mut panel_q: Query<(&mut Node, &ComputedNode, &mut ScrollList, &mut DefaultWidgetEntity)>,
        can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
    ) {
        let scroll_height: f32 = 20.0;

        // for (mut listview_node, _, _) in listview_q.iter_mut() {
        //     // always set paddings to 0 as ListView can't have paddings.
        //     listview_node.padding = UiRect::all(Val::Px(0.0));
        // }

        for e in mouse_wheel_events.read() {
            if let Some(hovered_listview) = can_be_scrolled_listview.entity {

                // get hovered listview
                if let Ok((_, listview_c_node, panel_entity)) = listview_q.get_mut(hovered_listview) {

                    // get panel
                    if let Ok((mut panel_node, panel_c_node, mut scroll_list, mut default_style)) = panel_q.get_mut(panel_entity.0) {

                        let dy = match e.unit {
                            MouseScrollUnit::Line => e.y * scroll_height,
                            MouseScrollUnit::Pixel => e.y,
                        };
                        let max_scroll = Self::_calculate_max_scroll(panel_c_node, listview_c_node);

                        scroll_list.position = (scroll_list.position + dy).clamp(-max_scroll, 0.0);

                        panel_node.top = Val::Px(scroll_list.position);
                        default_style.node.top = Val::Px(scroll_list.position);

                    }

                }

            }
        }
    }
}
