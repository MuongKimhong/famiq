# FaTextInput

Widget that allow user to type in texts.

Currently, this widget is not interactive.

### Usage
```rust
let input = fa_text_input(&mut builder, "Enter your name").build();
```
Return `Entity` which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `is_password()`: mask all the character as `*`.
- `color(&str)`: set custom background color.

### Example
```rust
// default
let input_default = fa_text_input(&mut builder, "Enter your name").build();

// password
let input_password = fa_text_input(&mut builder, "Enter your password")
    .is_password()
    .build();

fa_container(&mut builder)
    .children([input_default, input_password])
    .build();
```
