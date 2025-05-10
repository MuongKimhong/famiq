# Container

An empty and stylyable widget. Think of it as a **div** inside HTML.

**Note**

- `container!` has its default height set to `auto`, meaning its height
depends on its children.

### usage
```rust
container!(id: "#my-container");

// or with children
container!(
    id: "#my-container",
    children: [
        text!(text: "Hello")
    ]
);
```
Return `Entity` of the widget which can optionally be used as child for another containable widget.

### Available attributes
- **id**
- **class**
- **color**
- **children**
