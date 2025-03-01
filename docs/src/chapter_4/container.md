# FaContainer

An empty and stylyable widget. Think of it as a **div** inside HTML.
It can also be used to create custom widgets.

**Note**

- `fa_container` has its default height set to `auto`, meaning its height
depends on its children.

- If you use `fa_container` to create a custom widget without `children`, you
must set the height to a specific value.

### usage
```rust
let container = fa_container(&mut builder).build();
```
Return `Entity` of the widget which can optionally be used as child for another containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `children([Entity])`

### Example
```rust
let boss = fa_text(&mut builder, "Hello Boss").build();
let mom = fa_text(&mut builder, "Hello Mom").build();

fa_container(&mut builder)
    .id("#my-container")
    .children([boss, mom])
    .build();
```
