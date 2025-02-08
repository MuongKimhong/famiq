# Interaction

All widgets provided by **Famiq** have `Interaction` component attached by default.
That means all those widgets will emit `FaInteractionEvent` to bevy's `EventReader` with either `Pressed`, `Hovered` or `None`.

```rust
pub struct FaInteractionEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub interaction: Interaction,
    pub widget: WidgetType,
}
```
Available widget types
```rust
pub enum WidgetType {
    Root,
    Button,
    Container,
    Text,
    FpsText,
    TextInput,
    ListView,
    ListViewItem,
    Selection,
    SelectionChoice,
    Circular,
    Modal,
    Image
}
```

## Handle interaction
You can write a bevy system to handle Famiq’s widgets interaction.

```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {
        if e.widget == WidgetType::Image && e.interaction == Interaction::Hovered {
            // make sure this works only with widgets that have id provided
            if let Some(id) = e.widget_id.as_ref() {
                match id.as_str() {
                    "#image-one-id" => {
                        // do something
                    },
                    "#image-two-id" => {
                        // do something
                    }
                    _ => ()
                }
            }
        }
    }
}
```
