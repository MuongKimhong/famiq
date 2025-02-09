# Famiq

<p align="center">
  Simplifying UI Development in Bevy engine
</p>
<p align="center">
  <img width="200" height="200" src="logo.jpeg">
</p>

## What is Famiq?
**Famiq** is a UI library wrapped around Bevy UI module by providing default
widgets and a simple way to manage styles. 

Instead of writing Rust code for styling,
developers can define styles in a well known JSON file. These styles are then parsed
into Bevy's native UI styles, significantly reducing boilerplate code.

https://github.com/user-attachments/assets/739b0141-891e-4dea-ac26-3ab56f661992

[Demo code](https://github.com/MuongKimhong/famiq/tree/master/examples/demo)

hot-reload can be enabled during development so that any changes made to the json file
are reflected immediately in the running app, no need to re-compile.

## Installation
Get latest version of `Famiq`
```
cargo add famiq
```
or
```toml
famiq = "0.2.5"
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
