# FaSelectionResource

Use this resource to get value from `fa_selection`.

To use this resource on `fa_selection`, `id` must be provided.

```rust
fn my_system(mut selection_res: ResMut<FaSelectionResource>) {
    // some logic

    let choice: String = selection_res.get_value("#selection-id");
}
```
