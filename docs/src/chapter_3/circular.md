# FaCircular

```
🟡 Needs container
🟡 Doesn't accept child/children
```
Spinning circular.

### Variants
```rust
pub enum CircularVariant {
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
pub fn fa_circular(&mut self, id: &str, variant: &str, size: &str) -> Entity {
    // ..
}
```

### Usage via builder
```rust
let circular = builder.fa_circular(..);
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Example
```rust
// default
let cir = builder.fa_circular("#cir", "", "");

// warning & small
let warning_cir = builder.fa_circular("#warning-cir", "warning", "small");

// primary & large
let primary_cir = builder.fa_circular("#primary-cir", "primary", "large");

builder.fa_container("#container", &vec![
    cir,
    warning_cir,
    primary_cir
]);
```
![Example 1](../images/circular_example_1.png)
