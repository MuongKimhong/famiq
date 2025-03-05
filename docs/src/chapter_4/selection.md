# FaSelection

### Usage
```rust
let selection = fa_selection(&mut builder, "Select choice")
    .choices(["Choice 1", "Choice 2"])
    .build();
```
Return `Entity` which must be used inside a containable widget.


### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `choices([&str])`
- `color(&str)`: set custom background color.

### Example
```rust
let plans = fa_selection(&mut builder, "Select plan")
    .choices(["Personal", "Team", "Enterprise"])
    .build();
);

let subscriptions = fa_selection(&mut builder, "Select subscription payment")
    .choices(["Weekly", "Monthly", "Annually"])
    .build();
);

fa_container(&mut builder).children([plans, subscriptions]).build();
```

### Change event

Whenever `fa_selection`'s value changes, it emits an event called `FaSelectionChangeEvent` that contains
updated value.

```rust
pub struct FaSelectionChangeEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub new_value: String
}
```

Example,
```rust
fn my_system(mut events: EventReader<FaSelectionChangeEvent>) {
    for e in events.read() {
        // make sure this works only with selection that have id provided
        if let Some(id) = e.widget_id.as_ref() {
            match id.as_str() {
                "#selection-one" => {
                    println!("{:?}", e.new_value);
                }
                "#selection-two" => {
                    // do something with #selection-two new value
                }
            }
        }
    }
}
```
