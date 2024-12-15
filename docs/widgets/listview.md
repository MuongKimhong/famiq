## FaListView
```
🟢 Doesn't need a container
🟢 Accepts child/children
```
An empty, stylyable and scrollable widget.

When you call `fa_list_view()`, it creates 2 entities, `a moving panel` (to be scrolled) and the `listview container` itself.

### Default listview container bundle
```rust
FaWidgetBundle {
    style: ...,
    ..default()
}
```
### Default listview container style
```rust
Style {
    width: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    height: Val::Percent(50.0),
    overflow: Overflow::clip(),
    ..default()
}
```
### Default listview move panel bundle
```rust
FaWidgetBundle {
    style: ...,
    ..default()
}
```
### Default listview move panel style
```rust
Style {
    width: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    ..default()
}
```
### Can be created with
```rust
// method
pub fn fa_list_view(&mut self, id: &str, items: &Vec<Entity>) -> Entity {
    // ..
}

builder.fa_list_view(..);
```
