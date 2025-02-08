# FaText

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### API
```rust
pub fn fa_text<'a>(builder: &'a mut FamiqWidgetBuilder, value: &str) -> FaTextBuilder<'a> {
    // ..
}
```

### Usage
```rust
let text = fa_text(&mut builder, "Some text").build();
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Example
```rust
let boss = fa_text(&mut builder, "Hello Boss").id("#boss-txt").build();
let mom = fa_text(&mut builder, "Hello Mom").build();

fa_container(&mut builder).children([boss, mom]).build();
```
![Example 2](../images/container_example_2.png)

### Resource
```rust
pub struct FaTextResource;
```
- `FaTextResource` can be used to get & update specific text widget's value by either its `id` or `entity`.
  #### Available methods:
  - `get_value_by_id`: get `fa_text` value by id, return `empty string` if id doesn't exist.
  - `get_value_by_entity`: get `fa_text` value by entity, return `empty string` if entity doesn't exist.
  - `update_value_by_id`: update `fa_text` value by id.
  - `update_value_by_entity`: update `fa_text` value by entity.

  #### Example of using `FaTextResource`
  ```rust
  fn my_system(mut text_res: ResMut<FaTextResource>) {
      // some logic ..

      // get value
      let text = text_res.get_value_by_id("#boss-txt");

      // update value
      text_res.update_value_by_id("#boss-txt", "Good morning Boss");
  }
  ```
