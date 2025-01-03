use crate::widgets::selection::*;

pub fn get_text_size(size: &SelectionSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        SelectionSize::Small => size_small,
        SelectionSize::Normal => size_normal,
        SelectionSize::Large => size_large,
    };
    text_size
}
