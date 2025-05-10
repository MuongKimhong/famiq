# How to write bevy styles in JSON file?

### Example
```rust
button!(text: "Press me", id: "#btn");
```
```json
{
  "#btn": {
    "background_color": "blue"
  }
}
```

### Styles
**Famiq** supports almost all UI styles provided by Bevy engine.

- `color`: text color, supports only `srgba`, `linear_rgba`, `hsla` and basic colors.

  Examples,
  - `"color": "srgba 0.961, 0.0, 0.784, 0.9"`
  - `"color": "yellow"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.TextColor.html](https://docs.rs/bevy/latest/bevy/prelude/struct.TextColor.html)

- `font_size`: text font size.

  Example, `"font_size": "14"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html#structfield.font_size](https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html#structfield.font_size)

- `background_color`: supports only `srgba`, `linear_rgba`, `hsla` and basic colors.

  Examples,
  - `"background_color": "srgba 0.961, 0.0, 0.784, 0.95"`
  - `"background_color": "green"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html)

- `border_color`: supports only `srgba`, `linear_rgba`, `hsla` and basic colors.

  Examples,
  - `"border_color": "linear_rgba 0.961, 0.0, 0.784, 0.9"`
  - `"border_color": "pink"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html](https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html)

- `border_radius`: top_left, top_right, bottom_left, bottom_right.

  Example, `"border_radius": "10px 10px 10px 10px"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.BorderRadius.html](https://docs.rs/bevy/latest/bevy/prelude/struct.BorderRadius.html)

- `visibility`: supports only `visible`, `hidden` and `inherited`.

  [https://docs.rs/bevy/latest/bevy/prelude/enum.Visibility.html](https://docs.rs/bevy/latest/bevy/prelude/enum.Visibility.html)

- `z_index`: indicates that a widget’s front-to-back ordering is not controlled solely by its location in the UI hierarchy. A widget with a higher z-index will appear on top of sibling widgets with a lower z-index.

  Example, `"z_index": "2"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.ZIndex.html](https://docs.rs/bevy/latest/bevy/prelude/struct.ZIndex.html)

- `display`: defines the layout model used by node. Supports `flex`, `grid`, `block` and `none`.

  [https://docs.rs/bevy/latest/bevy/prelude/enum.Display.html](https://docs.rs/bevy/latest/bevy/prelude/enum.Display.html)

- `position_type`: the strategy used to position node. Supports `relative` and `absolute`.

  [https://docs.rs/bevy/latest/bevy/prelude/enum.PositionType.html](https://docs.rs/bevy/latest/bevy/prelude/enum.PositionType.html)

- `overflow_x`: whether to show or clip overflowing items on the x axis. Supports `visible`, `clip`, `hidden` and `scroll`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Overflow.html](https://docs.rs/bevy/latest/bevy/prelude/struct.Overflow.html)

- `overflow_y`: whether to show or clip overflowing items on the y axis. Supports `visible`, `clip`, `hidden` and `scroll`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Overflow.html](https://docs.rs/bevy/latest/bevy/prelude/struct.Overflow.html)

- `left`: the horizontal position of the left edge of the widget.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.left](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.left)

- `right`: the horizontal position of the right edge of the widget.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.right](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.right)

- `top`: the vertical position of the top edge of the widget.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.top](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.top)

- `bottom`: the vertical position of the bottom edge of the widget.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.bottom](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.bottom)

- `width`: the ideal width of the widget. width is used when it is within the bounds defined by `min_width` and `max_width`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.width](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.width)

- `height`: the ideal height of the widget. height is used when it is within the bounds defined by `min_height` and `max_height`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.height](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.height)

- `min_width`: the minimum `width` of the widget. `min_width` is used if it is greater than `width` and/or `max_width`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_width](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_width)

- `min_height`: the minimum `height` of the widget. `min_height` is used if it is greater than `height` and/or `max_height`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_height](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_height)

- `max_width`: the maximum `width` of the widget. `max_width` is used if it is within the bounds defined by `min_width` and `width`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.max_width](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.max_width)

- `max_height`: the maximum `height` of the widget. `max_height` is used if it is within the bounds defined by `min_height` and `height`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.max_height](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.max_height)

- `align_items`: used to control how each individual item is aligned by default within the space they’re given. Supports `default`, `start`,
  `end`, `flex_start`, `flex_end`, `center`, `base_line` and `stretch`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.align_items](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.align_items)

- `justify_items`: used to control how each individual item is aligned by default within the space they’re given. Supports `default`, `start`,
  `end`, `flex_start`, `flex_end`, `center`, `base_line` and `stretch`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.justify_items](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.justify_items)

- `align_left`: used to control how the specified item is aligned within the space it’s given. Supports `auto`, `start`, `end`, `flex_start`,
  `flex_end`, `center`, `base_line` and `stretch`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.align_self](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.align_self)

- `justify_content`: used to control how items are distributed. Supports `default`, `start`, `end`, `flex_start`, `flex_end`, `center`,    `stretch`, `space_between`, `space_evenly` and `space_around`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.justify_content](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.justify_content)

- `margin`: left, right, top, bottom.

  Example, `"margin": "10px 10px 5px 5px"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.margin](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.margin)

- `margin_left`: this will override left value defined in `margin`.

- `margin_right`: this will override right value defined in `margin`.

- `margin_top`: this will override top value defined in `margin`.

- `margin_bottom`: this will override bottom value defined in `margin`.

- `padding`: left, right, top, bottom.

  Example, `"padding": "10px 10px 5px 5px"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.padding](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.padding)

- `padding_left`: this will override left value defined in `padding`.

- `padding_right`: this will override right value defined in `padding`.

- `padding_top`: this will override top value defined in `padding`.

- `padding_bottom`: this will override bottom value defined in `padding`.

- `border`: left, right, top, bottom.

  Example, `"padding": "10px 10px 5px 5px"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.border](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.border)

- `border_left`: this will override left value defined in `border`.

- `border_right`: this will override right value defined in `border`.

- `border_top`: this will override top value defined in `border`.

- `border_bottom`: this will override bottom value defined in `border`.

- `flex_direction`: whether a Flexbox container should be a row or a column. Supports `row`, `column`, `row_reverse` and `column_reverse`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_direction](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_direction)

- `flex_wrap`: whether a Flexbox container should wrap its contents onto multiple lines if they overflow. Supports `no_wrap`, `wrap` and `wrap_reverse`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_wrap](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_wrap)

- `flex_grow`: defines how much a flexbox item should grow if there’s space available. Defaults to "0" (don’t grow at all).

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_grow](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_grow)

- `flex_shrink`: defines how much a flexbox item should shrink if there’s not enough space available. Defaults to 1.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_shrink](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_shrink)

- `flex_basis`: the initial length of a flexbox in the main axis, before flex growing/shrinking properties are applied.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_basis](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.flex_basis)
