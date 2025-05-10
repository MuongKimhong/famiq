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

// or with reactive data
fa_query.insert_bool("right", true);
fa_query.insert_bool("can_change_color", false);

fps!(right_side: "$[right]", change_color: "$[can_change_color]");
```

### Available attributes
- **id**
- **class**
- **color**
- **right_side**: show the fps at top-right corner.
- **change_color**: change color based on its value.
