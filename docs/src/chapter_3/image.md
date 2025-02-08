# FaImage

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### API
```rust
pub fn fa_image<'a>(builder: &'a mut FamiqWidgetBuilder, path: &str) -> FaImageBuilder<'a> {
    // ..
}
```
- `path`: Path to image relative to `assets` folder.
- Return entity of the widget which must be used as child of `FaContainer` widget.
- Support only `jpg` and `png` format.

### Usage
```rust
let image = fa_image(&mut builder, "path/to/image.jpg").build();
```

### Custom size
By default, `fa_image` will load image at the original size. To use custom size, simply call `size()` method.
```rust
let image = fa_image(&mut builder, "path/to/image.jpg")
    .size(Val::Px(200.0), Val::Px(200.0))
    .build();
```

### Example
```rust
let famiq_logo = fa_image(&mut builder, "logo.jpeg")
    .size(Val::Px(100.0), Val::Px(100.0))
    .build();

fa_container(&mut builder).children([famiq_logo]).build();
```
![Example 1](../images/image_example_1.png)
