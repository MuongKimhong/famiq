# How can I style my widgets?
**Famiq** uses a **JSON-based styling system**, similar to how **HTML** uses **CSS**.

Each widget can have an **id** or **classes**, which are used to apply styles from the JSON file.

```rust
// by id
let button = fa_button(&mut builder, "Press me").id("#button").build();

// by class or classes
let text_1 = fa_text(&mut builder, "Hello world").class("text important").build();
let text_2 = fa_text(&mut builder, "Hello mom").class("text").build();
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

Currently, **id** or **class** assigned to a widget is **immutable**.
