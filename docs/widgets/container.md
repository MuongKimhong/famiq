## FaContainer
```
🟢 Doesn't need a container
🟢 Accepts child/children
```
An empty and stylyable widget. Think of it as a div inside HTML.

### Default bundle
```rust
FaWidgetBundle {
    style: ...,
    ..default()
}
```
### Default style
```rust
Style {
    width: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    height: Val::Auto,
    border: UiRect::all(Val::Px(10.)),
    ..default()
}
```
### Can be created with
```rust
// method
pub fn fa_container(&mut self, id: &str, children: &Vec<Entity>) -> Entity {
    // ..
}

builder.fa_container(..);
```
