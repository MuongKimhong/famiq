# Getting Start

```rust
use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(famiq_plugin) // add plugin
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut builder_resource: ResMut<FamiqWidgetBuilderResource>, // required
    asset_server: ResMut<AssetServer>, // required
) {
    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(
        &mut commands,
        &asset_server,
        &mut builder_resource,
        "assets/fonts/some_font.ttf", // font_path
        "assets/my_styles.json",      // style_path
        true,                         // hot_reload_styles
    );

    // create simple texts using the builder
    let hello_boss = builder.fa_text("#boss", "", "Hello Boss");
    let hello_mom = builder.fa_text("#mom", "", "Hello Mom");

    // add texts to container
    builder.fa_container("#my-container", "", &vec![hello_boss, hello_mom]);
}
```
- `font_path`: Path to font source file. Must be inside `assets` folder & outside `src` folder.
- `style_path`: Path to style json file. Must be inside `assets` folder & outside `src` folder.
- `hot_reload_styles`: If `true` all changes in json file will reflect running app without needing
  to re-compile. Should be enabled only during development.

### Result
![Hello Boss Screenshot](images/helloboss_img.png)

### Widget argument patterns
From `0.2.2` onward, the first 2 arguments of the widgets are `id` and `classes`.
Just like in `HTML/CSS`, you can provide styles to widget via either `id` or `classes`.

### Apply styles
my_styles.json
```json
{
  "#mom": {
    "color": "srgba 0.961, 0.0, 0.784, 0.961"
  }
}
```

### Result
![Hello Boss 2 screenshot](images/helloboss_2_img.png)
