
#### ⚠️ IMPORTANT
Famiq is built on top [Bevy](https://bevyengine.org/) and relies entirely on its **ECS architecture**. When you use **Famiq**,
you're actually using the **Bevy engine**, with **Famiq** providing an abstraction for its UI system
to help you build GUI applications, rather than Game UI.

A solid understanding of **Bevy's ECS** is required. If you're new to **Bevy**,
I recommend checking out [Bevy's quick start guide](https://bevyengine.org/learn/quick-start/introduction/).

# Getting Start
```rust
use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin) // required by Famiq
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d::default());

    // create a builder
    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server);

    // simple text & button widgets
    let text = fa_text(&mut builder, "Hello world").build();
    let button = fa_button(&mut builder, "Press me").build();

    // add widgets to a container
    fa_container(&mut builder).children([text, button]).build();
}
```
run your project `cargo run`, you will see a text and a button.

### What is **`FamiqPlugin`**?
`FamiqPlugin` brings in all the required `Resources` & `Internal systems` in order to run the app.
It must be registered after `DefaultPlugins` provided by Bevy.

### What is **`FamiqBuilder`**?
In simple terms, `FamiqBuilder` is the root UI node that acts as a starting point for building and managing widgets. All widgets are created and structured on top of this root.

There are methods provided by `FamiqBuilder` including:
- [use_font_path()](#-use_font_path)
- [register_tooltip()](#-register_tooltip)
- [use_style_path()](#-use_style_path)
- [hot_reload()](#-hot_reload)

### 🔵 `use_font_path()`
By default, Famiq uses `Fira mono regular` as default font. To use another font, you can
simply call `use_font_path()` method.
#### Example
- **For normal project structure**:
    ```text
    my_project/
    ├── assets/
    │   ├── fonts/
    │   │   ├── Some-font.ttf
    ├── src/
    ```

    ```rust
    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
        .use_font_path("fonts/Some-font.ttf");
    ```
- **For Multi-Crate/Workspace project structure**:
    In a multi-crate workspace, the custom font path is read from the subcrate/member's `assets/` folder:

    ```text
    my_project/
    ├── sub_crate_1/
    │   ├── assets/
    │   │   ├── fonts/
    │   │   │   ├── Some-font.ttf
    │   ├── src/
    ├── sub_crate_2/
    │   ├── assets/
    │   ├── src/
    ```

    ```rust
    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
        .use_font_path("fonts/Some-font.ttf");
    ```
⚠️ some fonts might cause rendering issue including positioning and styling.

### 🔵 `register_tooltip()`
This method enable tooltip option for some widgets. Currently only `fa_button` and `fa_circular` support tooltip option.
#### Note
If `use_font_path` is called, `register_tooltip` must be called **after** `use_font_path`
to ensure that the custom font is applied to the tooltip.
```rust
let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
    .use_font_path("fonts/Some-font.ttf") // if use_font_path is called
    .register_tooltip();
```

### 🔵 `use_style_path()`
By default, Famiq will look for json file for styling at `assets/styles.json`, relative to root directory.
If you want to use another path or name, you can simply call `use_style_path()` method.
#### Note
- **For Multi-Crate/Workspace project structure**: if you have json file inside sub-crate `assets` directory, you need to specify
full path relative to root directory, not sub-crate.
```rust
// normal project
let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
    .use_style_path("assets/my-styles.json");

// multi crate/workspace
let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
    .use_style_path("path/to/sub-crate/assets/subcrate-style.json");
```

### 🔵 `hot_reload()`
This method will enable hot-reload. When it's enabled, every changes in json
file will reflect the running app immediately without needing to re-compile the app.
This should only be used during development.
```rust
let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server)
    .hot_reload();
```

### Types of widgets
There are 2 types of widgets provided by **Famiq** which are **Containable** widget and **Non-containable** widgets.

- **Containable** widgets can have child widgets and can also be nested inside other containable widgets, including `fa_container`, `fa_listview` and `fa_modal`.

- **Non-containable** widgets cannot have children and must be placed inside a containable widget. including `fa_text`,
  `fa_button`, `fa_text_input`, `fa_selection`, `fa_image`, `fa_circular` and `fa_progress_bar`.

As you can see in the code above, `fa_text` & `fa_button` were placed inside `fa_container`.
