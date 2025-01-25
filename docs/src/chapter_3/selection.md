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

### Resource
Resource to store key value pair of selector id & selected choice.
```rust
pub struct SelectedChoicesResource {
    pub choices: HashMap<String, String>
}

impl SelectedChoicesResource {
    pub fn update_or_insert(&mut self, id: String, selected_choice: String) {
        // ..
    }
}
```

### API
```rust
pub fn fa_selection<'a>(
    builder: &'a mut FamiqWidgetBuilder,
    placeholder: &str
) -> FaSelectionBuilder<'a> {
    // ..
}
```

### Usage via builder
```rust
let selection = fa_selection(&mut builder, "Select choice")
    .choices(vec!["Choice 1", "Choice 2"])
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
    .class("is-info")
    .choices(vec!["Personal", "Team", "Enterprise"])
    .build();
);

let subscriptions = fa_selection(&mut builder, "Select subscription payment")
    .class("is-rectangle")
    .choices(vec!["Weekly", "Monthly", "Annually"])
    .build();
);

fa_container(&mut builder)
    .children(vec![plans, subscriptions])
    .build();
```
![Example 1](../images/selection_example_1.png)

### Getting selected choice
The selected choice of a selection can be read from `SelectedItemsResource` within system.

```rust
fn my_system(selected_items: Res<SelectedItemsResource>) {
    if let Some(selected_choice) = selected_items.items.get("#my-selection-id") {
        println!("Choice: {:?}", selected_choice);
    }
}
```
