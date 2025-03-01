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
