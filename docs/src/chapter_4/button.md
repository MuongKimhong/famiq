# FaButton

### Usage
```rust
let button = fa_button(&mut builder, "Press me").build();
```
Return `Entity` of the widget which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `color(&str)`: set custom background color.

### Example
```rust
let default_btn = fa_button(&mut builder, "Default button")
    .id("#default-btn")
    .build();

let info_btn = fa_button(&mut builder, "Info button")
    .id("#info-btn")
    .class("is-info")
    .build();

fa_container(&mut builder)
    .children([default_btn, info_btn])
    .build();
```

### Handle button press

You can write a system that run in `Update` schedule to handle button events (hovered, pressed, none).

```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {

        // it's not a button press event, return early.
        if !e.is_pressed(WidgetType::Button) {
            return;
        }

        // make sure this works only with buttons that have id provided
        if let Some(id) = e.widget_id.as_ref() {
            match id.as_str() {
                "#default-btn" => {
                    // do something when default button is pressed
                },
                "#info-btn" => {
                    // do something when info button is pressed
                }
                _ => ()
            }
        }
    }
}
```
