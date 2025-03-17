use bevy::prelude::*;
use smol_str::SmolStr;

use crate::utils::extract_val;

#[derive(Default, Debug)]
pub enum NeedToScroll {
    #[default]
    None,
    Left,
    Right
}

/// Component used for text editing functionality
#[derive(Component, Debug, Default)]
pub struct FaTextEdit {
    pub value: String,
    pub cursor_index: usize,
    pub min_cursor_pos: f32,
    pub max_cursor_pos: f32,
    pub need_scroll: NeedToScroll,
    pub selected_text: String,
    pub selection_start_index: Option<usize>,
    pub selection_end_index: Option<usize>,
    pub char_width: f32,
    pub char_height: f32,
    pub widget_computed: ComputedNode
}


impl FaTextEdit {
    pub fn move_cursor_start(&mut self) {
        self.cursor_index = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_index = self.value.len();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_index > 0 {
            self.cursor_index -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_index < self.value.len() {
            self.cursor_index += 1;
        }
    }

    pub fn move_cursor_index(&mut self, index: usize) {
        if index > 0 && index <= self.value.len() {
            self.cursor_index = index;
        }
    }

    pub fn insert(&mut self, character: &SmolStr) {
        self.value.insert_str(self.cursor_index, character);
        self.move_cursor_right();
    }

    pub fn remove(&mut self) {
        if self.cursor_index > 0 {
            let byte_index = self.value.char_indices().nth(self.cursor_index - 1).map(|(i, _)| i).unwrap();
            self.value.remove(byte_index);
        }
        self.move_cursor_left();
    }

    pub fn remove_selected_text(&mut self, hl_visibility: &mut Visibility) {
        if self.selection_start_index.is_some() && self.selection_end_index.is_some() {
            let start_index = self.selection_start_index.unwrap();
            let end_index = self.selection_end_index.unwrap();

            if end_index < start_index {
                self.value.drain(end_index..start_index);
            }
            else if end_index > start_index {
                self.value.drain(start_index..end_index);
            }

            self.clear_selection();
            *hl_visibility = Visibility::Hidden;
        }
    }

    pub fn new_line(&mut self) {
        self.insert(&SmolStr::new("\n"));
        self.move_cursor_right();
    }

    pub fn text_width(&self) -> f32 {
        self.value.len() as f32 * self.char_width
    }

    pub fn widget_scale(&self) -> f32 {
        self.widget_computed.inverse_scale_factor()
    }

    pub fn widget_padding_right(&self) -> f32 {
        self.widget_computed.padding().right * self.widget_scale()
    }

    pub fn widget_padding_top(&self) -> f32 {
        self.widget_computed.padding().top * self.widget_scale()
    }

    pub fn widget_padding_left(&self) -> f32 {
        self.widget_computed.padding().left * self.widget_scale()
    }

    // without padding
    pub fn widget_width_no_padding(&self) -> f32 {
        let widget_width = self.widget_computed.size().x * self.widget_scale();
        widget_width - self.widget_padding_left() - self.widget_padding_right()
    }

    pub fn widget_width_padding(&self) -> f32 {
        self.widget_computed.size().x * self.widget_scale()
    }

    // text width is longer than widget width without padding
    pub fn is_text_overflow(&self) -> bool {
        self.text_width() > self.widget_width_no_padding()
    }

    pub fn overflow_offset(&self) -> f32 {
        self.text_width() - self.widget_width_no_padding()
    }

    pub fn max_scroll_left(&self) -> f32 {
        0.0
    }

    pub fn max_scroll_right(&self) -> f32 {
        if self.text_width() > self.widget_width_no_padding() {
            self.text_width() - self.widget_width_no_padding()
        }
        else {
            0.0
        }
    }

    pub fn set_min_max_cursor_pos(&mut self) {
        self.min_cursor_pos = self.widget_padding_left();
        self.max_cursor_pos = self.widget_width_no_padding();
    }

    pub fn calculate_cursor_pos(&self, placeholder_node: &Node, index: f32) -> f32 {
        let left_val = extract_val(placeholder_node.left).unwrap();

        (left_val + self.widget_padding_left()) + (index * self.char_width)
    }

    pub fn move_cursor_pos_right(&mut self, placeholder_node: &Node) {
        let cursor_pos = self.calculate_cursor_pos(placeholder_node, self.cursor_index as f32);

        if cursor_pos < self.max_cursor_pos {
            self.need_scroll = NeedToScroll::None;
        } else {
            self.need_scroll = NeedToScroll::Right;
        }
    }

    pub fn move_cursor_pos_left(&mut self, placeholder_node: &Node) {
        let cursor_pos = self.calculate_cursor_pos(placeholder_node, self.cursor_index as f32);

        if cursor_pos > self.min_cursor_pos {
            self.need_scroll = NeedToScroll::None;
        } else {
            self.need_scroll = NeedToScroll::Left;
        }
    }

    pub fn move_cursor_pos_left_as_delete(&mut self, placeholder_node: &Node) {
        let cursor_pos = self.calculate_cursor_pos(placeholder_node, self.cursor_index as f32);

        if cursor_pos > self.min_cursor_pos {
            if self.text_width() + self.char_width > self.widget_width_no_padding() {
                self.need_scroll = NeedToScroll::Left;
            } else {
                self.need_scroll = NeedToScroll::None;
            }
        }
        else {
            self.need_scroll = NeedToScroll::Left;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_text.clear();
        self.selection_start_index = None;
        self.selection_end_index = None;
    }

    pub fn select_all(&mut self, placeholder_node: &Node, highlighter_node: &mut Node, highlighter_visibility: &mut Visibility) {
        self.selection_start_index = Some(0);
        self.selection_end_index = Some(self.value.len());
        self.selected_text = self.value.clone();
        self.cursor_index = 0;

        let start_pos = self.calculate_cursor_pos(placeholder_node, 0 as f32);
        let end_pos = self.calculate_cursor_pos(placeholder_node, self.value.len() as f32);
        highlighter_node.left = Val::Px(start_pos);
        highlighter_node.top = Val::Px(self.widget_padding_top());
        highlighter_node.width = Val::Px(end_pos - start_pos);
        *highlighter_visibility = Visibility::Visible;
    }

    pub fn is_ctrl_a_pressed(&self, keys: &Res<ButtonInput<KeyCode>>, keycode: KeyCode) -> bool {
        keys.pressed(KeyCode::ControlLeft) && matches!(keycode, KeyCode::KeyA)
    }

    pub fn is_ctrl_c_pressed(&self, keys: &Res<ButtonInput<KeyCode>>, keycode: KeyCode) -> bool {
        keys.pressed(KeyCode::ControlLeft) && matches!(keycode, KeyCode::KeyC)
    }

    pub fn is_ctrl_v_pressed(&self, keys: &Res<ButtonInput<KeyCode>>, keycode: KeyCode) -> bool {
        keys.pressed(KeyCode::ControlLeft) && matches!(keycode, KeyCode::KeyV)
    }
}
