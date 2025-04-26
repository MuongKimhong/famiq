# Modal

### Usage
```rust
modal!();
```

### Example
```rust
modal!(
    model: "show_modal",
    children: [
        container!(children: [
            text!(text: "Hello from modal"),
            button!(text: "Close")
        ])
    ]
);
```

### Required attribute
- **model**

### Available attributes
- **id**
- **class**
- **clear_bg**: if `true`, the background is fully transparent
