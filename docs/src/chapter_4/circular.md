# FaCircular

A spinning circular.

### Usage
```rust
let circular = fa_circular(&mut builder).build();
```
Return `Entity` which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `size(f32)`: set custom size (width & height).
- `color(&str)`: set custom color.

### Example
```rust
// default
let cir = fa_circular(&mut builder).build();

// warning & small
let warning_cir = fa_circular(&mut builder)
    .class("is-warning is-small")
    .build();

// primary & custom size
let primary_cir = fa_circular(&mut builder)
    .class("is-primary")
    .size(50.0)
    .build();

// custom color
let custom_color_cir = fa_circular(&mut builder)
    .color("cyan_500")
    .build();

fa_container(&mut builder)
    .children([cir, warning_cir, primary_cir, custom_color_cir])
    .build();
```
