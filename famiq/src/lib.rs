//! <p align="center">
//!  Experimental GUI library, powered by Bevy engine.
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
//!     FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();
//!
//!     let txt = text!(text: "Hello world");
//!     let btn = button!(text: "Press me");
//!     container!(children: [txt, btn]);
//!
//!     // or
//!
//!     container!(children: [
//!         text!(text: "Hello world"),
//!         button!(text: "Press me")
//!     ]);
//! }
//! ```

#![deny(ambiguous_glob_reexports)]
pub mod errors;
pub mod resources;
pub mod event_writer;
pub mod plugin;
pub mod utils;
pub mod widgets;
pub mod reactivity;

pub mod prelude {
    pub use crate::plugin::FamiqPlugin;
    pub use crate::resources::*;
    pub use crate::reactivity::*;
    pub use crate::utils::*;
    pub use crate::widgets::{
        FamiqBuilder, WidgetType, WidgetId,
        DefaultTextConfig, DefaultWidgetConfig,
        SetWidgetAttributes, FaQuery, WidgetSelector,
        SetupWidget, WidgetBuilder, BuilderType,
        builder_mut,
        text::TextBuilder,
        button::ButtonBuilder,
        container::ContainerBuilder,
        image::ImageBuilder,
        checkbox::CheckboxBuilder,
        circular::CircularBuilder,
        text_input::TextInputBuilder,
        selection::SelectionBuilder,
        dialog::DialogBuilder,
        progress_bar::ProgressBarBuilder,
        scroll::ScrollBuilder,
        fps::FpsBuilder,
    };
    pub use famiq_macros::*;
    pub use crate::button;
    pub use crate::checkbox;
    pub use crate::circular;
    pub use crate::container;
    pub use crate::fps;
    pub use crate::image;
    pub use crate::scroll;
    pub use crate::dialog;
    pub use crate::progress_bar;
    pub use crate::selection;
    pub use crate::text;
    pub use crate::text_input;
    pub use crate::event_writer::FaMouseEvent;
    pub use crate::errors::*;
    pub use bevy::platform::collections::HashMap;
    pub use serde_json;
}

pub use prelude::*;
