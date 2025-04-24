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
//!     inject_builder(&mut builder);
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
        inject_builder,
        text::*,
        button::*,
        container::*,
        image::*,
        checkbox::*,
        circular::*,
        text_input::*,
        selection::*,
        modal::*,
        progress_bar::*,
        scroll::*,
        fps::*,
    };
    pub use macros::*;
    pub use args::prelude::*;
    pub use crate::button;
    pub use crate::checkbox;
    pub use crate::circular;
    pub use crate::container;
    pub use crate::fps;
    pub use crate::image;
    pub use crate::scroll;
    pub use crate::modal;
    pub use crate::progress_bar;
    pub use crate::selection;
    pub use crate::text;
    pub use crate::text_input;
    pub use crate::event_writer::{FaMouseEvent, FaValueChangeEvent};
    pub use crate::errors::*;
    pub use bevy::utils::hashbrown::HashMap;
    pub use serde_json;
}

pub use prelude::*;
