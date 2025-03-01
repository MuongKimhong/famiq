# FaStyleResource

Use this resource to update widget style directly within systems.

To use this resource on any widgets, `id` must be provided.

Currently, not all styles can be updated via this resource, but this will be improved in future.

### Available methods
Change widget background color.
```rust
set_background_color(id: &str, color: &str)
```

Change widget border color.
```rust
set_border_color(id: &str, color: &str)
```

Change widget border radius "left right top bottom".
```rust
set_border_radius(id: &str, radius: &str)
```

Change widget width.
```rust
set_width(id: &str, width: &str)
```

Change widget height.
```rust
set_height(id: &str, height: &str)
```

Change widget visibility.
```rust
set_visibility(id: &str, visibility: &str)
```

Change widget margin "left right top bottom".
```rust
set_margin(id: &str, margin: &str)
```

Change widget padding "left right top bottom".
```rust
set_padding(id: &str, padding: &str)
```

Change widget display.
```rust
set_display(id: &str, display: &str)
```

Change text's color.
```rust
set_color(id: &str, color: &str)
```

Change text's font size.
```rust
set_font_size(id: &str, size: &str)
```

Change widget shadow color.
```rust
set_shadow_color(id: &str, color: &str)
```
