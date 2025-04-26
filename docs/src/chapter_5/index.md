# Reactivity

**Famiq**'s reactivity is really simple, thus it's limited.

```
Data <-> Subscribers
```
When a `data` is changed, all the `subscribers` re-build themselve.

#### Unsupported features
- template iteration
- template condition

### Example
```rust
fn setup_ui(mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    fa_query.insert_num("count", 0);

    container!(
        children: [
            text!(text: "Count: $[count]"),
            button!(text: "Increase", id: "#btn")
        ]
    );
}

fn on_button_press(
    mut events: EventReader<FaMouseEvent>,
    mut fa_query: FaQuery
) {
    for e in events.read() {
        if !e.is_button_pressed("#btn") {
            continue;
        }
        if let Some(count) = fa_query.get_data_mut("count") {
            let count = count.as_num_mut();
            *count += 1;
        }
    }
}
```

### Inserting data
We start by inserting reactive data via `FaQuery` and the only supported data types are: `i32`, `f32`, `bool`, `Vec<String>`, and `String`.

- `insert_num`: for inserting data as **i32**.
- `insert_fnum`: for inserting data as **f32**.
- `insert_str`: for inserting data as **String**.
- `insert_bool`: for inserting data as **boolean**.
- `insert_list`: for inserting data as **Vector of String**.

```rust
fa_query.insert_num("count", 0);
```
`0` will be converted to `RVal::Num(0)`.

### Subscribing to data
We now can use the data in other widgets via reactive template `$[]`.
```rust
fa_query.insert_num("count", 0);
fa_query.insert_str("name", "");
fa_query.insert_str("text_color", "blue");
fa_query.insert_bool("can_change_color", true);

fps!(change_color: "$[can_change_color]");

container!(children: [
    text!(text: "Counter: $[count]", color: "$[text_color]"),
    text!(text: "Name: $[name]"),
    text_input!(placeholder: "Enter name", model: "name")
]);
```

### Getting & Mutating the data
**getting data**
```rust
if let Some(count) = fa_query.get_data("count") {
    println!("count is {:?}", count.as_num());
}
// or
let count = fa_query.get_data("count").unwrap();
println!("I'm sure count is available and it is {:?}", count.as_num());
```

**mutating data**

There are 2 ways to mutate a data.

1. using `get_data_mut`
```rust
// in system

if let Some(count) = fa_query.get_data_mut("count") {
    let count = count.as_num_mut();
    *count += 1;
}

if let Some(state) = fa_query.get_data_mut("can_change_color") {
    let state = state.as_bool_mut();
    *state = false;
}
```

2. explicitly calling `mutate` methods
```rust
// in system

fa_query.mutate_num("count", 2);
fa_query.mutate_bool("can_change_color", false);
```
