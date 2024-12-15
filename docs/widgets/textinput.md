## FaTextInput
```
🟡 Need a container
🟡 Can't accept child/children
```

simple text input.

### Variants
```rust
pub enum TextInputVariant {
    Default,
    Outlined,
    Underlined,
}
```
### Sizes
```rust
pub enum TextInputSize {
    Small,
    Normal,
    Large,
}
```
### Default bundle
```rust
FaWidgetBundle {
    style: ...,
    border_radius: ..., // depends on variant
    border_color: BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922)),
    ..default()
}
```
### Defautl style
```rust
Style {
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    border: ..., // depends on size
    padding: UiRect {
        left: Val::Px(5.0),
        right: Val::Px(5.0),
        top: Val::Px(1.0),
        bottom: Val::Px(1.0),
    },
    ..default()
}
```
### Can be created with
```rust
pub fn fa_text_input(
    &mut self,
    id: &str,
    placeholder: &str,
    size: Option<TextInputSize>,
    variant: Option<TextInputVariant>,
) -> Entity {
    // ..
}

let text_input = builder.fa_text_input(..);
```

### Getting input data
The input data can be read from `TextInputResource` within system.
```rust
fn my_system(input_resource: Res<TextInputResource>) {
    if let Some(data) = input_resource.inputs.get("#my-text-input-id") {
        println!("Data: {:?}", data);
    }
}
```
