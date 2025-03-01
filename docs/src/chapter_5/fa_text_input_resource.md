# FaTextInputResource

Use this resource to get value from `fa_text_input`.

To use this resource on `fa_text_input`, `id` must be provided.

```rust
fn my_system(mut input_res: ResMut<FaTextInputResource>) {
    // some logic

    let name: String = input_res.get_value("#name-input");
}
```
