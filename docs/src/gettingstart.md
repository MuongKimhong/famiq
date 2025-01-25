# Getting Start

```rust
use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin) // add plugin
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut builder_res: ResMut<FamiqWidgetResource>, // required
    asset_server: ResMut<AssetServer>, // required
) {
    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(
        &mut commands,
        &mut builder_res
        &asset_server,
    );

    // create simple texts using the builder
    let hello_boss = fa_text(&mut builder, "Hello Boss").build();
    let hello_mom = fa_text(&mut builder, "Hello Mom").build();

    // add texts to container
    fa_container(&mut builder)
        .children(vec![hello_boss, hello_mom])
        .build();
}
```

![Hello Boss Screenshot](images/helloboss_img.png)

### Custom font
By default, Famiq uses `Fira mono regular` as font. To use custom font, you can simply call
`use_font_path()` method.
```rust
// path is relative to assets folder outside src directory.
builder.use_font_path("path/to/font.ttf");
```
⚠️ some fonts might cause rendering issue including positioning and styling.

### Custom json file for styling
By default, Famiq will look for json file for styling at `assets/styles.json`. If you want to use another path or name, for example `assets/styles/widget_styles.json`, you can simply call `use_style_path()` method.
```rust
builder.use_style_path("assets/styles/widget_styles.json");
```

### Hot reload
Hot-reload can be enabled during development. When it's enabled, every changes in json
file will reflect the running app immediately without needing to re-compile the app.
```rust
builder.hot_reload();
```
