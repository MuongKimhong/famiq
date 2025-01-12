# FaText

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### API
```rust
pub fn fa_text(&mut self, id: &str, classes: &str, value: &str) -> Entity {
    // ..
}
```

### Usage via builder
```rust
let text = builder.fa_text(..);
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Example
```rust
let boss = builder.fa_text("#boss", "", "Hello Boss");
let mom = builder.fa_text("#mom", "", "Hello Mom");

builder.fa_container("#container", "", &vec![boss, mom]);
```
![Example 1](../images/container_example_2.png)
