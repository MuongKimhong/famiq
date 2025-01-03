# Interaction

All widgets provided by **Famiq** have `Interaction` component attached by default.
That means all those widgets will emit `FaInteractionEvent` to bevy's `EventReader` with either `Pressed`, `Hovered` or `None`.

```rust
pub struct FaInteractionEvent {
    pub entity: Entity,
    pub widget_id: String,
    pub interaction: Interaction,
    pub widget: WidgetType,
}
```
Available widget types
```rust
pub enum WidgetType {
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
    Modal
}
```

## Handle interaction
You can write a bevy system to handle Famiq’s widgets interaction.

```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {
        if e.widget == WidgetType::Button && e.interaction == Interaction::Pressed {

            // handle specific button using its id
            match e.widget_id.as_str() {
                "#my-login-btn" => {
                    // do something with login
                },
                "#my-forgot-password-btn" => {
                    // do something with forgot password
                }
                _ => ()
            }

        }
    }
}
```
