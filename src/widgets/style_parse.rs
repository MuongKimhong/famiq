use bevy::prelude::*;

// parse string of styles from json file to bevy ui style
// example:
// JSON value
// "background_color": "srgba 0.357, 0.565, 0.941, 0.902"
// to
// Bevy ui style value
// background_color: BackgroundColor(Color::srgba(0.357, 0.565, 0.941, 0.902))
//

fn parse_color_components(value: &str) -> Option<Vec<f32>> {
    let components: Vec<f32> = value
        .split(',')
        .map(|s| s.trim().parse::<f32>())
        .filter_map(Result::ok)
        .collect();

    // accept only number
    if components.is_empty() || components.iter().any(|&v| v.is_nan()) {
        return None;
    }
    Some(components)
}

fn parse_srgba(value: &str) -> Option<Color> {
    if let Some(value) = parse_color_components(value) {
        if value.len() == 4 {
            return Some(Color::srgba(value[0], value[1], value[2], value[3]));
        }
    }
    None
}

fn parse_linear_rgba(value: &str) -> Option<Color> {
    if let Some(value) = parse_color_components(value) {
        if value.len() == 4 {
            return Some(Color::linear_rgba(value[0], value[1], value[2], value[3]));
        }
    }
    None
}

fn parse_hsla(value: &str) -> Option<Color> {
    if let Some(value) = parse_color_components(value) {
        if value.len() == 4 {
            return Some(Color::hsla(value[0], value[1], value[2], value[3]));
        }
    }
    None
}

pub fn parse_color(value: &str) -> Option<Color> {
    if value.starts_with("srgba ") {
        parse_srgba(value.strip_prefix("srgba ").unwrap())
    } else if value.starts_with("linear_rgba ") {
        parse_linear_rgba(value.strip_prefix("linear_rgba ").unwrap())
    } else if value.starts_with("hsla ") {
        parse_hsla(value.strip_prefix("hsla ").unwrap())
    } else {
        None
    }
}

pub fn parse_val(value: &str) -> Option<Val> {
    if value.trim() == "auto" {
        return Some(Val::Auto);
    }
    let units: &[(&str, fn(f32) -> Val)] = &[
        ("px", Val::Px),
        ("%", Val::Percent),
        ("vw", Val::Vw),
        ("vh", Val::Vh),
    ];
    for &(suffix, constructor) in units {
        if let Some(number_str) = value.strip_suffix(suffix) {
            return number_str.trim().parse::<f32>().ok().map(constructor);
        }
    }
    None
}

pub fn parse_background_color(value: &str) -> Option<BackgroundColor> {
    if let Some(color) = parse_color(value) {
        return Some(BackgroundColor(color));
    }
    None
}

pub fn parse_border_color(value: &str) -> Option<BorderColor> {
    if let Some(color) = parse_color(value) {
        return Some(BorderColor(color));
    }
    None
}

pub fn parse_border_radius(value: &str) -> Option<BorderRadius> {
    let components: Vec<&str> = value.split_whitespace().collect();

    if components.len() != 4 {
        return None;
    }
    let top_left = parse_val(components[0])?;
    let top_right = parse_val(components[1])?;
    let bottom_right = parse_val(components[2])?;
    let bottom_left = parse_val(components[3])?;

    Some(BorderRadius {
        top_left,
        top_right,
        bottom_right,
        bottom_left,
    })
}

pub fn parse_ui_rect(value: &str) -> Option<UiRect> {
    let components: Vec<&str> = value.split_whitespace().collect();

    if components.len() != 4 {
        return None;
    }
    let left = parse_val(components[0])?;
    let right = parse_val(components[1])?;
    let top = parse_val(components[2])?;
    let bottom = parse_val(components[3])?;

    Some(UiRect {
        left,
        right,
        top,
        bottom,
    })
}

pub fn parse_visibility(value: &str) -> Option<Visibility> {
    match value.to_lowercase().as_str() {
        "visible" => Some(Visibility::Visible),
        "hidden" => Some(Visibility::Hidden),
        "inherited" => Some(Visibility::Inherited),
        _ => None,
    }
}

pub fn parse_display(value: &str) -> Option<Display> {
    match value.to_lowercase().as_str() {
        "flex" => Some(Display::Flex),
        "grid" => Some(Display::Grid),
        "block" => Some(Display::Block),
        _ => None,
    }
}

pub fn parse_position_type(value: &str) -> Option<PositionType> {
    match value.to_lowercase().as_str() {
        "relative" => Some(PositionType::Relative),
        "absolute" => Some(PositionType::Absolute),
        _ => None,
    }
}

pub fn parse_overflow_x(value: &str) -> Option<OverflowAxis> {
    match value.to_lowercase().as_str() {
        "visible" => Some(OverflowAxis::Visible),
        "clip" => Some(OverflowAxis::Clip),
        "hidden" => Some(OverflowAxis::Hidden),
        _ => None,
    }
}

pub fn parse_overflow_y(value: &str) -> Option<OverflowAxis> {
    parse_overflow_x(value)
}

pub fn parse_z_index(value: &str) -> Option<ZIndex> {
    let parts: Vec<&str> = value.split_whitespace().collect();

    if parts.len() != 2 {
        return None;
    }
    let prefix = parts[0].to_lowercase();
    let number = parts[1].parse::<i32>().ok()?;

    match prefix.as_str() {
        "local" => Some(ZIndex::Local(number)),
        "global" => Some(ZIndex::Global(number)),
        _ => None,
    }
}

pub fn parse_align_items(value: &str) -> Option<AlignItems> {
    match value.to_lowercase().as_str() {
        "default" => Some(AlignItems::Default),
        "start" => Some(AlignItems::Start),
        "End" => Some(AlignItems::End),
        "flex_start" => Some(AlignItems::FlexStart),
        "flex_end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        "base_line" => Some(AlignItems::Baseline),
        "stretch" => Some(AlignItems::Stretch),
        _ => None,
    }
}

pub fn parse_align_self(value: &str) -> Option<AlignSelf> {
    match value.to_lowercase().as_str() {
        "auto" => Some(AlignSelf::Auto),
        "start" => Some(AlignSelf::Start),
        "End" => Some(AlignSelf::End),
        "flex_start" => Some(AlignSelf::FlexStart),
        "flex_end" => Some(AlignSelf::FlexEnd),
        "center" => Some(AlignSelf::Center),
        "base_line" => Some(AlignSelf::Baseline),
        "stretch" => Some(AlignSelf::Stretch),
        _ => None,
    }
}

pub fn parse_justify_items(value: &str) -> Option<JustifyItems> {
    match value.to_lowercase().as_str() {
        "default" => Some(JustifyItems::Default),
        "start" => Some(JustifyItems::Start),
        "End" => Some(JustifyItems::End),
        "center" => Some(JustifyItems::Center),
        "base_line" => Some(JustifyItems::Baseline),
        "stretch" => Some(JustifyItems::Stretch),
        _ => None,
    }
}

pub fn parse_justify_content(value: &str) -> Option<JustifyContent> {
    match value.to_lowercase().as_str() {
        "default" => Some(JustifyContent::Default),
        "start" => Some(JustifyContent::Start),
        "End" => Some(JustifyContent::End),
        "center" => Some(JustifyContent::Center),
        "flex_start" => Some(JustifyContent::FlexStart),
        "flex_end" => Some(JustifyContent::FlexEnd),
        "stretch" => Some(JustifyContent::Stretch),
        "space_between" => Some(JustifyContent::SpaceBetween),
        "space_evenly" => Some(JustifyContent::SpaceEvenly),
        "space_around" => Some(JustifyContent::SpaceAround),
        _ => None,
    }
}

pub fn parse_flex_direction(value: &str) -> Option<FlexDirection> {
    match value.to_lowercase().as_str() {
        "row" => Some(FlexDirection::Row),
        "column" => Some(FlexDirection::Column),
        "row_reverse" => Some(FlexDirection::RowReverse),
        "column_reverse" => Some(FlexDirection::ColumnReverse),
        _ => None,
    }
}

pub fn parse_flex_wrap(value: &str) -> Option<FlexWrap> {
    match value.to_lowercase().as_str() {
        "no_wrap" => Some(FlexWrap::NoWrap),
        "wrap" => Some(FlexWrap::Wrap),
        "wrap_reverse" => Some(FlexWrap::WrapReverse),
        _ => None,
    }
}

pub fn parse_grid_auto_flow(value: &str) -> Option<GridAutoFlow> {
    match value.to_lowercase().as_str() {
        "row" => Some(GridAutoFlow::Row),
        "column" => Some(GridAutoFlow::Column),
        "row_dense" => Some(GridAutoFlow::RowDense),
        "column_dense" => Some(GridAutoFlow::ColumnDense),
        _ => None,
    }
}
