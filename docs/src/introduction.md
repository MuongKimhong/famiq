# What is Famiq?

**Famiq** is a UI library wrapped around Bevy UI module by providing default
widgets and a simple way to manage styles. Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

### Simple button
```rust
let my_btn = builder.fa_button("#my-btn", "Press me", "info", "large");
```

### Change background color
```json
{
  "#my-btn": {
    "background_color": "srgba 0.357, 0.565, 0.941, 0.902"
  }
}
```

### Hot reload
Hot-reload can be enabled during development. When it's enabled, every changes in json
file will reflect the running app immediately without needing to re-compile the app.

### Bevy versions support
Famiq is new and still in early stage of development. Currently, it supports only Bevy 0.14.x and 0.15.x.

| Bevy     | Famiq    |
|----------|----------|
| 0.14.x   | 0.1.0    |
| 0.15.x   | 0.2.0    |
