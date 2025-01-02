pub mod errors;
pub mod event_writer;
pub mod plugin;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::plugin::famiq_plugin;
    pub use crate::widgets::{
        FamiqWidgetBuilder, FamiqWidgetBuilderResource, WidgetType, FamiqWidgetId,
        DefaultTextEntity, DefaultWidgetEntity
    };
    pub use crate::widgets::modal::FaModalState;
    pub use crate::widgets::text_input::FaTextInputResource;
    pub use crate::widgets::selection::SelectedChoicesResource;
    pub use crate::event_writer::FaInteractionEvent;
}
