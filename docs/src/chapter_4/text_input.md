# FaTextInput

Widget that allow user to type in texts.

Currently, this widget is not interactive.

### Usage
```rust
let input = fa_text_input(&mut builder, "Enter your name").build();
```
Return `Entity` which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `is_password()`: mask all the character as `*`.
- `color(&str)`: set custom background color.

### Example
```rust
// default
let input_default = fa_text_input(&mut builder, "Enter your name").build();

// password
let input_password = fa_text_input(&mut builder, "Enter your password")
    .is_password()
    .build();

fa_container(&mut builder)
    .children([input_default, input_password])
    .build();
```

### Change event

Whenever `fa_text_input`'s value changes, it emits an event called `FaTextInputChangeEvent` that contains
updated value.

```rust
pub struct FaTextInputChangeEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub new_value: String
}
```

Example,
```rust
fn my_system(mut events: EventReader<FaTextInputChangeEvent>) {
    for e in events.read() {
        // make sure this works only with text input that have id provided
        if let Some(id) = e.widget_id.as_ref() {
            match id.as_str() {
                "#text-input-one" => {
                    println!("{:?}", e.new_value);
                }
                "#text-input-two" => {
                    // do something with #text-input-two new value
                }
            }
        }
    }
}
```
