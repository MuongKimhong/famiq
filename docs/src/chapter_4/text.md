# FaText

### Usage
```rust
let text = fa_text(&mut builder, "Some text").build();
```
Return `Entity` of the widget which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `color(&str)`: set custom color.

### Example
```rust
let boss = fa_text(&mut builder, "Hello Boss").build();
let mom = fa_text(&mut builder, "Hello Mom").build();

fa_container(&mut builder).children([boss, mom]).build();
```
