pub mod errors;
pub mod event_writer;
pub mod plugin;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::plugin::FamiqPlugin;
    pub use crate::widgets::{
        FamiqBuilder, FamiqResource, WidgetType, FamiqWidgetId,
        DefaultTextEntity, DefaultWidgetEntity, InputResourceMap,
        SetWidgetAttributes, ContainableResourceAction,
        fa_button,
        fa_circular,
        fa_container,
        fa_fps,
        fa_image,
        fa_listview,
        fa_modal,
        fa_text,
        fa_text_input,
        fa_selection,
        fa_bg_image,
        fa_progress_bar
    };
    pub use crate::widgets::modal::{IsFamiqModalContainer, IsFamiqModalBackground, FaModalState};
    pub use crate::widgets::text_input::{IsFamiqTextInput, FaTextInputResource};
    pub use crate::widgets::selection::{IsFamiqSelectionSelector, FaSelectionResource};
    pub use crate::widgets::progress_bar::{IsFamiqProgressBar, IsFamiqProgressValue, FaProgressBarResource};
    pub use crate::widgets::container::{IsFamiqContainer, FaContainerResource};
    pub use crate::widgets::fps::IsFamiqFPSTextCount;
    pub use crate::widgets::text::{IsFamiqText, FaTextResource};
    pub use crate::widgets::list_view::{IsFamiqListView, FaListViewResource};
    pub use crate::widgets::bg_image::{FaBgImageResource, IsFamiqBgImage};
    pub use crate::event_writer::FaInteractionEvent;
}
