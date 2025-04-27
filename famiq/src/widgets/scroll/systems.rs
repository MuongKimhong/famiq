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
                    let max_scroll = ScrollBuilder::calculate_max_scroll(panel_c_node, scroll_c_node);
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
