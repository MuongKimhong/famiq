use bevy::prelude::*;
use super::*;

/// System to track hover interactions on ListView widgets.
pub fn on_hover_system(
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

/// Internal system to handle scrolling on scroll widgets.
pub fn on_scroll_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scroll_q: Query<
        (&mut Node, &ComputedNode, &ScrollMovePanelEntity, Option<&WidgetId>),
        (With<IsFamiqScroll>, Without<IsFamiqScrollMovePanel>)
    >,
    mut panel_q: Query<
        (&mut ScrollList, &ComputedNode),
        (With<IsFamiqScrollMovePanel>, Without<IsFamiqScroll>)
    >,
    mut can_be_scrolled: ResMut<CanBeScrolled>,
    mut mouse_event_writer: EventWriter<FaMouseEvent>,
) {
    for e in mouse_wheel_events.read() {
        if let Some(hovered) = can_be_scrolled.entity {

            // get hovered scroll
            if let Ok((_, scroll_c_node, panel_entity, scroll_id)) = scroll_q.get_mut(hovered) {
                if let Ok((mut scroll_list, panel_c_node)) = panel_q.get_mut(panel_entity.0) {
                    let dy = match e.unit {
                        MouseScrollUnit::Line => e.y * scroll_list.scroll_height,
                        MouseScrollUnit::Pixel => e.y,
                    };
                    let max_scroll = ScrollBuilder::calculate_max_scroll(panel_c_node, scroll_c_node);

                    scroll_list.target_position = (scroll_list.position + dy).clamp(-max_scroll, 0.0);
                    can_be_scrolled.is_scrolling = true;

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

pub fn on_update_scroll_position(
    mut panel_q: Query<(&mut Node, &mut DefaultWidgetConfig, &mut ScrollList), With<IsFamiqScrollMovePanel>>,
    scroll_q: Query<&ScrollMovePanelEntity>,
    mut can_be_scrolled: ResMut<CanBeScrolled>
) {
    if can_be_scrolled.entity.is_some() && can_be_scrolled.is_scrolling {
        let hovered_scroll_entity = can_be_scrolled.entity.unwrap();

        if let Ok(panel_entity) = scroll_q.get(hovered_scroll_entity) {
            if let Ok((mut panel_node, mut default, mut scroll_list)) = panel_q.get_mut(panel_entity.0) {
                let diff = scroll_list.target_position - scroll_list.position;

                if diff.abs() > 0.1 {
                    scroll_list.position += diff * SMOOTH_SCROLL_FACTOR;
                    panel_node.top = Val::Px(scroll_list.position);
                    default.node.top = Val::Px(scroll_list.position);
                }
                else {
                    can_be_scrolled.is_scrolling = false;
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
