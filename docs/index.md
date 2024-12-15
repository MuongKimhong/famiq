## What is Famiq?
**Famiq** is a library that wrap around **Bevy UI** module by providing default
widgets and a simple way to manage styles. Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

## What Famiq is not?
Famiq is not 100% battery-included. It gives you the bricks, you bring the glue.
While Famiq simplifies some aspects of working with the Bevy UI module, certain tasks,
such as handling button presses or defining custom behaviors, will still require you to
write your own systems and logic.

```rust
// default button with white background color
let btn = ui.fa_button("#mybtn", "Press me", None, None);
```
if you want to change background color to something else
```json
{
  "#mybtn": {
    "background_color": "srgba 0.357, 0.565, 0.941, 0.902"
  }
}
```
hot-reload can be enabled during development so that any changes made to the json file
are reflected immediately in the running app, no need to re-compile.

## Getting start
```rust
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
    asset_server: ResMut<AssetServer>, // required
    mut builder_resource: ResMut<FamiqWidgetBuilderResource>, // required
) {
    // create a widget builder
    let mut builder = FamiqWidgetBuilder::new(
        &mut commands,
        &asset_server,
        &mut builder_resource,
        "assets/fonts/some_font.ttf", // should be inside assets folder & outside src folder
        "assets/mystyles.json",       // should be inside assets folder & outside src folder
        true,                         // hot_reload_styles, useful during development
    );

    // create widgets using the builder
    let txt = builder.fa_text("#mytxt", "Hello Boss");
    builder.fa_container("#mycontainer", &vec![txt]); // add txt to container
}
```
if `hot_reload_styles` is true, famiq will read the json file, parse the styles and apply it to the widget(s)
every single frame. Should be enabled only during development.

### Styling
[How styling in json file works?](https://muongkimhong.github.io/famiq/styling)

### Interaction Events
[Handle interaction events including Press, Hover, None (Leave)](https://muongkimhong.github.io/famiq/interactionevents)

## Default Widgets
- [Container - fa_container](https://muongkimhong.github.io/famiq/widgets/container)
- [Button - fa_button](https://muongkimhong.github.io/famiq/widgets/button)
- [ListView - fa_listview](https://muongkimhong.github.io/famiq/widgets/listview)
- [Text - fa_text](https://muongkimhong.github.io/famiq/widgets/text)
- [Selection - fa_selection](https://muongkimhong.github.io/famiq/widgets/selection)
- [TextInput - fa_text_input](https://muongkimhong.github.io/famiq/widgets/textinput)

## Warning
- Many useful features are missing.
- currently support only **Bevy 0.14.x**.
- A better documentation is required.
- Improvements & Optimizations are needed.
