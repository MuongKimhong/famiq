# FaImage

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### API
```rust
pub fn fa_image(&mut self, id: &str, classes: &str, path: &str) -> Entity {
    // ..
}
```
- `path`: Path to image relative to `assets` folder.
- Return entity of the widget which must be used as child of `FaContainer` widget.

### Usage via builder
```rust
let image = builder.fa_image(..);
```

### Example
```rust
let man = builder.fa_image("#bevy-logo", "", "300px", "300px", "man.JPG");

builder.fa_container("#container", "", &vec![man]);
```
![Example 1](../images/image_example_1.png)
