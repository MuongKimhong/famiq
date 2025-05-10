
#### ⚠️ IMPORTANT
Famiq is built on top [Bevy](https://bevyengine.org/) and relies entirely on its **ECS architecture**. When you use **Famiq**,
you're actually using the **Bevy engine**, with **Famiq** providing an abstraction for its UI system
to help you build GUI applications, rather than Game.

A solid understanding of **Bevy's ECS** is required. If you're new to **Bevy**,
I recommend checking out [Bevy's quick start guide](https://bevyengine.org/learn/quick-start/introduction/).

# Getting Start
```rust
use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin::new()) // required by Famiq
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    // inject builder
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();

    // simple text & button
    container!(
        children: [
            text!(text: "Hello world"),
            button!(text: "Press me")
        ]
    );
}
```
run your project `cargo run`, you will see a text and a button.

### What is **`FamiqPlugin`**?
`FamiqPlugin` brings in all the required `Resources` & `Internal systems` in order to run the app.
It must be registered after `DefaultPlugins` provided by Bevy.

- `new()`: use this method for default settings with 2d `camera`.
- `new_no_camera()`: use this method if you want to spawn your own `camera` either 2d or 3d.

see [FamiqPlugin](https://docs.rs/famiq/latest/famiq/plugin/struct.FamiqPlugin.html).

### What is **`FaQuery`**?
`FaQuery` is like `document` in HTML/Javascript. It allows us to interact with widgets including
**inserting reactive data**, **update reactive data**, etc.

see [FaQuery](https://docs.rs/famiq/latest/famiq/widgets/struct.FaQuery.html).

### What is **`FamiqBuilder`**?
In simple terms, `FamiqBuilder` is the root UI node that acts as a starting point for building and managing widgets. All widgets are created and structured on top of this root. **Before creating any UI widgets, `FamiqBuilder` must be created and injected.**

There are methods provided by `FamiqBuilder` including:
- [use_font_path()](#-use_font_path)
- [use_style_path()](#-use_style_path)
- [hot_reload()](#-hot_reload)
- [inject()](#-inject)

see [FamiqBuilder](https://docs.rs/famiq/latest/famiq/widgets/struct.FamiqBuilder.html).

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
    FamiqBuilder::new(&mut fa_query, &mut famiq_res)
        .use_font_path("fonts/Some-font.ttf")
        .inject();
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
    FamiqBuilder::new(&mut fa_query, &mut famiq_res)
        .use_font_path("fonts/Some-font.ttf")
        .inject();
    ```
⚠️ some fonts might cause rendering issue including positioning and styling.

### 🔵 `use_style_path()`
By default, Famiq will look for json file for styling at `assets/styles.json`, relative to root directory.
If you want to use another path or name, you can simply call `use_style_path()` method.
#### Note
- **For Multi-Crate/Workspace project structure**: if you have json file inside sub-crate `assets` directory, you need to specify
full path relative to root directory, not sub-crate.
- **Wasm build**: if you target wasm, you need to explicitly call `use_style_path()`, where path is relative to assets directory.
```rust
// normal project
FamiqBuilder::new(&mut fa_query, &mut famiq_res)
    .use_style_path("assets/my-styles.json")
    .inject();

// wasm build
FamiqBuilder::new(&mut fa_query, &mut famiq_res)
    .use_style_path("my-styles.json")
    .inject();

// multi crate/workspace
FamiqBuilder::new(&mut fa_query, &mut famiq_res)
    .use_style_path("path/to/sub-crate/assets/subcrate-style.json")
    .inject();
```

### 🔵 `hot_reload()`
This method will enable hot-reload (exclude wasm). When it's enabled, every changes in json
file will reflect the running app immediately without needing to re-compile the app.
This should only be used during development.
```rust
FamiqBuilder::new(&mut fa_query, &mut famiq_res)
    .hot_reload()
    .inject();
```

### 🔵 `inject()`
This method must be called to inject builder into all widgets.

### Types of widgets
There are 2 types of widgets provided by **Famiq** which are **Containable** widget and **Non-containable** widgets.

- **Containable** widgets can have children and can also be nested inside other containable widgets, including `container`, `scroll` and `dialog`.

- **Non-containable** widgets cannot have children and must be placed inside a containable widget. including `text`,
  `button`, `text_input`, `selection`, `image`, `circular` and `progress_bar`.

As you can see in the example code above, `text` & `button` were placed inside `container`.
