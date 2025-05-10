# ProgressBar

There are 2 types of progress bar, **Normal** & **Indeterminate**. Default is **Indeterminate**.

### Usage
```rust
let bar = progress_bar!();
```
Return `Entity` which must be used inside a containable widget.

### Example
```rust
// default
let default_bar = progress_bar!();

// info & large
let info_large_bar = progress_bar!(class: "info large");

// warning & 50%
fa_query.insert_fnum("percent", 50.0);
let warning_bar = progress_bar!(class: "warning", model: "percent");

container!(children: [
    default_bar,
    info_large_bar,
    warning_bar
]);
```

#### Available attributes
- **id**
- **class**
- **color**
- **model**: if `model` is not provided it will be rendered as `indeterminate`. Type f32.
