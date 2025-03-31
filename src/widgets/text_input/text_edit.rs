use bevy::prelude::*;
use cosmic_text::{
    Color as CosmicColor, Editor, Edit, Cursor, Selection, Attrs, Metrics, LayoutGlyph
};
use smol_str::SmolStr;
use arboard::Clipboard;

#[cfg(target_os = "linux")]
use arboard::{SetExtLinux, LinuxClipboardKind};

use crate::utils::*;

pub const DEFAULT_CURSOR_COLOR: CosmicColor = CosmicColor::rgb(0, 0, 0); // black
pub const CURSOR_INVISIBLE: CosmicColor = CosmicColor::rgba(0, 0, 0, 0);
pub const DEFAULT_TEXT_COLOR: CosmicColor = CosmicColor::rgb(0, 0, 0);
pub const DEFAULT_SELECTION_COLOR: CosmicColor = CosmicColor::rgba(156, 156, 156, (0.35 * 255.0) as u8);
pub const DEFAULT_SELECTED_TEXT_COLOR: CosmicColor = CosmicColor::rgb(0, 0, 0);

#[derive(Default, Debug)]
pub enum NeedScroll {
    #[default]
    None,
    Left,
    Right
}

#[derive(Component, Debug)]
pub struct CosmicData {
    pub editor: Option<Editor<'static>>,
    pub attrs: Option<Attrs<'static>>,
    pub metrics: Option<Metrics>,
    pub font_size: f32,
    pub buffer_dim: Vec2, // CosmicData's buffer dim is always bigger than text_width & and text_height
}

#[derive(Debug, Default, PartialEq)]
pub enum MoveDirection {
    #[default]
    Right,
    Left
}

impl Default for CosmicData {
    fn default() -> Self {
        Self {
            editor: None,
            attrs: None,
            metrics: None,
            font_size: f32::default(),
            buffer_dim: Vec2::default(),
        }
    }
}

#[derive(Component, Debug)]
pub struct CosmicDataColor {
    pub text_color: CosmicColor,
    pub cursor_color: CosmicColor,
    pub selection_color: CosmicColor,
    pub selected_text_color: CosmicColor,
}

impl Default for CosmicDataColor {
    fn default() -> Self {
        Self {
            cursor_color: CURSOR_INVISIBLE,
            selection_color: DEFAULT_SELECTION_COLOR,
            text_color: DEFAULT_TEXT_COLOR,
            selected_text_color: DEFAULT_SELECTED_TEXT_COLOR
        }
    }
}

impl CosmicDataColor {
    pub fn new(text_color: Color) -> Self {
        let mut cosmic_data_color = CosmicDataColor::default();

        if let Some(converted_color) = bevy_color_to_cosmic_rgba(text_color) {
            cosmic_data_color.text_color = converted_color;
            cosmic_data_color.selected_text_color = converted_color;
        }
        cosmic_data_color
    }
}

/// Component used for text editing functionality
#[derive(Component, Debug)]
pub struct FaTextEdit {
    pub value: String, // actual value when user editing
    pub placeholder: String, // whenever actual value is empty and text input is focused, draw placeholder into buffer.
    pub text_width: f32, // CosmicData's buffer dim is always bigger than text_width & and text_height
    pub text_height: f32,
    pub glyph_width: f32,
    pub cursor_index: usize,
    pub min_cursor_pos: f32,
    pub max_cursor_pos: f32,
    pub move_direction: MoveDirection,
    pub need_scroll: NeedScroll,
    pub selected_text: String,
    pub selection_start_index: Option<usize>,
    pub selection_end_index: Option<usize>,
    pub widget_computed: ComputedNode,
    pub buffer_empty: bool
}

impl Default for FaTextEdit {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            text_width: 0.0,
            text_height: 0.0,
            glyph_width: 0.0,
            cursor_index: 0,
            move_direction: MoveDirection::default(),
            min_cursor_pos: 0.0,
            max_cursor_pos: 0.0,
            need_scroll: NeedScroll::default(),
            selected_text: String::new(),
            selection_start_index: None,
            selection_end_index: None,
            widget_computed: ComputedNode::default(),
            buffer_empty: false
        }
    }
}

impl FaTextEdit {
    pub fn new(placeholder: &str) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            ..default()
        }
    }
    // pub fn move_cursor_start(&mut self) {
    //     self.cursor_index = 0;
    // }

    // pub fn move_cursor_end(&mut self) {
    //     self.cursor_index = self.value.len();
    // }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_index > 0 {
            self.cursor_index -= 1;
            self.move_direction = MoveDirection::Left;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_index < self.value.len() {
            self.cursor_index += 1;
            self.move_direction = MoveDirection::Right;
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

    pub fn remove_selected_text(&mut self) {
        if self.selection_start_index.is_some() && self.selection_end_index.is_some() {
            let start_index = self.selection_start_index.unwrap();
            let end_index = self.selection_end_index.unwrap();

            if end_index < start_index {
                self.value.drain(end_index..start_index);
                self.cursor_index = end_index;
            }
            else if end_index > start_index {
                self.value.drain(start_index..end_index);
                self.cursor_index = start_index;
            }
            self.clear_selection();
        }
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

    pub fn widget_padding_bottom(&self) -> f32 {
        self.widget_computed.padding().bottom * self.widget_scale()
    }

    pub fn widget_width(&self) -> f32 {
        self.widget_computed.size().x * self.widget_scale()
    }

    pub fn is_text_overflow(&self) -> bool {
        self.text_width > self.widget_width()
    }

    pub fn max_scroll_left(&self) -> f32 {
        0.0
    }

    pub fn max_scroll_right(&self) -> f32 {
        if self.text_width > self.widget_width() {
            self.text_width - self.widget_width()
        }
        else {
            0.0
        }
    }

    pub fn set_min_max_cursor_pos(&mut self) {
        self.min_cursor_pos = 0.0;
        self.max_cursor_pos = self.widget_width();
    }

    /// calculate cursor position for given cursor index
    pub fn calculate_cursor_pos(
        &mut self,
        glyphs: &Vec<LayoutGlyph>,
        texture_node: &Node,
        index: usize
    ) -> f32 {
        let left_val = extract_val(texture_node.left).unwrap();
        let max_index = glyphs.len().min(index);
        let mut pos = 0.0;

        for i in 0..max_index {
            self.glyph_width = glyphs[i].w;
            pos += self.glyph_width;
        }
        left_val + pos
    }

    pub fn check_need_scroll(
        &mut self,
        glyphs: &Vec<LayoutGlyph>,
        texture_node: &Node,
    ) {
        let cursor_pos = self.calculate_cursor_pos(glyphs, texture_node, self.cursor_index);
        if cursor_pos >= self.max_cursor_pos - self.glyph_width {
            self.need_scroll = NeedScroll::Right;
        }
        else if cursor_pos <= self.min_cursor_pos {
            self.need_scroll = NeedScroll::Left;
        }
        else if !self.is_text_overflow() && self.move_direction == MoveDirection::Left {
            self.need_scroll = NeedScroll::Left;
        }
        else {
            self.need_scroll = NeedScroll::None;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_text.clear();
        self.selection_start_index = None;
        self.selection_end_index = None;
    }

    /// Select all, return true if text is not empty.
    pub fn select_all(&mut self, editor: &mut Editor) -> bool {
        if !self.value.is_empty() {
            self.cursor_index = 0;
            self.selected_text = self.value.clone();
            self.selection_start_index = Some(0);
            self.selection_end_index = Some(self.value.len());
            editor.set_cursor(Cursor::new(0, 0));
            editor.set_selection(Selection::Line(editor.cursor()));
            return true;
        }
        false
    }

    /// Copy text in `fa_text_input`.
    /// - return None if text is empty.
    pub fn copy_text(&mut self) -> Option<String> {
        if !self.selected_text.trim().is_empty() {
            let mut ctx = Clipboard::new().unwrap();
            #[cfg(target_os = "linux")]
            ctx.set().clipboard(LinuxClipboardKind::Clipboard).text(self.selected_text.clone()).unwrap();
            #[cfg(not(target_os = "linux"))]
            ctx.set_text(self.selected_text.clone()).unwrap();

            if let Ok(copied_text) = ctx.get_text() {
                return Some(copied_text);
            }
        }
        None
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
