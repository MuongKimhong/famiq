# Dialog

### Usage
```rust
dialog!();
```

### Example
```rust
fa_query.insert_bool("show_dialog", false);

dialog!(
    model: "show_dialog",
    children: [
        container!(children: [
            text!(text: "Hello from dialog"),
            button!(text: "Close")
        ])
    ]
);
```

### Required attribute
- **model**: type bool.

### Available attributes
- **id**
- **class**
- **clear_bg**: if `true`, the background is fully transparent
