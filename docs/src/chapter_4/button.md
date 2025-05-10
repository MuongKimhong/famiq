# Button

### Usage
```rust
let button = button!(text: "Press me");
```
Return `Entity` of the widget which must be used inside a containable widget.

### Example
```rust
let default_btn = button!(text: "Default button", id: "#default-btn");
let info_btn = button!(text: "Info button", id: "#info-btn", class: "info");

container!(children: [default_btn, info_btn]);
```

### Handle button press
```rust
fn handle_button_press(mut events: EventReader<FaMouseEvent>) {
    for e in events.read() {
        if e.button_press().is_none() {
            return;
        }

        match e.button_press().unwrap().as_str() {
            "#default-btn" => todo!(),
            "#info-btn" => todo!(),
            _ => {}
        }
    }
}
```

### Required attribute
- **text**

### Available attributes
- **id**
- **class**
- **color**
