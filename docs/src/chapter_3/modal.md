# FaModal

```
🟢 Doesn't need container
🟢 Accepts child/children
```

### API
```rust
pub fn fa_modal<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaModalBuilder<'a> {
    // ..
}
```

### Usage
```rust
let modal = fa_modal(&mut builder).build();
```

### Resource
```rust
pub struct FaModalState;
```
- `FaModalState` can be used to show and hide specific modal by either `id` or `entity`.
  #### Available methods:
  - `show_by_id`: show modal by id.
  - `show_by_entity`: show modal by entity.
  - `hide_by_id`: hide modal by id.
  - `hide_by_entity`: hide modal by entity.

  #### Example of using `FaModalState`
  ```rust
  fn my_system(mut modal_state: ResMut<FaModalState>) {
      // some logic ..

      // show modal
      modal_state.show_by_id("#modal-id");

      // hide modal
      modal_state.hide_by_id("#modal-id");
  }
  ```
