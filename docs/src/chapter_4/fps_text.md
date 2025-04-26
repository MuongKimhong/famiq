# FpsText

Show **FPS** value at top-left corner of the window.

This widget doesn't need to be inside a containable widget.

### Usage
```rust
fps!();
```
return `Entity`.

### Example
```rust
fps!(right_side: true, change_color: false);
```

### Available attributes
- **id**
- **class**
- **color**
- **right_side**: show the fps at top-right corner
- **change_color**: change_color based on its value
