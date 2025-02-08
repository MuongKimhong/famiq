# Famiq

<p align="center">
  Simplifying UI Development in Bevy engine
</p>
<p align="center">
  <img width="200" height="200" src="demo.gif">
</p>

## What is Famiq?
**Famiq** is a UI library wrapped around Bevy UI module by providing default
widgets and a simple way to manage styles. Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

```rust
let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);

let txt = fa_text(&mut builder, "SIGN UP").build();

let f_name = fa_text_input(&mut builder, "First name").build();
let l_name = fa_text_input(&mut builder, "Last name").build();
let name_container = fa_container(&mut builder)
    .id("#name-container")
    .children([f_name, l_name])
    .build();

let ask = fa_selection(&mut builder, "Select your subscription").choices(["Personal", "Team"]).build();
let ask_container = fa_container(&mut builder).id("#ask-container")
    .children([ask])
    .build();

let btn = fa_button(&mut builder, "Confirm").build();

fa_container(&mut builder).id("#container")
    .children([txt, name_container, ask_container, btn])
    .build();
```
<p align="center">
  <img src="screenshot.png">
</p>


hot-reload can be enabled during development so that any changes made to the json file
are reflected immediately in the running app, no need to re-compile.

## Installation
Get latest version of `Famiq`
```
cargo add famiq
```
or
```toml
famiq = "0.2.4"
```

## Documentation
- [Rustdoc](https://docs.rs/crate/famiq/latest)
- [See the docs](https://muongkimhong.github.io/famiq/)

## Contributing
Famiq needs your contributions to grow. Please see [contributing](https://github.com/MuongKimhong/famiq/blob/master/CONTRIBUTING.md).

## Versions
Famiq is in early stage of development. Currently, it supports only 0.15.x onward.


## Goals
- API Improvements
- Provide more useful default widgets

## License
Famiq is released under the [MIT License](https://opensource.org/licenses/MIT).
