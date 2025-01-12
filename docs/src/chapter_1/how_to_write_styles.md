
# How to write bevy styles in JSON file.

**Famiq** supports almost all UI styles provided by Bevy engine.

### id & Class
**Famiq** supports styles via `id` and `classes`.
```json
{
  "#my-widget-id": {
    ..
  },
  ".some-class": {
    ..
  }
}
```
- `class_name` must starts with dot `.`.

### For text widgets
- `color`: text color. Supports only `srgba`, `linear_rgba` and `hsla`.

  Example, `"color": "srgba 0.961, 0, 0.784, 0.961"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.TextColor.html](https://docs.rs/bevy/latest/bevy/prelude/struct.TextColor.html)

- `font_size`: text font size.

  Example, `"font_size": "14px"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html#structfield.font_size](https://docs.rs/bevy/latest/bevy/prelude/struct.TextFont.html#structfield.font_size)

Can be used for `fa_text` and `fa_fps` widgets.

### For node widgets
- `background_color`: supports only `srgba`, `linear_rgba` and `hsla`.

  Example, `"background_color": "srgba 0.961, 0, 0.784, 0.961"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html)

- `border_color`: supports only `srgba`, `linear_rgba` and `hsla`.

  Example, `"border_color": "linear_rgba 0.961, 0, 0.784, 0.961"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html](https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html)

- `border_radius`: top_left, top_right, bottom_left, bottom_right.

  Example, `"border_radius": "10px 10px 10px 10px"`

  [https://docs.rs/bevy/latest/bevy/prelude/struct.BorderRadius.html](https://docs.rs/bevy/latest/bevy/prelude/struct.BorderRadius.html)

- `border_radius_top_left`: this will override top_left value defined in `border_radius`.

- `border_radius_top_right`: this will override top_right value defined in `border_radius`.

- `border_radius_bottom_left`: this will override bottom_left value defined in `border_radius`.

- `border_radius_bottom_right`: this will override bottom_right value defined in `border_radius`.

- `visibility`: supports only `visible`, `hidden` and `inherited`.

  [https://docs.rs/bevy/latest/bevy/prelude/enum.Visibility.html](https://docs.rs/bevy/latest/bevy/prelude/enum.Visibility.html)

- `z_index`: indicates that this Node entity’s front-to-back ordering is not controlled solely by its location in the UI hierarchy. A node with a higher z-index will appear on top of sibling nodes with a lower z-index.

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

- `left`: the horizontal position of the left edge of the node.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.left](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.left)

- `right`: the horizontal position of the right edge of the node.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.right](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.right)

- `top`: the vertical position of the top edge of the node.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.top](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.top)

- `bottom`: the vertical position of the bottom edge of the node.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.bottom](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.bottom)

- `width`: the ideal width of the node. width is used when it is within the bounds defined by `min_width` and `max_width`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.width](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.width)

- `height`: the ideal height of the node. height is used when it is within the bounds defined by `min_height` and `max_height`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.height](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.height)

- `min_width`: the minimum `width` of the node. `min_width` is used if it is greater than `width` and/or `max_width`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_width](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_width)

- `min_height`: the minimum `height` of the node. `min_height` is used if it is greater than `height` and/or `max_height`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_height](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.min_height)

- `max_width`: the maximum `width` of the node. `max_width` is used if it is within the bounds defined by `min_width` and `width`.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.max_width](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.max_width)

- `max_height`: the maximum `height` of the node. `max_height` is used if it is within the bounds defined by `min_height` and `height`.

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

- `row_gap`: the size of the gutters between items in a vertical flexbox layout or between rows in a grid layout.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.row_gap](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.row_gap)

- `column_gap`: the size of the gutters between items in a horizontal flexbox layout or between column in a grid layout.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.column_gap](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.column_gap)

- `grid_auto_flow`: controls whether automatically placed grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used. Only affects Grid layouts.

  [https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.grid_auto_flow](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html#structfield.grid_auto_flow)
