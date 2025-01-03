# FaButton

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### Variants
```rust
pub enum BtnVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
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
pub fn fa_button(&mut self, id: &str, text: &str, variant: &str, size: &str) -> Entity {
    // ..
}
```

### Usage via builder
```rust
let button = builder.fa_button(..);
```
Return `Entity` of the widget which must be used inside `FaContainer` widget.

### Example
```rust
// default
let my_btn = builder.fa_button("#my-btn", "Press me", "", "");

// info
let info_btn = builder.fa_button("#info-btn", "Press me", "info", "");

// small
let small_btn = builder.fa_button("#small-btn", "Press me", "", "small");

// warning & large
let warning_btn = builder.fa_button("#warning-btn", "Press me", "warning", "large");

builder.fa_container("#container", &vec![
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
