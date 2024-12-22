pub mod helper;

// use crate::event_writer::FaInteractionEvent;
use crate::utils;
use crate::widgets::{DefaultWidgetEntity, FamiqWidgetId, WidgetType};
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
        let visibility = Visibility::Visible;

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

    fn _build_listview(id: &str, root_node: &'a mut EntityCommands) -> Entity {
        let node = default_listview_node();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(id.to_string()),
                IsFamiqListView,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default()
            ))
            .id()
    }

    pub fn new(id: &str, root_node: &'a mut EntityCommands, items: &Vec<Entity>) -> Entity {
        let move_panel = Self::_build_move_panel(id, items, root_node);
        let listview = Self::_build_listview(id, root_node);

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

    fn _calculate_max_scroll(panel_node: &ComputedNode, listview_node: &ComputedNode, listview_style: &Node) -> f32 {
        let panel_height = panel_node.size().y;
        let container_height = listview_node.size().y;

        let mut max_scroll = panel_height - container_height;

        let padding_y = Self::_calculate_listview_padding_y(listview_style);
        if let Some((top, bottom)) = padding_y {
            max_scroll += top + bottom;
        }
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

    pub fn on_mouse_scroll_system(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        listview_q: Query<(&ComputedNode, &Node, &ListViewMovePanelEntity), Without<ScrollList>>,
        mut listview_panel_q: Query<(&mut ScrollList, &mut Node, &ComputedNode)>,
        can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
    ) {
        let scroll_height = 12.0;

        for e in mouse_wheel_events.read() {
            if let Some(hovered_listview) = can_be_scrolled_listview.entity {
                if let Ok((l_node, l_style, panel_entity)) = listview_q.get(hovered_listview) {
                    if let Ok((mut scrolling, mut style, panel_node)) =
                        listview_panel_q.get_mut(panel_entity.0)
                    {
                        let max_scroll = Self::_calculate_max_scroll(panel_node, l_node, l_style);

                        let dy = match e.unit {
                            MouseScrollUnit::Line => e.y * scroll_height,
                            MouseScrollUnit::Pixel => e.y,
                        };
                        scrolling.position += dy;
                        scrolling.position = scrolling.position.clamp(-max_scroll, 0.);
                        style.top = Val::Px(scrolling.position);
                        break;
                    }
                }
            }
        }
    }

    pub fn scroll_top(
        listview_entity: Entity,
        listview_q: &Query<(&ComputedNode, &Node, &ListViewMovePanelEntity)>,
        listview_panel_q: &mut Query<(&mut ScrollList, &mut Node, &ComputedNode)>,
    ) {
        if let Ok((_, listview_style, panel_entity)) = listview_q.get(listview_entity) {
            if let Ok((mut list, mut style, _)) = listview_panel_q.get_mut(panel_entity.0) {
                match Self::_calculate_listview_padding_y(listview_style) {
                    Some((top, _)) => list.position = top,
                    None => list.position = 0.0,
                }
                style.top = Val::Px(list.position);
            }
        }
    }

    pub fn scroll_bottom(
        listview_entity: Entity,
        listview_q: &Query<(&ComputedNode, &Node, &ListViewMovePanelEntity)>,
        listview_panel_q: &mut Query<(&mut ScrollList, &mut Node, &ComputedNode)>,
    ) {
        if let Ok((listview_node, listview_style, panel_entity)) = listview_q.get(listview_entity) {
            if let Ok((mut list, mut style, panel_node)) = listview_panel_q.get_mut(panel_entity.0)
            {
                let listview_height = listview_node.size().y;
                let panel_height = panel_node.size().y;

                list.position = panel_height - listview_height;

                match Self::_calculate_listview_padding_y(listview_style) {
                    Some((top, bottom)) => {
                        list.position += top + bottom;
                    }
                    None => (),
                }
                style.top = Val::Px(list.position.max(0.0));
            }
        }
    }
}

// // containable
// impl<'a> FaListView {
//     pub fn new(id: &str, root_node: &'a mut EntityCommands, items: &Vec<Entity>) -> Entity {
//         let container_bundle = default_list_view_container();
//         let move_panel_bundle = default_list_view_move_panel();

//         let move_panel_entity = root_node
//             .commands()
//             .spawn((
//                 move_panel_bundle.clone(),
//                 ScrollingList::default(),
//                 FamiqWidgetId(format!("{id}_move_panel")),
//                 IsFamiqListViewMovePanel,
//                 DefaultWidgetBundle(move_panel_bundle),
//             ))
//             .id();

//         let container_entity = root_node
//             .commands()
//             .spawn((
//                 container_bundle.clone(),
//                 ListView::new(items),
//                 FamiqWidgetId(id.to_string()),
//                 ListViewMovePanelEntity(move_panel_entity),
//                 IsFamiqListView,
//                 DefaultWidgetBundle(container_bundle),
//             ))
//             .id();

//         root_node.add_child(container_entity);

//         utils::entity_add_child(root_node, move_panel_entity, container_entity);

//         // insert ListViewItemId component into user provided items's entities
//         for (_index, item_entity) in items.iter().enumerate() {
//             let cloned = item_entity.clone();
//             root_node
//                 .commands()
//                 .entity(cloned)
//                 .insert((IsFamiqListViewItem,));
//         }
//         utils::entity_push_children(root_node, items, move_panel_entity);
//         container_entity
//     }

//     pub fn on_hover_system(
//         mut interaction_events: EventReader<FaInteractionEvent>,
//         mut can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
//     ) {
//         for e in interaction_events.read() {
//             if e.widget_type == WidgetType::ListView && e.interaction_type == Interaction::Hovered {
//                 can_be_scrolled_listview.entity = Some(e.entity);
//                 break;
//             }
//         }
//     }

//     // (top & bottom)
//     fn _calculate_listview_padding_y(listview_style: &Style) -> Option<(f32, f32)> {
//         let padding_top = utils::extract_val(listview_style.padding.top);
//         let padding_bottom = utils::extract_val(listview_style.padding.bottom);

//         if padding_top == None || padding_bottom == None {
//             return None;
//         }
//         Some((padding_top.unwrap(), padding_bottom.unwrap()))
//     }

//     fn _calculate_max_scroll(
//         panel_node: &Node,
//         listview_node: &Node,
//         listview_style: &Style,
//     ) -> f32 {
//         let panel_height = panel_node.size().y;
//         let container_height = listview_node.size().y;

//         let mut max_scroll = panel_height - container_height;

//         let padding_y = Self::_calculate_listview_padding_y(listview_style);
//         if let Some((top, bottom)) = padding_y {
//             max_scroll += top + bottom;
//         }
//         max_scroll.max(0.0)
//     }

//     pub fn on_mouse_scroll_system(
//         mut mouse_wheel_events: EventReader<MouseWheel>,
//         listview_q: Query<(&Node, &Style, &ListViewMovePanelEntity), Without<ScrollingList>>,
//         mut listview_panel_q: Query<(&mut ScrollingList, &mut Style, &Node)>,
//         can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
//     ) {
//         let scroll_height = 12.0;

//         for e in mouse_wheel_events.read() {
//             if let Some(hovered_listview) = can_be_scrolled_listview.entity {
//                 if let Ok((l_node, l_style, panel_entity)) = listview_q.get(hovered_listview) {
//                     if let Ok((mut scrolling, mut style, panel_node)) =
//                         listview_panel_q.get_mut(panel_entity.0)
//                     {
//                         let max_scroll = Self::_calculate_max_scroll(panel_node, l_node, l_style);

//                         let dy = match e.unit {
//                             MouseScrollUnit::Line => e.y * scroll_height,
//                             MouseScrollUnit::Pixel => e.y,
//                         };
//                         scrolling.position += dy;
//                         scrolling.position = scrolling.position.clamp(-max_scroll, 0.);
//                         style.top = Val::Px(scrolling.position);
//                         break;
//                     }
//                 }
//             }
//         }
//     }

//     pub fn scroll_top(
//         listview_entity: Entity,
//         listview_q: &Query<(&Node, &Style, &ListViewMovePanelEntity)>,
//         listview_panel_q: &mut Query<(&mut ScrollingList, &mut Style, &Node)>,
//     ) {
//         if let Ok((_, listview_style, panel_entity)) = listview_q.get(listview_entity) {
//             if let Ok((mut list, mut style, _)) = listview_panel_q.get_mut(panel_entity.0) {
//                 match Self::_calculate_listview_padding_y(listview_style) {
//                     Some((top, _)) => list.position = top,
//                     None => list.position = 0.0,
//                 }
//                 style.top = Val::Px(list.position);
//             }
//         }
//     }

//     pub fn scroll_bottom(
//         listview_entity: Entity,
//         listview_q: &Query<(&Node, &Style, &ListViewMovePanelEntity)>,
//         listview_panel_q: &mut Query<(&mut ScrollingList, &mut Style, &Node)>,
//     ) {
//         if let Ok((listview_node, listview_style, panel_entity)) = listview_q.get(listview_entity) {
//             if let Ok((mut list, mut style, panel_node)) = listview_panel_q.get_mut(panel_entity.0)
//             {
//                 let listview_height = listview_node.size().y;
//                 let panel_height = panel_node.size().y;

//                 list.position = panel_height - listview_height;

//                 match Self::_calculate_listview_padding_y(listview_style) {
//                     Some((top, bottom)) => {
//                         list.position += top + bottom;
//                     }
//                     None => (),
//                 }
//                 style.top = Val::Px(list.position.max(0.0));
//             }
//         }
//     }
// }
