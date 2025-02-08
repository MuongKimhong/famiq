# FaSelection

```
🟡 Needs container
🟡 Doesn't accepts child/children
```

### Variants
```rust
pub enum SelectorVariant {
    Outlined,
    Default,
    Underlined,
}
```

### Colors
```rust
pub enum SelectorColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}
```

### Sizes
```rust
pub enum SelectionSize {
    Small,
    Normal,
    Large,
}
```

### Shapes
```rust
pub enum SelectorShape {
    Default,
    Round,
    Rectangle
}
```

### Widget API
```rust
pub fn fa_selection<'a>(
    builder: &'a mut FamiqWidgetBuilder,
    placeholder: &str
) -> FaSelectionBuilder<'a> {
    // ..
}
```

### Usage
```rust
let selection = fa_selection(&mut builder, "Select choice")
    .choices(["Choice 1", "Choice 2"])
    .build();
```
Return `Entity` of the widget which must be used as child of `FaContainer` widget.

### Built-in classes
- Color: `is-primary`, `is-secondary`, `is-warning`, `is-info`, `is-success`, `is-danger`.

- Size: `is-small`, `is-normal`, `is-large`.

- Shapes: `is-round`, `is-rectangle`.

- Variant: `is-underlined`, `is-outlined`.

### Example
```rust
let plans = fa_selection(&mut builder, "Select plan")
    .id("#plan")
    .class("is-info")
    .choices(["Personal", "Team", "Enterprise"])
    .build();
);

let subscriptions = fa_selection(&mut builder, "Select subscription payment")
    .class("is-rectangle")
    .choices(["Weekly", "Monthly", "Annually"])
    .build();
);

fa_container(&mut builder).children([plans, subscriptions]).build();
```
![Example 1](../images/selection_example_1.png)

### Resource
```rust
pub struct FaSelectionResource;
```
- `FaSelectionResource` can be used to retrieve specific `fa_selection` value by either `id` or `entity`.
  #### Available methods:
  - `get_value_by_id`: get input value by id, return `empty string` it id doesn't exist.
  - `get_value_by_entity`: get input value by entity, return `empty string` it entity doesn't exist.

  #### Example of using `FaSelectionResource`
  ```rust
  fn my_system(input_res: Res<FaSelectionResource>) {
      // some logic ..

      // get value
      let text = input_res.get_value_by_id("#plan");
  }
  ```
