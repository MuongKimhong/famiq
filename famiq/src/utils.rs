use bevy::picking::backend::PointerHits;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::ui::UiStack;
use bevy::window::WindowResized;
use bevy::asset::{io::AssetSourceId, AssetPath, AssetPlugin};
use bevy::prelude::*;
use cosmic_text::Color as CosmicColor;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use once_cell::sync::Lazy;
use std::any::Any;

use crate::plugin::{CursorIcons, CursorType};
use crate::widgets::{style_parse::*, ReactiveModelKey, WidgetAttributes};
use crate::widgets::{WidgetStyle, DefaultWidgetConfig, WidgetColor, WidgetSize};
use crate::widgets::color::*;
use crate::reactivity::RVal;
use crate::errors::*;
use crate::widgets::{WidgetId, WidgetClasses, TooltipEntity, IsFamiqTooltip};

pub(crate) fn read_styles_json_file(path: &str) -> Result<HashMap<String, WidgetStyle>, StylesFileError> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(StylesFileError::StylesFileDoesNotExist),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| StylesFileError::ReadStylesFileFail)?;

    contents = contents
        .replace("\u{a0}", " ")
        .replace("\t", "    ")
        .replace("\\\"", "\"");

    let styles: HashMap<String, WidgetStyle> = serde_json::from_str(&contents)
        .map_err(|_| StylesFileError::ReadStylesFromFileToStructFail)?;

    Ok(styles)
}

/// extract bevy Val enum value
pub(crate) fn extract_val(val: Val) -> Option<f32> {
    match val {
        Val::Px(value) => Some(value),
        Val::Percent(value) => Some(value),
        Val::Vw(value) => Some(value),
        Val::Vh(value) => Some(value),
        _ => None,
    }
}

pub fn adjust_color(percentage: f32, color: &Color, darken: bool) -> Option<Color> {
    let factor = percentage / 100.0;

    let adjust = |channel: f32| -> f32 {
        if darken {
            (channel - factor).max(0.0)
        } else {
            (channel + factor).min(1.0)
        }
    };

    match color {
        Color::Srgba(value) => {
            let mut value = *value;
            value.red = adjust(value.red);
            value.green = adjust(value.green);
            value.blue = adjust(value.blue);
            Some(Color::Srgba(value.into()))
        }
        Color::LinearRgba(value) => {
            let mut value = *value;
            value.red = adjust(value.red);
            value.green = adjust(value.green);
            value.blue = adjust(value.blue);
            Some(Color::Srgba(value.into()))
        }
        Color::Hsla(value) => {
            let mut value = *value;
            value.lightness = adjust(value.lightness);
            Some(Color::Hsla(value))
        }
        _ => None,
    }
}


pub fn darken_color(percentage: f32, color: &Color) -> Option<Color> {
    adjust_color(percentage, color, true)
}

pub fn lighten_color(percentage: f32, color: &Color) -> Option<Color> {
    adjust_color(percentage, color, false)
}

pub(crate) fn get_embedded_asset_path(file_path: &str) -> AssetPath {
    // path: relative to embedded_assets dir

    let path = Path::new("famiq").join(file_path);
    let source = AssetSourceId::from("embedded");
    AssetPath::from_path(&path).with_source(source).into_owned()
}

pub fn create_test_app() -> App {
    let mut app = App::new();
    // Note the use of `MinimalPlugins` instead of `DefaultPlugins`, as described above.
    app.add_plugins(MinimalPlugins);
    app.add_plugins(UiPickingPlugin);
    app.add_plugins(AssetPlugin::default());
    app.init_resource::<Assets<Shader>>();
    app.add_event::<PointerHits>();
    app.insert_resource(Events::<WindowResized>::default());
    app.insert_resource(UiStack::default());
    app.init_asset::<Font>();
    app.init_asset::<Image>();
    // Spawning a fake window allows testing systems that require a window.
    app.world_mut().spawn(Window::default());
    app
}

pub fn mask_string(input: &str) -> String {
    "*".repeat(input.len())
}

pub(crate) fn insert_class_id(
    commands: &mut Commands,
    entity: Entity,
    id: &Option<String>,
    class: &Option<String>
) {
    if let Some(id) = id {
        commands.entity(entity).insert(WidgetId(id.to_owned()));
    }
    if let Some(class) = class {
        commands.entity(entity).insert(WidgetClasses(class.to_owned()));
    }
}

pub(crate) fn insert_class_id_world(
    world: &mut World,
    entity: Entity,
    id: &Option<String>,
    class: &Option<String>
) {
    if let Some(id) = id {
        world.entity_mut(entity).insert(WidgetId(id.to_owned()));
    }
    if let Some(class) = class {
        world.entity_mut(entity).insert(WidgetClasses(class.to_owned()));
    }
}

pub(crate) fn insert_model(
    commands: &mut Commands,
    entity: Entity,
    model: &Option<String>
) {
    if let Some(model) = model {
        commands.entity(entity).insert(ReactiveModelKey(model.to_owned()));
    }
}

pub(crate) fn _handle_apply_margin(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetConfig,
    node: &mut Node
) {
    if let Some(margin) = &widget_style.margin {
        // If the full margin is defined, set it and skip individual sub-properties
        if let Some(v) = parse_ui_rect(&margin) {
            node.margin = v;
        }
    } else {
        // If the full margin is not defined, check sub-properties individually
        if let Some(margin_left) = &widget_style.margin_left {
            if let Some(v) = parse_val(margin_left) {
                node.margin.left = v;
            }
        } else {
            node.margin.left = default_widget_entity.node.margin.left.clone();
        }

        if let Some(margin_right) = &widget_style.margin_right {
            if let Some(v) = parse_val(margin_right) {
                node.margin.right = v;
            }
        } else {
            node.margin.right = default_widget_entity.node.margin.right.clone();
        }

        if let Some(margin_top) = &widget_style.margin_top {
            if let Some(v) = parse_val(margin_top) {
                node.margin.top = v;
            }
        } else {
            node.margin.top = default_widget_entity.node.margin.top.clone();
        }

        if let Some(margin_bottom) = &widget_style.margin_bottom {
            if let Some(v) = parse_val(margin_bottom) {
                node.margin.bottom = v;
            }
        } else {
            node.margin.bottom = default_widget_entity.node.margin.bottom.clone();
        }
    }
}

pub(crate) fn _handle_apply_padding(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetConfig,
    node: &mut Node
) {
    if let Some(padding) = &widget_style.padding {
        // If the full padding is defined, set it and skip individual sub-properties
        if let Some(v) = parse_ui_rect(&padding) {
            node.padding = v;
        }
    } else {
        // If the full padding is not defined, check sub-properties individually
        if let Some(padding_left) = &widget_style.padding_left {
            if let Some(v) = parse_val(padding_left) {
                node.padding.left = v;
            }
        } else {
            node.padding.left = default_widget_entity.node.padding.left.clone();
        }

        if let Some(padding_right) = &widget_style.padding_right {
            if let Some(v) = parse_val(padding_right) {
                node.padding.right = v;
            }
        } else {
            node.padding.right = default_widget_entity.node.padding.right.clone();
        }

        if let Some(padding_top) = &widget_style.padding_top {
            if let Some(v) = parse_val(padding_top) {
                node.padding.top = v;
            }
        } else {
            node.padding.top = default_widget_entity.node.padding.top.clone();
        }

        if let Some(padding_bottom) = &widget_style.padding_bottom {
            if let Some(v) = parse_val(padding_bottom) {
                node.padding.bottom = v;
            }
        } else {
            node.padding.bottom = default_widget_entity.node.padding.bottom.clone();
        }
    }
}

pub(crate) fn _handle_apply_border(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetConfig,
    node: &mut Node
) {
    if let Some(border) = &widget_style.border {
        // If the full border is defined, set it and skip individual sub-properties
        if let Some(v) = parse_ui_rect(&border) {
            node.border = v;
        }
    } else {
        // If the full border is not defined, check sub-properties individually
        if let Some(border_left) = &widget_style.border_left {
            if let Some(v) = parse_val(border_left) {
                node.border.left = v;
            }
        } else {
            node.border.left = default_widget_entity.node.border.left.clone();
        }

        if let Some(border_right) = &widget_style.border_right {
            if let Some(v) = parse_val(border_right) {
                node.border.right = v;
            }
        } else {
            node.border.right = default_widget_entity.node.border.right.clone();
        }

        if let Some(border_top) = &widget_style.border_top {
            if let Some(v) = parse_val(border_top) {
                node.border.top = v;
            }
        } else {
            node.border.top = default_widget_entity.node.border.top.clone();
        }

        if let Some(border_bottom) = &widget_style.border_bottom {
            if let Some(v) = parse_val(border_bottom) {
                node.border.bottom = v;
            }
        } else {
            node.border.bottom = default_widget_entity.node.border.bottom.clone();
        }
    }
}

pub(crate) fn _handle_apply_box_shadow(
    widget_style: &WidgetStyle,
    default_widget_entity: &DefaultWidgetConfig,
    box_shadow: &mut BoxShadow
) {
    let mut shadow_style = box_shadow.0[0];

    if let Some(shadow_color_value) = &widget_style.shadow_color {
        if let Some(v) = parse_color(&shadow_color_value) {
            shadow_style.color = v;
        }
    } else {
        shadow_style.color = default_widget_entity.box_shadow.0[0].color;
    }

    if let Some(shadow_spread_value) = &widget_style.shadow_spread {
        if let Some(v) = parse_val(&shadow_spread_value) {
            shadow_style.spread_radius = v;
        }
    } else {
        shadow_style.spread_radius = default_widget_entity.box_shadow.0[0].spread_radius;
    }

    if let Some(shadow_blur_value) = &widget_style.shadow_blur {
        if let Some(v) = parse_val(&shadow_blur_value) {
            shadow_style.blur_radius = v;
        }
    } else {
        shadow_style.blur_radius = default_widget_entity.box_shadow.0[0].blur_radius;
    }

    if let Some(shadow_x_value) = &widget_style.shadow_x_offset {
        if let Some(v) = parse_val(&shadow_x_value) {
            shadow_style.x_offset = v;
        }
    } else {
        shadow_style.x_offset = default_widget_entity.box_shadow.0[0].x_offset;
    }

    if let Some(shadow_y_value) = &widget_style.shadow_y_offset {
        if let Some(v) = parse_val(&shadow_y_value) {
            shadow_style.y_offset = v;
        }
    } else {
        shadow_style.y_offset = default_widget_entity.box_shadow.0[0].y_offset;
    }
}

pub(crate) fn _change_cursor_icon(
    commands: &mut Commands,
    res: &Res<CursorIcons>,
    window_entity: Entity,
    _type: CursorType
) {
    commands.entity(window_entity).insert(match _type {
        CursorType::Pointer => res.pointer.clone(),
        CursorType::Text => res.text.clone(),
        _ => res.normal.clone(),
    });
}

/// Turn WigetColor to actual color
pub(crate) fn get_color(color: &WidgetColor) -> Color {
    match color {
        WidgetColor::Dark => BLACK_COLOR,
        WidgetColor::Primary => PRIMARY_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_DARK_COLOR,
        WidgetColor::Secondary => SECONDARY_COLOR,
        WidgetColor::Success => SUCCESS_COLOR,
        WidgetColor::SuccessDark => SUCCESS_DARK_COLOR,
        WidgetColor::Danger => DANGER_COLOR,
        WidgetColor::DangerDark => DANGER_DARK_COLOR,
        WidgetColor::Warning => WARNING_COLOR,
        WidgetColor::WarningDark => WARNING_DARK_COLOR,
        WidgetColor::Info => INFO_COLOR,
        WidgetColor::InfoDark => INFO_DARK_COLOR,
        WidgetColor::Transparent => TRANSPARENT_COLOR,
        WidgetColor::Custom(color) => {
            if let Some(parsed_color) = built_in_color_parser(color) {
                parsed_color
            } else {
                DEFAULT_COLOR
            }
        },
        WidgetColor::CustomSrgba(srgba) => {
            Color::srgba(srgba.0, srgba.1, srgba.2, srgba.3)
        },
        _ => WHITE_COLOR
    }
}

pub(crate) fn get_text_color(variant: &WidgetColor) -> Color {
    match variant {
        WidgetColor::Secondary => WHITE_COLOR,
        WidgetColor::PrimaryDark => PRIMARY_COLOR,
        WidgetColor::SuccessDark => SUCCESS_COLOR,
        WidgetColor::DangerDark => DANGER_COLOR,
        WidgetColor::WarningDark => WARNING_COLOR,
        WidgetColor::InfoDark => INFO_COLOR,
        WidgetColor::Dark => WHITE_COLOR,
        _ => BLACK_COLOR,
    }
}

pub(crate) fn get_text_size(size: &WidgetSize) -> f32 {
    let size_small = 12.0;
    let size_normal = 14.0;
    let size_large = 18.0;

    match size {
        WidgetSize::Small => size_small,
        WidgetSize::Large => size_large,
        _ => size_normal
    }
}

pub(crate) fn show_tooltip(
    entity: Option<&TooltipEntity>,
    tooltip_q: &mut Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
    parent_translation: Vec3
) {
    if entity.is_some() {
        if let Ok((mut node, mut transform)) = tooltip_q.get_mut(entity.unwrap().0) {
            transform.translation = parent_translation;
            node.display = Display::Flex;
        }
    }
}

pub(crate) fn hide_tooltip(
    entity: Option<&TooltipEntity>,
    tooltip_q: &mut Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
) {
    if entity.is_some() {
        if let Ok((mut node, _)) = tooltip_q.get_mut(entity.unwrap().0) {
            node.display = Display::None;
        }
    }
}

/// Convert mouse position from world to UI node local position
pub fn mouse_pos_to_local_node_pos(
    mouse_pos: &Vec2,
    computed_node: &ComputedNode,
    node_transform: &GlobalTransform
) -> Vec2 {
    let scale_factor = computed_node.inverse_scale_factor();
    let size = computed_node.size();
    let paddings = computed_node.padding();

    let node_top = (node_transform.translation().y * scale_factor) - ((size.y * scale_factor) / 2.0);
    let padding_top = paddings.top * scale_factor;
    let node_left = (node_transform.translation().x * scale_factor) - ((size.x * scale_factor) / 2.0);
    let padding_left = paddings.left * scale_factor;

    let pos_x = (mouse_pos.x) - (node_left + padding_left);
    let pos_y = (mouse_pos.y) - (node_top + padding_top);

    Vec2::new(pos_x, pos_y)
}

/// Convert cosmic-text Color rgba (u8) to bevy srgba (f32)
pub fn cosmic_rgba_to_bevy_srgba(cosmic_color: CosmicColor) -> Color {
    let (r, g, b, a) = cosmic_color.as_rgba_tuple();
    Color::from(Srgba::rgba_u8(r, g, b, a))
}

/// Convert bevy color to cosmic rgba, support only srgba and linear-rgba
pub fn bevy_color_to_cosmic_rgba(bevy_color: Color) -> Option<CosmicColor> {
    match bevy_color {
        Color::Srgba(value) => {
            return Some(CosmicColor::rgba(
                (value.red * 255.0) as u8,
                (value.green * 255.0) as u8,
                (value.blue * 255.0) as u8,
                (value.alpha * 255.0) as u8,
            ));
        }
        Color::LinearRgba(value) => {
            return Some(CosmicColor::rgba(
                (value.red * 255.0) as u8,
                (value.green * 255.0) as u8,
                (value.blue * 255.0) as u8,
                (value.alpha * 255.0) as u8,
            ));
        }
        _ => {}
    }
    None
}

pub static REACTIVE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\[(.*?)\]").unwrap());

/// Extract reactive key(s) from string.
/// Eg, "Hello $[some_thing]" -> "some_thing".
pub fn get_reactive_key(text: &str) -> Vec<String> {
    let mut keys = HashSet::new();
    for capture in REACTIVE_REGEX.captures_iter(text) {
        if let Some(matched) = capture.get(1) {
            keys.insert(matched.as_str().to_string());
        }
    }
    keys.into_iter().collect()
}

/// Example, key "color", value: "blue", text: "This is $[color]".
/// result: "This is blue".
pub fn replace_reactive_keys(
    old_text: &str,
    reactive_keys: &Vec<String>,
    reactive_data: &HashMap<String, RVal>
) -> String {
    let mut new_text = old_text.to_string();

    for key in reactive_keys {
        let placeholder = format!("$[{}]", key);
        if let Some(value) = reactive_data.get(key) {
            new_text = new_text.replace(&placeholder, &value.to_string());
        }
    }
    new_text
}

pub fn replace_reactive_keys_common_attrs(
    attr: &mut WidgetAttributes,
    reactive_data: &HashMap<String, RVal>,
    all_reactive_keys: &mut Vec<String>
) {
    if let Some(class) = attr.class.as_mut() {
        let class_r_keys = get_reactive_key(class);
        *class = replace_reactive_keys(class, &class_r_keys, reactive_data);
        all_reactive_keys.extend(class_r_keys);
    }

    if let Some(id) = attr.id.as_mut() {
        let id_r_keys = get_reactive_key(id);
        *id = replace_reactive_keys(id, &id_r_keys, reactive_data);
        all_reactive_keys.extend(id_r_keys);
    }

    if let Some(width) = attr.width.as_mut() {
        let width_r_keys = get_reactive_key(width);
        *width = replace_reactive_keys(width, &width_r_keys, reactive_data);
        all_reactive_keys.extend(width_r_keys);

        if let Some(parsed_width) = parse_val(&width) {
            attr.node.width = parsed_width;
        }
    }

    if let Some(height) = attr.height.as_mut() {
        let height_r_keys = get_reactive_key(height);
        *height = replace_reactive_keys(height, &height_r_keys, reactive_data);
        all_reactive_keys.extend(height_r_keys);

        if let Some(parsed_height) = parse_val(&height) {
            attr.node.height = parsed_height;
        }
    }

    if let Some(display) = attr.display.as_mut() {
        let display_r_keys = get_reactive_key(display);
        *display = replace_reactive_keys(display, &display_r_keys, reactive_data);
        all_reactive_keys.extend(display_r_keys);

        if let Some(parsed_display) = parse_display(&display) {
            attr.node.display = parsed_display;
        }
    }

    if let WidgetColor::Custom(ref mut c) = attr.color {
        let color_r_keys = get_reactive_key(c);
        *c = replace_reactive_keys(c, &color_r_keys, reactive_data);
        all_reactive_keys.extend(color_r_keys);
    }

    if attr.has_tooltip {
        let tooltip_r_keys = get_reactive_key(&attr.tooltip_text);
        attr.tooltip_text = replace_reactive_keys(&attr.tooltip_text, &tooltip_r_keys, reactive_data);
        all_reactive_keys.extend(tooltip_r_keys);
    }
}

pub trait TypeName {
    fn type_name() -> &'static str;
}

impl TypeName for f32 {
    fn type_name() -> &'static str { "f32" }
}

impl TypeName for bool {
    fn type_name() -> &'static str { "boolean" }
}

impl TypeName for &str {
    fn type_name() -> &'static str { "&str" }
}

impl TypeName for Vec<String> {
    fn type_name() -> &'static str { "Vec<String>" }
}

impl TypeName for [&str] {
    fn type_name() -> &'static str { "[&str]" }
}

pub fn check_val_type<T: TypeName>(_val: &T) -> &'static str {
    T::type_name()
}

pub fn to_rval<T: Any + ToString + TypeName>(value: T) -> Result<RVal, ToRValErr> {
    match check_val_type(&value) {
        "boolean" => {
            let any = &value as &dyn Any;
            match any.downcast_ref::<bool>() {
                Some(val) => Ok(RVal::Bool(*val)),
                None => Err(ToRValErr::ConvertToRValFail)
            }
        }
        "f32" => {
            let any = &value as &dyn Any;
            match any.downcast_ref::<f32>() {
                Some(val) => Ok(RVal::FNum(*val)),
                None => Err(ToRValErr::ConvertToRValFail)
            }
        }
        "&str" => Ok(RVal::Str(value.to_string())),
        _ => Err(ToRValErr::UnsupportedType)
    }
}
