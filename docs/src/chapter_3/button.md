# FaButton

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### Colors
```rust
pub enum BtnColor {
    Default,
    Primary,
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark
}
```

### Shapes
```rust
pub enum BtnShape {
    Default,
    Round,
    Rectangle
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

### Widget API
```rust
pub fn fa_button<'a>(builder: &'a mut FamiqWidgetBuilder, text: &str) -> FaButtonBuilder<'a> {
    // ..
}
```

### Usage
```rust
let button = fa_button(&mut builder, "Press me").build();
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Built-in classes
- Color: `is-primary`, `is-primary-dark`, `is-secondary`, `is-danger`, `is-danger-dark`, `is-info`, `is-info-dark`,
         `is-success`, `is-success-dark`, `is-warning`, `is-warning-dark`.

- Size: `is-small`, `is-normal`, `is-large`.

- Shape: `is-round`, `is-rectangle`.

### Example
```rust
// default
let my_btn = fa_button(&mut builder, "Press me")
    .id("#my-btn")
    .build();

// info
let info_btn = fa_button(&mut builder, "Press me")
    .id("#info-btn")
    .class("is-info")
    .build();

// success & small
let small_btn = fa_button(&mut builder, "Press me")
    .class("is-success is-small")
    .build();

// warning & large
let large_btn = fa_button(&mut builder, "Press me")
    .class("is-warning is-large")
    .build();

fa_container(&mut builder)
    .children(vec![my_btn, info_btn, small_btn, large_btn])
    .build();
```
![Example 1](../images/btn_example_1.png)

### Handle button press
```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {
        if e.widget == WidgetType::Button && e.interaction == Interaction::Pressed {
            // make sure this works only with widgets that have id provided
            if e.widget_id.is_some() {
                // handle specific button using its id
                match e.widget_id.as_ref().as_str() {
                    "#my-btn" => {
                        // do something with my button
                    },
                    "#info-btn" => {
                        // do something with info button
                    }
                    _ => ()
                }
            }
        }
    }
}
```
