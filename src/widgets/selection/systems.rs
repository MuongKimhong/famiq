// use crate::widgets::selection::*;
// use bevy::prelude::*;

// // on focus use PLACEHOLDER_COLOR_FOCUSED else use PLACEHOLDER_COLOR_UNFOCUSED
// pub fn update_selector_placeholder_color_system(
//     selection_q: Query<(&Selection, &SelectorPlaceHolderEntity), Changed<Selection>>,
//     mut text_q: Query<&mut Text>,
// ) {
//     for (selection, placeholder) in selection_q.iter() {
//         if let Ok(mut text) = text_q.get_mut(placeholder.0) {
//             text.sections[0].style.color = if selection.focused {
//                 PLACEHOLDER_COLOR_FOCUSED
//             } else {
//                 PLACEHOLDER_COLOR_UNFOCUSED
//             };
//         }
//     }
// }

// // on focus use arrow up else use arrow down
// pub fn update_selector_arrow_icon_system(
//     selection_q: Query<(&Selection, &SelectorArrowIconEntity), Changed<Selection>>,
//     mut text_q: Query<&mut Text>,
// ) {
//     for (selection, arrow_icon_entity) in selection_q.iter() {
//         if let Ok(mut text) = text_q.get_mut(arrow_icon_entity.0) {
//             text.sections[0].value = if selection.focused {
//                 "▲".to_string()
//             } else {
//                 "▼".to_string()
//             };
//         }
//     }
// }

// // on focus show panel else hide
// pub fn update_selection_items_panel_visibility_system(
//     selection_q: Query<(&Selection, &SelectionItemsPanelEntity), Changed<Selection>>,
//     mut visibility_q: Query<&mut Visibility>,
// ) {
//     for (selection, items_panel_entity) in selection_q.iter() {
//         if let Ok(mut visibility) = visibility_q.get_mut(items_panel_entity.0) {
//             *visibility = if selection.focused {
//                 Visibility::Visible
//             } else {
//                 Visibility::Hidden
//             };
//         }
//     }
// }

// pub fn update_selection_items_panel_position_and_width_system(
//     selection_q: Query<(
//         &Selection,
//         &Node,
//         &SelectionItemsPanelEntity,
//         Option<&SelectionLabelEntity>,
//         &Parent,
//     )>,
//     node_q: Query<&ComputedNode>,
//     mut style_q: Query<&mut Node, Without<Selection>>,
// ) {
//     for (selection, selection_style, panel_entity, label_entity, parent) in selection_q.iter() {
//         if selection.focused {
//             let mut label_height = 0.0;

//             if let Some(label) = label_entity {
//                 if let Ok(label_node) = node_q.get(label.0) {
//                     label_height = label_node.size().y;
//                 }
//             }

//             if let Ok(mut panel_style) = style_q.get_mut(panel_entity.0) {
//                 set_selection_panel_pos(selection_style, &mut panel_style, label_height);
//                 set_selection_panel_width(&node_q, parent, &mut panel_style);
//             }

//             break;
//         }
//     }
// }
