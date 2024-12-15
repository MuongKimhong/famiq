## FaSelection
```
🟢 Doesn't need a container
🟡 Can't accept child/children
```
Widget that allow user to select one choice.

When you call `fa_selection()`, it creates multiple entities including a `container`, `selector`, `choice panel`, and `item(s)`.

### Variants
```rust
pub enum SelectorVariant {
    Outlined,
    Default,
    Underlined,
}
```
### Sizes
```rust
pub enum SelectionSize {
    Small,
    Normal,
    Large,
}
```
### Default container bundle
```rust
FaWidgetBundle {
    style: ...,
    ..default()
}
```
### Default container style
```rust
Style {
    width: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    height: Val::Auto,
    ..default()
}
```
### Default selector bundle
```rust
FaWidgetBundle {
    style: ...,
    border_radius: ..., // depends on variant
    border_color: BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922)),
    ..default()
};
```
### Default selector style
```rust
Style {
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceBetween,
    border: ..., // depends on sizes
    padding: UiRect {
        left: Val::Px(10.0),
        right: Val::Px(10.0),
        top: Val::Px(2.0),
        bottom: Val::Px(2.0),
    },
    margin: UiRect {
        top: Val::Px(5.0),
        right: Val::Px(0.0),
        left: Val::Px(0.0),
        bottom: Val::Px(0.0),
    },
    ..default()
}
```
### Default choice panel bundle
```rust
pub const PANEL_BG_COLOR: Color = Color::srgba(0.29, 0.29, 0.282, 1.0);

FaWidgetBundle {
    style: ...,
    border_radius: BorderRadius::all(Val::Px(5.0)),
    z_index: ZIndex::Global(10),
    visibility: Visibility::Hidden,
    background_color: BackgroundColor(PANEL_BG_COLOR),
    ..default()
}
```
### Default choice panel style
```rust
Style {
    width: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Start,
    height: Val::Auto,
    padding: UiRect {
        top: Val::Px(5.0),
        bottom: Val::Px(5.0),
        left: Val::Px(0.0),
        right: Val::Px(0.0),
    },
    margin: UiRect::all(Val::Px(2.0)),
    position_type: PositionType::Absolute,
    ..default()
}
```
### Default item(s) bundle
```rust
pub const ITEM_ON_HOVER_BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
pub const ITEM_NORMAL_BG_COLOR: Color = Color::NONE; // transparent

FaWidgetBundle {
    style: ...,
    background_color: ITEM_NORMAL_BG_COLOR.into(),
    ..default()
}
```
### Defautl item(s) style
```rust
Style {
    width: Val::Percent(100.0),
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    padding: UiRect {
        top: Val::Px(10.0),
        bottom: Val::Px(10.0),
        right: Val::Px(0.0),
        left: Val::Px(10.0),
    },
    ..default()
}
```
### Can be created with
```rust
// method
pub fn fa_selection(
    &mut self,
    id: &str,
    placeholder: &str,
    items: &Vec<String>,
    label: Option<&str>,
    size: Option<SelectionSize>,
    variant: Option<SelectorVariant>,
) -> Entity {
    // ..
}

builder.fa_selection(..);
```
