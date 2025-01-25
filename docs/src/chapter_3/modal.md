# FaModal

```
🟢 Doesn't need container
🟢 Accepts child/children
```

### API
```rust
pub fn fa_modal<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaModalBuilder<'a> {
    // ..
}
```

### Usage
```rust
let modal = fa_modal(&mut builder).build();
```

### Show/Hide modal
Modals can be shown or hided by updating `FaModalState` component.

Example,
```rust

// store modal entity here to show/hide in future
#[derive(Resource)]
struct Modals {
    entity: Entity
}

fn setup_ui(
    // other args required by Famiq..
    mut modals: ResMut<Modals>
) {
    let text = fa_text(&mut builder, "Hello world").build();

    let modal = fa_modal(&mut builder)
        .children(vec![text])
        .build();

    modals.entity = modal;
}

// example system handle button press
fn on_btn_press_system(
    modals: Res<Modals>,
    mut modals_q: Query<&mut FaModalState>
) {
    // some other code to handle button press event..

    // set to true to show modal, false to hide.
    if let Ok(mut state) = modals_q.get_mut(modals.entity) {
        state.0 = true;
    }
}
```
