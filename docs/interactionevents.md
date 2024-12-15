## Interaction Events

All widgets created with `FaWidgetBundle` will have `Interaction` component
attached by default.

```rust
pub struct FaInteractionEvent {
    pub entity: Entity,
    pub widget_id: String,
    pub interaction_type: Interaction,
    pub widget_type: WidgetType,
}
```
Available widget types
```rust
pub enum WidgetType {
    Button,
    Container,
    Text,
    TextInput,
    ListView,
    ListViewItem,
    Selection,
    SelectionItem,
}
```

### Handle events
You can write a bevy system to handle Famiq's widgets interaction events
```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {
        if e.widget_type == WidgetType::Button && e.interaction_type == Interaction::Pressed {
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
By default, all widgets emit a `FaInteractionEvent` upon any interaction.
You can narrow it down to a specific widget type by checking `e.widget_type == WidgetType::Button`
to avoid unnecessary code processing.
