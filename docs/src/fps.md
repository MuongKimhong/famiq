# FPS

High **fps** means **fast & smooth**.

By default, `bevy` uses a present mode called `Fifo`, traditionally called **"Vsync On"**.  This synchronizes your app’s FPS with your monitor’s refresh rate.

So, no matter how powerful your **GPU** is, if your monitor runs at **60Hz**, your app will be capped at **60 FPS**.

To fully utilize your GPU and unlock higher frame rates, switch to the **Immediate** present mode in **WindowPlugin**. This removes the FPS cap but may cause **screen tearing**.

### Example
```rust
use bevy::prelude::*;
use bevy::window::PresentMode; // make sure to import this
use famiq::prelude::*;

fn custom_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Some title".into(),
            name: Some("Some app name".into()),
            resolution: (1000.0, 800.0).into(),
            resizable: false,
            present_mode: PresentMode::Immediate, // here
            ..default()
        }),
        ..default()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(custom_window())) // set custom window into DefaultPlugins
        .add_plugins(FamiqPlugin)
        .add_systems(StartUp, setup_ui)
        .run();
}

fn setup_ui() {
    // some widgets ..
}
```
