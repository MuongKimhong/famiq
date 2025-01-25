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

### Usage via builder
```rust
let text = fa_text(&mut builder, "Some text").build();
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Example
```rust
let boss = fa_text(&mut builder, "Hello Boss").build();
let mom = fa_text(&mut builder, "Hello Mom").build();

fa_container(&mut builder)
    .children(vec![boss, mom])
    .build();
```
![Example 2](../images/container_example_2.png)
