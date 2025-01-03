# FaModal

```
🟢 Doesn't need container
🟢 Accepts child/children
```

### API
```rust
pub fn fa_modal(&mut self, id: &str, items: &Vec<Entity>) {
    // ..
}
```

### Usage via builder
```rust
builder.fa_modal(..);
```

### Show/Hide modal
Modals can be shown or hided by updating `FaModalState` component.

```rust
fn show_or_hide_modal_system(mut modal: Query<(&FamiqWidgetId, &mut FaModalState)>) {
    let should_visible = true;

    for (id, mut state) in modal.iter_mut() {
        if id.0 == "#my-modal" && should_visible {
            state.0 = true; // show
        }
    }
}
```
