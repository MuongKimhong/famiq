# Circular

A spinning circular.

### Usage
```rust
let circular = circular!();
```
Return `Entity` which must be used inside a containable widget.

### Example
```rust
// default
let cir = circular!();

// warning & small
let warning_cir = circular!(class: "warning small");

// primary & custom size
let primary_cir = circular!(class: "primary", size: 50.0);

// custom color
let custom_color_cir = circular!(color: "cyan_500");

container!(
    children: [
        cir,
        warning_cir,
        primary_cir,
        custom_color_cir
    ]
);
```

### Available attributes
- **id**
- **class**
- **color**
- **tooltip**
- **size**
