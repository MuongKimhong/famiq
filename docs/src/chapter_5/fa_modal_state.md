# FaModalState

Use this resource to **show** or **hide** `fa_modal`.

Only one `fa_modal` can be shown at a time.

To use this resource on `fa_modal`, `id` must be provided.

```rust
fn my_system(mut modal_state: ResMut<FaModalState>) {
    // Some logic

    modal_state.show("#welcome-modal");

    // or

    modal_state.hide("#other-modal");
}
```
