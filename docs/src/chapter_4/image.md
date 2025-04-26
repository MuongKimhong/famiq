# Image

Image widget.

supports only `jpg` and `png` format.

### Usage
```rust
let image = image!(path: "path/to/image.jpg");
```
return `Entity` which must be used inside a containable widget.

### Example
```rust
container!(children: [
    image!(path: "logo.png", width: "100px", height: "100px")
]);
```

### Required attribute
- **path**: path to image relative to `assets` directory.

### Available attributes
- **id**
- **class**
- **color**
- **tooltip**
- **width**
- **height**
