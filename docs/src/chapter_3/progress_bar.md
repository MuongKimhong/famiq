# FaProgressBar

```
🟡 Needs container
🟡 Doesn't accept child/children
```

### Sizes
```rust
pub enum ProgressBarSize {
    Normal,
    Small,
    Large
}
```

### Colors
```rust
pub enum ProgressBarColor {
    Default,
    Primary,
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark
}
```

### Widget API
```rust
pub fn fa_progress_bar<'a>(
    builder: &'a mut FamiqWidgetBuilder
) -> FaProgressBarBuilder<'a> {
    // ..
}
```

### Usage
```rust
let bar = fa_progress_bar(&mut builder).build();
```
Return `Entity` of the widget which must be used as child of `FaContainer` widget.
- `percentage()`: by default, `fa_progress_bar` is indeterminate, use `percentage()` to set percentage.


### Built-in classes
- Color: `is-primary`, `is-secondary`, `is-warning`, `is-info`, `is-success`, `is-danger`.

- Size: `is-small`, `is-normal`, `is-large`.

### Example
```rust
// default
let default_bar = fa_progress_bar(&mut builder)
    .id("#default-bar")
    .build();

// info & large
let info_large_bar = fa_progress_bar(&mut builder)
    .class("is-info is-large")
    .build();

// warning & 50%
let warning_bar = fa_progress_bar(&mut builder)
    .id("#warning-bar")
    .percentage(50.0)
    .class("is-warning")
    .build();

fa_container(&mut builder)
    .children([default_bar, info_large_bar, warning_bar])
    .build();
```

### Resource
```rust
pub struct FaProgressBarResource;
```
- `FaProgressBarResouce` can be used to set get & set percentage of the bars or set to indeterminate.
  #### Available methods:
  - `get_percentage_by_id`: get percentage by id, return `None` if id doesn't exist.
  - `get_percentage_by_entity`: get percentage by entity, return `None` if entity doesn't exist.
  - `set_percentage_by_id`: set percentage by id.
  - `set_percentage_by_entity`: set percentage by entity.

  #### Example of `FaProgressBarResource`
  ```rust
  fn my_system(bar_res: Res<FaProgressBarResource>) {
      // some logic ..

      // return None as #default-bar is indeterminate
      let default_bar_percent = bar.get_percentage_by_id("#default-bar");

      // return 50.0
      let warning_bar_percent = bar.get_percentage_by_id("#warning-bar").unwrap();

      // set to None to make bar indeterminate
      bar.set_percentage_by_id("#warning-bar", None);

      // set #default-bar to 30 percent
      bar.set_percentage_by_id("#default-bar", Some(30.0));
  }
  ```
