pub mod errors;
pub mod event_writer;
pub mod plugin;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::plugin::FamiqPlugin;
    pub use crate::widgets::{
        FamiqWidgetBuilder, FamiqWidgetResource, WidgetType, FamiqWidgetId,
        DefaultTextEntity, DefaultWidgetEntity,
        fa_button,
        fa_circular,
        fa_container,
        fa_fps,
        fa_image
    };
    pub use crate::widgets::modal::{IsFamiqModalContainer, IsFamiqModalBackground, FaModalState};
    pub use crate::widgets::text_input::{IsFamiqTextInput, FaTextInputResource};
    pub use crate::widgets::selection::{IsFamiqSelectionSelector, SelectedChoicesResource};
    pub use crate::widgets::container::IsFamiqContainer;
    pub use crate::widgets::fps::IsFamiqFPSTextCount;
    pub use crate::widgets::text::IsFamiqText;
    pub use crate::widgets::list_view::IsFamiqListView;
    pub use crate::event_writer::FaInteractionEvent;
}
