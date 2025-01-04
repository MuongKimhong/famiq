# FaImage

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### API
```rust
pub fn fa_image(&mut self, id: &str, path: &str) -> Entity {
    // ..
}
```
- `path`: Path to image relative to `assets` folder.
- Return entity of the widget which must be used as child of `FaContainer` widget.

### Usage via builder
```rust
let image = builder.fa_image(..);

builder.fa_container("#container", &vec![image]);
```

### Example
```rust
let bevy_logo = builder.fa_image("#bevy-logo", "bevylogo.png");
let falcon = builder.fa_image("#falcon", "falcon.png");

builder.fa_container("#container", &vec![bevy_logo, falcon]);
```
![Example 1](../images/image_example_1.png)
