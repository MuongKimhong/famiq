# Customising your app window

Currently, the only way to customise your window is via **bevy**'s `WindowPlugin`.

see more about [WindowPlugin](https://docs.rs/bevy/latest/bevy/window/struct.WindowPlugin.html).

### Example
```rust
use bevy::prelude::*;
use famiq::prelude::*;

fn custom_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Some title".into(),
            name: Some("Some app name".into()),
            resolution: (1000.0, 800.0).into(),
            resizable: false,
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
