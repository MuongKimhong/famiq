# FaCircular

```
🟡 Needs container
🟡 Doesn't accept child/children
```
Spinning circular.

### Variants
```rust
pub enum CircularColor {
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
pub enum CircularSize {
    Small,
    Normal,
    Large
}
```

### API
```rust
pub fn fa_circular(&mut self, id: &str, classes: &str) -> Entity {
    // ..
}
```

### Usage via builder
```rust
let circular = builder.fa_circular(..);
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Built-in classes
- Color: `is-primary`, `is-secondary`, `is-warning`, `is-info`, `is-success`, `is-danger`.

- Size: `is-small`, `is-normal`, `is-large`.

### Example
```rust
// default
let cir = builder.fa_circular("#cir", "");

// warning & small
let warning_cir = builder.fa_circular("#warning-cir", "is-warning is-small");

// primary & large
let primary_cir = builder.fa_circular("#primary-cir", "is-primary is-large");

builder.fa_container("#container", "", &vec![
    cir,
    warning_cir,
    primary_cir
]);
```
![Example 1](../images/circular_example_1.png)
