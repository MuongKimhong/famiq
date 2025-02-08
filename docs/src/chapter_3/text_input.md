# FaTextInput

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### Variants
```rust
pub enum TextInputVariant {
    Default,
    Outlined,
    Underlined,
}
```

### Colors
```rust
pub enum TextInputColor {
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
pub enum TextInputSize {
    Small,
    Normal,
    Large,
}
```

### Shapes
```rust
pub enum TextInputShape {
    Default,
    Round,
    Rectangle
}
```

### Widget API
```rust
pub fn fa_text_input<'a>(
    builder: &'a mut FamiqWidgetBuilder,
    placeholder: &str
) -> FaTextInputBuilder<'a> {
    // ..
}
```

### Usage
```rust
let input = fa_text_input(&mut builder, "Enter your name").build();
```
Return `Entity` of the widget which must be used as child of `FaContainer` widget.

### Built-in classes
- Color: `is-primary`, `is-secondary`, `is-warning`, `is-info`, `is-success`, `is-danger`.

- Size: `is-small`, `is-normal`, `is-large`.

- Shapes: `is-round`, `is-rectangle`.

- Variant: `is-underlined`, `is-outlined`.

### Example
```rust
// default
let input_default = fa_text_input(&mut builder, "Enter your name")
    .id("#name-input")
    .build();

// info & large
let input_info_large = fa_text_input(&mut builder, "Enter your name")
    .class("is-info is-large")
    .build();

// warning & round
let input_warning_round = fa_text_input(&mut builder, "Enter your name")
    .class("is-warning is-round")
    .build();

fa_container(&mut builder)
    .children([input_default, input_info_large, input_warning_round])
    .build();
```
![Example 1](../images/input_example_1.png)

### Resource
```rust
pub struct FaTextInputResource;
```
- `FaTextInputResource` can be used to retrieve specific `fa_text_input` value by either `id` or `entity`.
  #### Available methods:
  - `get_value_by_id`: get input value by id, return `empty string` it id doesn't exist.
  - `get_value_by_entity`: get input value by entity, return `empty string` it entity doesn't exist.

  #### Example of using `FaTextInputResource`
  ```rust
  fn my_system(input_res: Res<FaTextInputResource>) {
      // some logic ..

      // get value
      let text = input_res.get_value_by_id("#name-input");
  }
  ```
