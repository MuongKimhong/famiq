# FaBgImageResource

Use this resource to change window's background image.

```rust
fn my_system(mut bg_res: ResMut<FaBgImageResource>) {
    // some logic

    bg_res.change_image("path/to/image-in-assets.png");
}
```
