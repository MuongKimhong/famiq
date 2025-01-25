# FaListView

```
🟢 Doesn't need container
🟢 Accepts child/children
```

### API
```rust
pub fn fa_listview<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaListViewBuilder<'a> {
    // ..
}
```
return `Entity` which can be used as child of `FaContainer`.

### Usage
```rust
let button = fa_button(&mut builder, "Press me").build();
let input = fa_text_input(&mut builder, "Enter your name").build();

fa_listview(&mut builder)
    .children(vec![input, button])
    .build();
```
