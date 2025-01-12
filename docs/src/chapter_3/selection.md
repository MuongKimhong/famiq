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
pub fn fa_selection(
    &mut self,
    id: &str,
    classes: &str,
    placeholder: &str,
    choices: &Vec<String>
) -> Entity {
    // ..
}
```

### Usage via builder
```rust
let selection = builder.fa_selection(..);
```
Return `Entity` of the widget which must be used as child of `FaContainer` widget.

### Built-in classes
- Color: `is-primary`, `is-secondary`, `is-warning`, `is-info`, `is-success`, `is-danger`.

- Size: `is-small`, `is-normal`, `is-large`.

- Shapes: `is-round`, `is-rectangle`.

- Variant: `is-underlined`, `is-outlined`.

### Example
```rust
let plans = builder.fa_selection(
    "#plans",
    "is-info",
    "Select plan",
    &vec!["Personal".to_string(), "Team".to_string(), "Enterprise".to_string()],
);

let subscriptions = builder.fa_selection(
    "#subscriptions",
    "is-rectangle",
    "Select subscription payment",
    &vec!["Weekly".to_string(), "Monthly".to_string(), "Annually".to_string()],
);

builder.fa_container("#container", "", &vec![plans, subscriptions]);
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
