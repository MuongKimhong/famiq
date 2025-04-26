# TextInput

Widget that allow user to type in texts.

#### Note
- Support single line only.
- On web, **clipboard** is not supported yet.
- **model** attribute is required.

### Usage
```rust
let input = text_input!(placeholder: "Enter your name", model: "name");
```
Return `Entity` which must be used inside a containable widget.

### Example
```rust
container!(
    children: [
        text!(text: "$[name]"),
        text_input!(placeholder: "Enter your name", model: "name")
    ]
);
```

### Required attribute
- **placeholder**
- **model**

### Available attributes
- **id**
- **class**
- **color**
- **tooltip**
