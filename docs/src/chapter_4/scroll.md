# Scroll

Vertical scrollable container.

**Note**

`scroll!` has height set to 50% of the `window` or its `parent` container height.

### Usage
```rust
let button = button!(text: "Press me");
let input = text_input!(placeholder: "Enter your name", model: "name");

scroll(children: [input, button]);
```
return `Entity`.

### Available attributes
- **id**
- **class**
- **color**
- **scroll_height**: default to `15`
