# FaFpsText

```
🟢 Doesn't need container
🟡 Doesn't accept child/children
```

### Widget API
```rust
pub fn fa_fps<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaFpsTextBuilder<'a> {
    // ..
}
```

### Usage via builder
```rust
fa_fps(&mut builder).build();
```
return `Entity` which can be used as a child of a `FaContainer`.
-  `change_color()`: change number color based on the value.
- `right_side()`: make fps text appears at the top right corner.

### Example
```rust
fa_fps(&mut builder).change_color().right_side().build();
```
