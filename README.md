## What is Famiq?
**Famiq** is a library that wrap around **Bevy UI** module by providing default
widgets and a simple way to manage styles. Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

## What Famiq is not?
Famiq is not a 100% battery-included. It gives you the bricks, you bring the glue.
While Famiq simplifies some aspects of working with the Bevy UI module, certain tasks,
such as handling button presses or defining behaviors, will still require you to
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

### Documentation
- [See the docs]()

### Simple Example & DEMO
- [Example]()
- [DEMO](https://imgur.com/a/qQ3aluN)

## Versions
| Bevy     | Famiq |
|----------|----------|
| 0.14.x   | 0.1.0    |


## Warning
- Many useful features are missing.
- currently support only **Bevy 0.14.x**.
- Improvements & Optimizations are needed.
- Need more default widgets
