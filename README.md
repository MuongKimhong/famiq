![Static Badge](https://img.shields.io/badge/Version-0.2.6-blue)
![Static Badge](https://img.shields.io/badge/OS-Mac%20Linux%20Window-orange)

<p align="center">
  <img width="250" src="logo.png">
</p>

# Famiq
Build GUI with rust, based on ECS pattern, powered by Bevy game engine.

https://github.com/user-attachments/assets/739b0141-891e-4dea-ac26-3ab56f661992

See the [examples](https://github.com/MuongKimhong/famiq/tree/master/examples) directory.

```rust
use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamiqPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(
    mut commands: Commands,
    mut famiq_res: ResMut<FamiqResource>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d::default());

    let mut builder = FamiqBuilder::new(&mut commands, &mut famiq_res, &asset_server);

    let txt = fa_text(&mut builder, "Hello world").build();
    let btn = fa_button(&mut builder, "Press me").build();

    fa_container(&mut builder).children([txt, btn]).build();
}
```

## Installation
Get latest version of `Famiq`
```
cargo add famiq
```
or in Cargo.toml
```toml
famiq = "0.2.6"
```

## Documentation
- [Rust-doc](https://docs.rs/crate/famiq/latest)
- [See the docs](https://muongkimhong.github.io/famiq/)

## Contributing
Famiq needs your contributions. Please see [contributing](https://github.com/MuongKimhong/famiq/blob/master/CONTRIBUTING.md).

## Versions
Currently, it supports only 0.15.x onward. Currently, the latest version of bevy is `0.15.2`.

> [!NOTE]
> - **Famiq is new** and still in early stage of development, many useful features are missing.
> - This project **focuses on desktop GUI**, not Game UI.
> - It might not yet be fully ready for your needs, but feel free to try it and share your feedback!


## License
Famiq is released under the [MIT License](https://opensource.org/licenses/MIT).
