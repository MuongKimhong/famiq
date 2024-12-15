## How styling in json file works?

Instead of writing rust code for styling provided by **Bevy UI**
```rust
NodeBundle {
    style: Style {
        border: UiRect::all(Val::Px(3.0)),
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0)
        },
        margin: UiRect::All(Val::Auto),
        width: Val::Percent(100.0),
        ..default()
    },
    border_color: BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
    border_radius: BorderRadius::all(Val::Px(5.0)),
    ..default()
}
```
you can simply give widget an id, then write styles in json file and any changes made
to json file will be reflected to the running app without needing to re-compile the app.
```json
{
  "#container": {
    "padding": "5px 5px 10px 10px",
    "border": "3px 3px 3px 3px",
    "border_color": "srgba 1.0, 1.0, 1.0, 0.3",
    "border_radius": "5px 5px 5px 5px",
    "width": "100%",
    "margin": "auto auto auto auto"
  }
}
```

## Supported & Unsupported styles
- Unsupported
```rust
grid_template_rows: Vec<RepeatedGridTrack>
grid_template_columns: Vec<RepeatedGridTrack>
grid_auto_rows: Vec<GridTrack>
grid_auto_columns: Vec<GridTrack>
grid_row: GridPlacement
grid_column: GridPlacement
```

- Supported
```rust
color: Color // can be used for fa_text & TextBundle only
font_size: f32 // can be used for fa_text & TextBundle only

background_color: BackgroundColor
border_color: BorderColor
border_radius: BorderRadius
visibility: Visibility
z_index: ZIndex

display: Display
position_type: PositionType
overflow: Overflow
direction: Direction
left: Val
right: Val
top: Val
bottom: Val
width: Val
height: Val
min_width: Val
min_height: Val
max_width: Val
max_height: Val
aspect_ratio: Option<f32>
align_items: AlignItems
justify_items: JustifyItems
align_self: AlignSelf
justify_self: JustifySelf
align_content: AlignContent
justify_content: JustifyContent
margin: UiRect
padding: UiRect
border: UiRect
flex_direction: FlexDirection
flex_wrap: FlexWrap
flex_grow: f32
flex_shrink: f32
flex_basis: Val
row_gap: Val
column_gap: Val
grid_auto_flow: GridAutoFlow
```

- Val struct support only
```rust
Val {
    Auto,
    Px,
    Percent,
    Vw,
    Vh
}
```

- Color struct support only
```rust
Color {
    Srgba(Srgba),
    LinearRgba(LinearRgba),
    Hsla(Hsla)
}
```

## How to write styles in json file?

it's same as bevy styling except that you write it in a string.
for example,
```rust
// bevy -> json
background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3))
"background_color": "srgba 1.0, 1.0, 1.0, 0.3"

padding: UiRect {
    left: Val::Px(10.0),
    right: Val::Px(10.0),
    top: Val::Percent(5.0),
    bottom: Val::Percent(5.0)
}
"padding": "10px 10px 5% 5%" // left right top bottom (order matters)

justify_items: JusitfyItems::Center
"justify_items": "center"

height: Val::Vw(100.0)
"height": "100vw"

flex_wrap: FlexWrap::NoWrap
"flex_wrap": "no_wrap"
```
