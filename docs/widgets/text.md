## FaText
```
🟡 Need a container
🟡 Can't accept child/children
```
a simple TextBundle.

### Default bundle
```rust
TextBundle::from_section(value, text_style).with_background_color(Color::NONE);
```
### Default text style
```rust
TextStyle {
    font: .., // user's provided font
    ..default()
}
```
### Can be created with
```rust
// method
pub fn fa_text(&mut self, id: &str, value: &str) -> Entity {
    // ..
}

let text = builder.fa_text(..);
```
