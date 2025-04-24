//! Famiq's global resources, used by all modules.

use bevy::utils::hashbrown::HashMap;
use bevy::prelude::*;
use cosmic_text::{FontSystem, SwashCache};
use cosmic_text::{Edit, Shaping, Cursor};

use crate::widgets::button::IsFamiqButtonText;
use crate::widgets::text::IsFamiqText;
use crate::widgets::*;
use crate::utils::*;
use crate::widgets::progress_bar::*;
use crate::widgets::checkbox::*;
use crate::widgets::modal::*;
use crate::widgets::selection::*;
use crate::widgets::text_input::*;
use crate::widgets::text_input::helper;
use crate::widgets::color::PRIMARY_DARK_COLOR;
use crate::FaValueChangeEvent;

/// Resource for detecting style changes in json file
#[derive(Resource, Default)]
pub(crate) struct StylesKeyValueResource {
    pub values: HashMap<String, WidgetStyle>, // key-value of "#widget-id"/".class-name" and all its styles in styles.json
    pub changed_keys: Vec<String>
}

impl StylesKeyValueResource {
    pub fn get_style_by_id(&self, widget_id: &str) -> Option<&WidgetStyle> {
        self.values.get(widget_id)
    }

    pub fn get_style_by_class_name(&self, class_name: &str) -> Option<&WidgetStyle> {
        self.values.get(class_name)
    }
}

#[derive(Resource)]
pub struct FamiqResource {
    /// font path relative to project root
    pub font_path: String,

    /// user external style (json) file path relative to project root
    pub style_path: String,

    /// read external style (json) file and apply styles to widget every single frame
    pub hot_reload_styles: bool,

    /// copied text from text input
    pub copied_text: String,
    pub root_node_entity: Option<Entity>,

    pub(crate) widget_focus_state: HashMap<Entity, bool>,
    pub(crate) external_style_applied: bool,
}

#[derive(Resource)]
pub struct CosmicFontSystem(pub FontSystem);

#[derive(Resource)]
pub struct CosmicSwashCache(pub SwashCache);

impl FamiqResource {
    pub fn update_or_insert_focus_state(&mut self, entity: Entity, state: bool) {
        if let Some(old_value) = self.widget_focus_state.get_mut(&entity) {
            *old_value = state;
        } else {
            self.widget_focus_state.insert(entity, state);
        }
    }

    pub fn update_all_focus_states(&mut self, new_state: bool) {
        for (_, state) in self.widget_focus_state.iter_mut() {
            *state = new_state;
        }
    }

    pub fn get_widget_focus_state(&self, entity: &Entity) -> Option<bool> {
        if let Some(&state) = self.widget_focus_state.get(entity) {
            return Some(state);
        }
        None
    }

    pub fn new() -> Self {
        Self {
            font_path: get_embedded_asset_path("embedded_assets/fonts/fira-mono-medium.ttf").to_string(),
            style_path: "assets/styles.json".to_string(),
            hot_reload_styles: false,
            widget_focus_state: HashMap::new(),
            external_style_applied: false,
            root_node_entity: None,
            copied_text: String::new()
        }
    }
}

/// Store children entity for containable widgets
#[derive(Resource, Debug, Default)]
pub struct ContainableChildren {
    pub data: HashMap<Entity, Vec<Entity>>
}

impl ContainableChildren {
    /// Insert or update data
    pub fn insert(&mut self, entity: Entity, children: Vec<Entity>) {
        self.data.insert(entity, children);
    }

    /// Replace child entity with provided one
    pub fn update_child(&mut self, parent: Entity, new_child: Entity, old_child: Entity) {
        if let Some(children_list) = self.data.get_mut(&parent) {
            for child in children_list.iter_mut() {
                if *child == old_child {
                    *child = new_child;
                    return;
                }
            }
        }
    }

    /// Insert new containable, remove old one.
    pub fn update_containable(&mut self, old: Entity, new: Entity) {
        if let Some(children_list) = self.data.remove(&old) {
            self.data.insert(new, children_list);
        }
    }

    pub fn get_children(&self, containable: Entity) -> Option<Vec<Entity>> {
        if let Some(children_list) = self.data.get(&containable) {
            return Some(children_list.to_owned());
        }
        None
    }
}

// pub(crate) fn detect_reactive_data_change(
//     mut r_data: ResMut<RData>,
//     mut r_text_q: Query<
//         (&mut Text, &ReactiveText, &ReactiveKeys),
//         Or<(With<IsFamiqText>, With<IsFamiqButtonText>)>
//     >,

//     mut progress_bar_value_q: Query<
//         (Entity, &ReactiveModelKey, Option<&FamiqWidgetId>, &mut Node, &mut FaProgressValuePercentage),
//         With<IsFamiqProgressValue>
//     >,

//     checkbox_q: Query<
//         (Entity, &ReactiveModelKey, Option<&FamiqWidgetId>),
//         With<IsFamiqCheckbox>
//     >,
//     checkbox_item_q: Query<(&CheckBoxItemBoxEntity, &CheckBoxItemText)>,
//     mut checkbox_item_box_q: Query<(&mut CheckBoxChoiceTicked, &mut BackgroundColor)>,

//     modal_bg_q: Query<(Entity, &ReactiveModelKey), With<IsFamiqModalBackground>>,
//     mut modal_res: ResMut<FaModalState>,

//     selector_q: Query<
//         (Entity, &Selection, &ReactiveModelKey, Option<&FamiqWidgetId>, &SelectorPlaceHolderEntity),
//         With<IsFamiqSelectionSelector>
//     >,
//     mut selector_placeholder_q: Query<
//         &mut Text,
//         (With<SelectorPlaceHolder>, Without<IsFamiqText>, Without<IsFamiqButtonText>)
//     >,

//     mut text_input_q: Query<
//         (
//             Entity,
//             &mut FaTextEdit,
//             &ReactiveModelKey,
//             &mut CosmicData,
//             Option<&FamiqWidgetId>,
//             &FaTextInputBufferTextureEntity
//         ),
//         With<IsFamiqTextInput>
//     >,
//     texture_q: Query<
//         &Node,
//         (With<IsFamiqTextInputBufferTexture>, Without<IsFamiqMainWidget>, Without<IsFamiqProgressValue>)
//     >,
//     mut font_system: ResMut<CosmicFontSystem>,

//     mut change_writer: EventWriter<FaValueChangeEvent>,
//     mut request_redraw: EventWriter<RequestRedrawBuffer>
// ) {
//     if !r_data.is_changed() && !r_data.is_added() {
//         return;
//     }
//     // reactive text
//     for (mut text, r_text, r_keys) in r_text_q.iter_mut() {
//         let mut temp_r_text = r_text.0.clone();

//         for (i, key) in r_keys.0.iter().enumerate() {
//             if let Some(value) = r_data.data.get(key) {
//                 let placeholder = format!("$[{}]", i);

//                 match value {
//                     RVal::Num(v) => temp_r_text = temp_r_text.replace(&placeholder, &v.to_string()),
//                     RVal::FNum(v) => temp_r_text = temp_r_text.replace(&placeholder, &v.to_string()),
//                     RVal::Str(v) => temp_r_text = temp_r_text.replace(&placeholder, v),
//                     _ => {}
//                 }
//             }
//         }
//         text.0 = temp_r_text;
//     }

//     // progress bar
//     for (entity, model_key, id, mut bar_value_node, mut percentage) in progress_bar_value_q.iter_mut() {
//         if let Some(value) = r_data.data.get(&model_key.0) {
//             let mut value_changed = false;
//             match value {
//                 RVal::FNum(v) => {
//                     percentage.0 = Some(v.to_owned());
//                     bar_value_node.width = Val::Percent(v.to_owned());
//                     value_changed = true;
//                 }
//                 RVal::None => {
//                     percentage.0 = None;
//                     bar_value_node.width = Val::Percent(100.0);
//                     value_changed = true;
//                 }
//                 _ => {}
//             }
//             if value_changed {
//                 change_writer.send(FaValueChangeEvent::new(
//                     entity,
//                     id.map(|_id| _id.0.clone()),
//                     model_key.0.clone()
//                 ));
//             }
//         }
//     }

//     // checkbox
//     for (entity, model_key, id) in checkbox_q.iter() {
//         if let Some(value) = r_data.data.get(&model_key.0) {
//             match value {
//                 RVal::List(items) => {
//                     for (box_entity, item_text) in checkbox_item_q.iter() {
//                         if let Ok((mut box_ticked, mut bg_color)) = checkbox_item_box_q.get_mut(box_entity.0) {
//                             box_ticked.0 = items.contains(&item_text.0);

//                             if box_ticked.0 {
//                                 bg_color.0 = PRIMARY_DARK_COLOR;
//                             } else {
//                                 bg_color.0 = Color::NONE;
//                             }
//                         }
//                     }
//                     change_writer.send(FaValueChangeEvent::new(
//                         entity,
//                         id.map(|_id| _id.0.clone()),
//                         model_key.0.clone()
//                     ));
//                 }
//                 _ => {}
//             }
//         }
//     }

//     // modal
//     for (entity, model_key) in modal_bg_q.iter() {
//         if let Some(value) = r_data.data.get(&model_key.0) {
//             let mut current_state = false;

//             match value {
//                 RVal::Bool(state) => current_state = *state,
//                 _ => {}
//             }
//             let is_visible = modal_res
//                 .get_state_by_entity(entity)
//                 .copied()
//                 .unwrap_or(false);

//             if current_state != is_visible {
//                 if current_state {
//                     modal_res.show_by_entity(entity);
//                 } else {
//                     modal_res.hide_by_entity(entity);
//                 }
//             }
//         }
//     }

//     // selection
//     for (entity, selection, model_key, id, pl_entity) in selector_q.iter() {
//         if let Some(value) = r_data.data.get(&model_key.0) {
//             if let Ok(mut pl_text) = selector_placeholder_q.get_mut(pl_entity.0) {
//                 match value {
//                     RVal::Str(v) => {
//                         if !v.is_empty() {
//                             pl_text.0 = v.to_owned();
//                         } else {
//                             pl_text.0 = selection.placeholder.to_owned();
//                         }
//                         change_writer.send(FaValueChangeEvent::new(
//                             entity,
//                             id.map(|_id| _id.0.clone()),
//                             model_key.0.clone()
//                         ));
//                     },
//                     _ => {}
//                 }
//             }
//         }
//     }

//     // text input
//     for (entity, mut text_edit, model_key, mut cosmic_data, id, buf_entity) in text_input_q.iter_mut() {
//         if let Ok(texture_node) = texture_q.get(buf_entity.0) {
//             if let Some(value) = r_data.data.get(&model_key.0) {
//                 match value {
//                     RVal::Str(v) => {
//                         if text_edit.value != *v {
//                             text_edit.value = v.to_owned();

//                             let CosmicData { buffer_dim, attrs, editor, .. } = &mut *cosmic_data;

//                             if let Some(editor) = editor.as_mut() {
//                                 editor.with_buffer_mut(|buffer| {
//                                     buffer.set_size(&mut font_system.0, None, None);
//                                     buffer.set_text(&mut font_system.0, &text_edit.value, attrs.unwrap(), Shaping::Advanced);
//                                     helper::update_buffer_text_layout(
//                                         &mut font_system.0,
//                                         &mut text_edit,
//                                         buffer_dim,
//                                         buffer,
//                                         texture_node
//                                     );
//                                 });
//                                 editor.set_cursor(Cursor::new(0, text_edit.cursor_index));
//                             }
//                         }
//                         else {
//                             change_writer.send(FaValueChangeEvent::new(
//                                 entity,
//                                 id.map(|_id| _id.0.clone()),
//                                 model_key.0.clone()
//                             ));
//                         }
//                         request_redraw.send(RequestRedrawBuffer::new(entity));
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }

//     r_data.changed_keys.clear();
// }
