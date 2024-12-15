## FaButton
```
🟡 Need a container
🟡 Can't accept child/children
```
Button widget with Press, Hover and Leave interaction events.

### Variants
```rust
pub enum BtnVariant {
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
pub enum BtnSize {
    Small,
    Normal,
    Large,
}
```
### Default bundle
```rust
FaWidgetBundle {
    style: ...,
    border_radius: BorderRadius::all(Val::Px(5.0)),
    ..default()
}
```
### Default style
```rust
Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
}
```
### Can be created with
```rust
// method
pub fn fa_button(&mut self, id: &str, text: &str, variant: Option<BtnVariant>, size: Option<BtnSize>) -> Entity {
    // ..
}

let button = builder.fa_button(..);
```
