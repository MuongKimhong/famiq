# FaFpsText

Show **FPS** value at top-left corner of the window.

This widget doesn't need to be inside a containable widget.

### Usage
```rust
fa_fps(&mut builder).build();
```
return `Entity`.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `change_color()`: change number color based on its value.
- `right_side()`: make **FPS** text appears at the top right corner.

### Example
```rust
fa_fps(&mut builder).change_color().right_side().build();
```
