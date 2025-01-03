## What is Famiq?
**Famiq** is a UI library wrapped around Bevy UI module by providing default
widgets and a simple way to manage styles. Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

```rust
// default button with white background color
let btn = builder.fa_button("#mybtn", "Press me", "", "");
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

### Installation
```toml
[dependencies]
famiq = { git = "https://github.com/muongkimhong/famiq", tag = "v0.2.0" }
```

### Documentation
- [See the docs](https://muongkimhong.github.io/famiq/)

### Versions
| Bevy     | Famiq |
|----------|----------|
| 0.14.x   | 0.1.0    |
| 0.15.x   | 0.2.0    |


### Warning
- Many useful features are missing.
- Improvements & Optimizations are needed.
- Need more default widgets
