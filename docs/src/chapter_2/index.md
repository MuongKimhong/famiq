# Interaction

There are 3 types of interaction
- Pressed
- Hovered
- None (leaving hovered)

**Famiq** emits an event with type of `FaInteractionEvent` whenever one of the iteraction is matched.
The event then can be read from bevy's `EventReader`.

Each event has a widget type flag.

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
    Button,
    Container,
    Text,
    FpsText,
    TextInput,
    TextInputTogglePasswordIcon,
    ListView,
    ListViewItem,
    Selection,
    SelectionChoice,
    Circular,
    ProgressBar,
    Image
}
```

## Handle interaction
You can write a bevy system that runs in `Update` schedule to handle Famiq’s widgets interaction.

Example,

```rust
fn handle_button_press_system(mut events: EventReader<FaInteractionEvent>) {
    for e in events.read() {
        if !e.is_pressed(WidgetType::Button) {
            return;
        }

        // make sure this works only with widgets that have id provided
        if let Some(id) = e.widget_id.as_ref() {
            match id.as_str() {
                "#button-one" => {
                    // do something when #button-one is pressed
                },
                "#button-two" => {
                    // do something when #button-two is pressed
                }
                _ => ()
            }
        }
    }
}
```
Beside `is_pressed`, there are also `is_hovered` and `is_left` (from hovered -> none).
