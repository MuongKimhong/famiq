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
pub fn fa_circular<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaCircularBuilder<'a> {
    // ..
}
```

### Usage
```rust
let circular = fa_circular(&mut builder).build();
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.
- `size()`: set custom size of `fa_circular`.

### Built-in classes
- Color: `is-primary`, `is-secondary`, `is-warning`, `is-info`, `is-success`, `is-danger`.

- Size: `is-small`, `is-normal`, `is-large`.

### Example
```rust
// default
let cir = fa_circular(&mut builder).build();

// warning & small
let warning_cir = fa_circular(&mut builder)
    .class("is-warning is-small")
    .build();

// primary & custom size
let primary_cir = fa_circular(&mut builder)
    .class("is-primary is-large")
    .build();

fa_container(&mut builder).children([cir, warning_cir, primary_cir]).build();
```
![Example 1](../images/circular_example_1.png)
