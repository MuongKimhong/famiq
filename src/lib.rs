//! <p align="center">
//!  <img src="https://i.imgur.com/C5GvBd2.png" width="250" alt="Widget Styles Logo">
//! </p>
//!
//! <p align="center">
//!  Build beatiful desktop GUI with pure rust, powered by Bevy game engine.
//! </p>
//!
//! Example:
//! ```rust, no_run
//! use bevy::prelude::*;
//! use famiq::prelude::*;
//!
//! fn main() {
//!     App::new()
//!        .add_plugins(DefaultPlugins)
//!        .add_plugins(FamiqPlugin) // add plugin
//!        .add_systems(Startup, setup_ui)
//!        .run();
//! }
//!
//! fn setup_ui(
//!     mut fa_query: FaQuery, // required
//!     mut famiq_res: ResMut<FamiqResource>, // required
//! ) {
//!     let mut builder = FamiqBuilder::new(&mut fa_query, &mut famiq_res);
//!
//!     let txt = fa_text(&mut builder, "Hello world").build();
//!     let btn = fa_button!(&mut builder, text: "Press me");
//!
//!     fa_container!(&mut builder, children: [txt, btn]);
//! }
//! ```

pub mod errors;
pub mod resources;
pub mod event_writer;
pub mod plugin;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::plugin::FamiqPlugin;
    pub use crate::resources::*;
    pub use crate::widgets::{
        FamiqBuilder, WidgetType, FamiqWidgetId,
        DefaultTextEntity, DefaultWidgetEntity,
        SetWidgetAttributes, FaQuery, WidgetSelector,
        fa_button_builder,
        fa_circular_builder,
        fa_container_builder,
        fa_fps_builder,
        fa_image_builder,
        fa_listview_builder,
        fa_modal_builder,
        fa_text_builder,
        fa_text_input_builder,
        fa_selection_builder,
        fa_bg_image_builder,
        fa_progress_bar_builder,
        fa_checkbox_builder,
    };
    pub use crate::fa_button;
    pub use crate::fa_bg_image;
    pub use crate::fa_checkbox;
    pub use crate::fa_circular;
    pub use crate::fa_container;
    pub use crate::fa_fps;
    pub use crate::fa_image;
    pub use crate::fa_listview;
    pub use crate::fa_modal;
    pub use crate::fa_progress_bar;
    pub use crate::fa_selection;
    pub use crate::fa_text;
    pub use crate::fa_text_input;
    pub use crate::children;
    pub use crate::widgets::modal::{IsFamiqModalContainer, IsFamiqModalBackground};
    pub use crate::widgets::text_input::{IsFamiqTextInput, FaTextInputResource};
    pub use crate::widgets::selection::{IsFamiqSelectionSelector, FaSelectionResource};
    pub use crate::widgets::progress_bar::{IsFamiqProgressBar, IsFamiqProgressValue, FaProgressBarResource};
    pub use crate::widgets::container::IsFamiqContainer;
    pub use crate::widgets::fps::IsFamiqFPSTextCount;
    pub use crate::widgets::text::IsFamiqText;
    pub use crate::widgets::list_view::IsFamiqListView;
    pub use crate::widgets::bg_image::{FaBgImageResource, IsFamiqBgImage};
    pub use crate::event_writer::{FaMouseEvent, FaValueChangeEvent};
    pub use bevy::utils::hashbrown::HashMap;
}

pub use prelude::*;
