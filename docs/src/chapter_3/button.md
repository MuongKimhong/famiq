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

### API
```rust
pub fn fa_button(&mut self, id: &str, classes: &str, text: &str) -> Entity {
    // ..
}
```

### Usage via builder
```rust
let button = builder.fa_button(..);
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
let my_btn = builder.fa_button("#my-btn", "", "Press me");

// info
let info_btn = builder.fa_button("#info-btn", "is-info", "Press me");

// small
let small_btn = builder.fa_button("#small-btn", "is-success, is-small", "Press me");

// warning & large
let warning_btn = builder.fa_button("#warning-btn", "is-warning is-large", "Press me");

builder.fa_container("#container", "", &vec![
    my_btn,
    info_btn,
    small_btn,
    warning_btn
]);
```
![Example 1](../images/btn_example_1.png)

### Handle button press
```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {
        if e.widget == WidgetType::Button && e.interaction == Interaction::Pressed {

            // handle specific button using its id
            match e.widget_id.as_str() {
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
```
