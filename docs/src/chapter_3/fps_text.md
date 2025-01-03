# FaFpsText

```
🟢 Doesn't need container
🟡 Doesn't accept child/children
```

### API
```rust
pub fn fa_fps(&mut self, id: &str, change_color: bool) -> Entity {
    // ..
}
```
- `change_color` if `true` it changes color based on number.

### Usage via builder
```rust
builder.fa_fps(..);
```

### Example
```rust
builder.fa_fps("#fps", true);
```
![Example 1](../images/fps_example_1.png)
