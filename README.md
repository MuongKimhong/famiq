![Static Badge](https://img.shields.io/badge/Version-0.3.0-blue)
![Static Badge](https://img.shields.io/badge/OS-Mac%20Linux%20Window%20Web-orange)
[![static Badge](https://img.shields.io/badge/crate.io-0.3.0-green)](https://crates.io/crates/famiq)

<p align="center">
  <img width="250" src="logo.png">
</p>

# Famiq
<p align="center">
    <img width="600" src="record.gif">
</p>

```rust
fn setup_ui( mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    fa_query.insert_num("count", 0);
    fa_query.insert_str("name", "");

    FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();

    container!(
        id: "#container",
        children: [
            text!(text: "$[name]", class: "my-2"),
            text_input!(placeholder: "Enter name", model: "name", class: "my-1"),
            button!(text: "Press me $[count]", id: "#btn")
        ]
    );
}

fn on_btn_press(mut events: EventReader<FaMouseEvent>, mut fa_query: FaQuery) {
    for e in events.read() {
        if e.is_button_pressed("#btn") {
            let count = fa_query.get_data_mut("count").unwrap().as_num_mut();
            *count += 1;
        }
    }
}
```

> [!WARNING]
> - **Famiq is new**, many useful features are missing.
> - It's not there yet.... but feel free to try it and share your feedback!

## Features
- Built-in useful widgets including text input, modal, progress bar and more
- Simple & lightweight, yet useful reactivity system
- JSON based styling (similar to css), keep your rust code clean!
- Yes it's fast!
- use it for GUI apps or directly in your games

## Demo
- [live demo](https://muongkimhong.github.io/famiq_live_demo/)
- [live demo source code](https://github.com/MuongKimhong/famiq_live_demo).

## Installation
Get latest version of `Famiq`
```
cargo add famiq
```
or in Cargo.toml
```toml
famiq = "0.3.0"
```

## Documentation
- [Rust-doc](https://docs.rs/famiq/latest/famiq/)
- [See the docs](https://muongkimhong.github.io/famiq/)

## Contributing
Famiq needs your contributions. Please see [contributing](https://github.com/MuongKimhong/famiq/blob/master/CONTRIBUTING.md).

## Versions
Currently, it supports only bevy 0.16.x onward.

## License
Famiq is released under the [MIT License](https://opensource.org/licenses/MIT).
