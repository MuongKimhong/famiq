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
//!     mut commands: Commands, // required
//!     mut famiq_res: ResMut<FamiqResource>, // required
//!     asset_server: Res<AssetServer> // required
//! ) {
//!     commands.spawn(Camera2d::default());
//!
//!     let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server);
//!
//!     let txt = fa_text(&mut builder, "Hello world").build();
//!     let btn = fa_button(&mut builder, "Press me").build();
//!
//!     fa_container(&mut builder).children([txt, btn]).build();
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
        SetWidgetAttributes,
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
    pub use crate::widgets::modal::{
        IsFamiqModalContainer,
        IsFamiqModalBackground,
        FaModalState,
        FaModalResource
    };
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
