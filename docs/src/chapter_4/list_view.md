# FaListView

Vertical scrollable container.

**Note**

`fa_listview` has height set to 50% of the `window` or its `parent` container height.

### Usage
```rust
let button = fa_button(&mut builder, "Press me").build();
let input = fa_text_input(&mut builder, "Enter your name").build();

fa_listview(&mut builder).children([input, button]).build();
```
return `Entity`.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `scroll_height(f32)`
- `children([Entity])`
