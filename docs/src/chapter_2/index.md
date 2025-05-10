# Events

Event is a big part of GUI library. So far, only **FaMouseEvent** is emitted by **Famiq**.

### What is **FaMouseEvent**?
This [event](https://github.com/MuongKimhong/famiq/blob/master/src/event_writer.rs) is emitted whenever one of the interaction is matched.
- mouse-left down
- mouse-right down
- mouse up
- mouse over
- mouse out
- mouse scroll

The event then can be read from bevy's `EventReader`.

## Handle interaction
You can write a bevy system that runs in `Update` schedule to handle the events.

Example,

```rust
// register system
app.add_systems(Update, on_mouse_over_text);

// system
fn on_mouse_over_text(mut events: EventReader<FaMouseEvent>) {
    for e in events.read() {
        // not mouse over text, early return
        if !e.is_mouse_over(WidgetType::Text) {
            return;
        }

        // ok, now some text has mouse over it!

        // let's check which text
        if let Some(id) = e.id.as_ref() {
            match id.as_str() {
                "#welcome-text" => todo!(),
                "#other-text" => todo!(),
                _ => {}
            }
        }
    }
}
```
see
- [WidgetType](https://docs.rs/famiq/latest/famiq/widgets/enum.WidgetType.html).
- [FaMouseEvent](https://docs.rs/famiq/latest/famiq/event_writer/struct.FaMouseEvent.html).
