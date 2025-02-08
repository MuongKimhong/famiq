# What is Famiq?

**Famiq** is a UI library built on top of Bevy UI by providing default
widgets and a simple way to manage styles.

Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

Built on top of Bevy UI, based on Bevy ECS.
- **Simple**: Follows Bevy's philosophy—widgets are just Rust functions.
- **Clean**: Styles can be defined in a JSON file, reducing boilerplate code.
- **Widgets**: Includes useful default components like button, modal, listview and more.
- **Flexible**: Just like in `HTML/CSS`, styles can be applied using `id` or `classes`.

### Simple button
```rust
let my_btn = fa_button(&mut builder, "Press me").build();
```
. `&mut builder` is a mutable reference of `FamiqWidgetBuilder`.


If you want to make changes to the widget, you can simply give it an `id` or `class`.
```rust
let my_btn = fa_button(&mut builder, "Press me").id("#my-btn").build();
```
```json
{
  "#my-btn": {
    "background_color": "yellow"
  }
}
```

### Hot reload
Hot-reload can be enabled during development. When it's enabled, every changes in json
file will reflect the running app immediately without needing to re-compile the app.
```rust
let mut builder = FamiqWidgetBuilder::new(
    &mut commands,
    &mut builder_res,
    &asset_server
);

builder.hot_reload();
```

### Bevy versions support
Famiq is new and still in early stage of development. Currently, it supports only 0.15.x onward.
