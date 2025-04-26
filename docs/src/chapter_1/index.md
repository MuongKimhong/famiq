# How can I style my widgets?
**Famiq** uses a **JSON-based styling system**, similar to how **HTML** uses **CSS**.

Each widget can have an **id** or **classes**, which are used to apply styles from the JSON file.

```rust
// by id
let button = button!(text: "Press me", id: "#button");

// by class or classes
let text_1 = text!(text: "Hello world", class: "text important");
let text_2 = text!(text: "Hello Mom", class: "text");
```
```json
{
  "#button": {
    "background_color": "blue"
  },
  ".text": {
    "font_size": "40"
  },
  ".important": {
    "color": "red"
  }
}
```
**Notes**
- **IDs (id)** must start with **`#`** and must match between the widget and the JSON file.
- **Class names (class)** must start with **`.`** in the JSON file.
