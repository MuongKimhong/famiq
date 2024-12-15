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

## Versions
| Bevy     | Famiq |
|----------|----------|
| 0.14.x   | 0.1.0    |

## Warning
- Many useful features are missing.
- currently support only **Bevy 0.14.x**.
- A better documentation is required.
- Improvements & Optimizations are needed.
