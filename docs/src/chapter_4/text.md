# Text

### Usage
```rust
let text = text!(text: "Some text");
```
Return `Entity` of the widget which must be used inside a containable widget.

### Example
```rust
let boss = text!(text: "Hello Boss");
let world = text!(text: "Hello World");

container!(children: [boss, world]);
```

### Required attribute
- **text**

### Available attributes
- **id**
- **class**
- **color**
