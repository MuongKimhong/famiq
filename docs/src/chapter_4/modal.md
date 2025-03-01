# FaModal

### Usage
```rust
fa_modal(&mut builder).build();
```

### Example
```rust
let text = fa_text(&mut builder, "Hello from modal").build();
let close_btn = fa_button(&mut builder, "Close").build();

let container = fa_container(&mut builder)
    .children([text, close_btn])
    .build();

fa_modal(&mut builder).children([container]).build();
```
