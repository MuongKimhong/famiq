pub mod errors;
pub mod event_handler;
pub mod event_writer;
pub mod plugin;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::event_writer::FaInteractionEvent;
    pub use crate::plugin::famiq_plugin;
    pub use crate::widgets::{
        selection::SelectedItemsResource, text_input::TextInputResource, DefaultTextBundle,
        DefaultWidgetBundle, FaWidgetBundle, FamiqWidgetBuilder, FamiqWidgetBuilderResource,
        FamiqWidgetId, WidgetType,
    };
}
