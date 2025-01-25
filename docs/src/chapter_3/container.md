# FaContainer

```
🟢 Doesn't need container
🟢 Accepts child/children
```
An empty and stylyable widget. Think of it as a div inside HTML.

### Widget API
```rust
pub fn fa_container<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaContainerBuilder<'a> {
    // ..
}
```

### usage
```rust
let container = fa_container(&mut builder).build();
```
Return `Entity` of the widget which can be used as child for another widget.

### Example
Texts without container
```rust
let boss = fa_text(&mut builder, "Hello Boss").build();
let mom = fa_text(&mut builder, "Hello Mom").build();
```
![Example 1](../images/container_example_1.png)

Texts inside container
```rust
let boss = fa_text(&mut builder, "Hello Boss").build();
let mom = fa_text(&mut builder, "Hello Mom").build();

fa_container(&mut builder)
    .children(vec![boss, mom])
    .build();
```
![Example 2](../images/container_example_2.png)

### Styling
`id` and `classes` can be provided to container to be able to style it from json file.
```rust
fa_container(&mut builder)
    .id("#container")
    .children(vec![boss, mom])
    .build();
```
```json
{
  "#container": {
    "background_color": "yellow",
    "border_color": "yellow",
    "border_radius": "10px 10px 10px 10px"
  }
}
```
![Example 3](../images/container_example_3.png)
