# FaProgressBar

There are 2 types of progress bar, **Normal** & **Indeterminate**. Default is **Indeterminate**.


### Usage
```rust
let bar = fa_progress_bar(&mut builder).build();
```
Return `Entity` which must be used inside a containable widget.

### Available methods
- `id(&str)`
- `class(&str)`
- `display(&str)`
- `color(&str)`: set custom color.
- `percentage(f32)`: set percentage value of the bar.

### Example
```rust
// default
let default_bar = fa_progress_bar(&mut builder).build();

// info & large
let info_large_bar = fa_progress_bar(&mut builder)
    .class("is-info is-large")
    .build();

// warning & 50%
let warning_bar = fa_progress_bar(&mut builder)
    .percentage(50.0)
    .class("is-warning")
    .build();

fa_container(&mut builder)
    .children([default_bar, info_large_bar, warning_bar])
    .build();
```
