# FaTextResource

Use this resource to **get** or **update** `fa_text`'s value.

To use this resource on `fa_text`, `id` must be provided.

```rust
fn my_system(mut text_res: ResMut<FaTextResource>) {
    // Some logic

    let title = text_res.get_value("#text-id");
    println!("{:?}", title); // How to play dota 2

    // update title
    text_res.update_value("#text-id", "How to play dota 2 in 2025");
}
```
