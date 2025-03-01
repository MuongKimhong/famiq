# FaContainableResources

Used to **add**, **insert** and **remove** children from a containable widget (`fa_container`, `fa_modal`, `fa_listview`).

To use this `resource` on any containable widget, `id` must be provided.

see more info about [Resources](https://bevy-cheatbook.github.io/programming/res.html).

### Add children
append children to a containable widget.
```rust
fn my_system(mut containable_res: ResMut<FaContainableResource>) {
    // some logic

    containable_res.add_children("#container-id", &[new_text, new_button]);
}
```

### Insert children
Insert children at given index into containable widget.
```rust
fn my_system(mut containable_res: ResMut<FaContainableResource>) {
    // some logic
    //
    let index: usize = 2;
    containable_res.add_children("#container-id", index,  &[new_text, new_button]);
}
```

### Remove children
Remove children from a containable widget.

You must store `Entity` of widgets that you want to remove later on in a resource, so that it is
accessible from systems.
```rust
#[derive(Resource)]
struct MyResource(Vec<Entity>);

fn setup_ui(
    // other params
    mut commands: Commands
) {
    // some code ...

    let my_button = fa_button(&mut builder, "Press").build();
    commands.insert_resource(MyResource(vec![my_button]));
}

fn my_system(
    mut containable_res: ResMut<FaContainableResource>,
    my_resource: Res<MyResource>
) {
    // some logic

    containable_res.remove_children("#container-id", &my_resource.0);
}
```
