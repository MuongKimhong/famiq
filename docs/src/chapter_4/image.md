# FaImage

Image widget.

**Note**

Currently, it supports only `jpg` and `png` format.

### Usage
```rust
let image = fa_image(&mut builder, "path/to/image.jpg").build();
```
return `Entity` which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `set_size(Val, Val)`: set custom size (width, height). If this method is not called, image's original `width` & `height` will  be used.

### Example
```rust
let famiq_logo = fa_image(&mut builder, "logo.png")
    .set_size(Val::Px(100.0), Val::Px(100.0))
    .build();

fa_container(&mut builder).children([famiq_logo]).build();
```
