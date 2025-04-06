/// All components required by all widgets

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::text_input::CosmicTextData;

#[derive(Component)]
pub struct FamiqToolTipText(pub String);

#[derive(Component, Deref, Debug)]
pub struct FamiqWidgetId(pub String);

#[derive(Component, Deref, Debug)]
pub struct FamiqWidgetClasses(pub String);

#[derive(Component)]
pub(crate) struct IsFamiqTooltip;

#[derive(Component)]
pub(crate) struct FamiqTooltipEntity(pub Entity);

#[derive(Component)]
pub struct IsFamiqMainWidget;

#[derive(Component)]
pub struct IsFamiqContainableWidget;

#[derive(Component, Debug)]
pub struct ReactiveKeys(pub Vec<String>);

#[derive(Component, Debug)]
pub struct ReactiveText(pub String);

/// Base styles components required by all widgets
#[derive(Bundle, Clone)]
pub struct BaseStyleComponents {
    pub node: Node,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub background_color: BackgroundColor,
    pub z_index: ZIndex,
    pub visibility: Visibility,
    pub interaction: Interaction,
    pub box_shadow: BoxShadow,
}

impl Default for BaseStyleComponents {
    fn default() -> Self {
        Self {
            node: Node::default(),
            border_color: BorderColor::default(),
            border_radius: BorderRadius::default(),
            background_color: BackgroundColor::default(),
            z_index: ZIndex::default(),
            visibility: Visibility::default(),
            interaction: Interaction::default(),
            box_shadow: BoxShadow {
                color: Color::NONE,
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(0.0),
                spread_radius: Val::Px(0.5),
                blur_radius: Val::Px(1.0)
            }
        }
    }
}

#[derive(Component)]
pub struct DefaultWidgetEntity {
    pub node: Node,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub background_color: BackgroundColor,
    pub z_index: ZIndex,
    pub visibility: Visibility,
    pub box_shadow: BoxShadow
}

impl From<BaseStyleComponents> for DefaultWidgetEntity {
    fn from(base: BaseStyleComponents) -> Self {
        DefaultWidgetEntity {
            node: base.node,
            border_color: base.border_color,
            border_radius: base.border_radius,
            background_color: base.background_color,
            z_index: base.z_index,
            visibility: base.visibility,
            box_shadow: base.box_shadow
        }
    }
}

#[derive(Component)]
pub struct DefaultTextEntity {
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}

impl DefaultTextEntity {
    pub fn new(
        text: Text,
        text_font: TextFont,
        text_color: TextColor,
        text_layout: TextLayout,
    ) -> Self {
        Self {
            text,
            text_font,
            text_color,
            text_layout,
        }
    }
}

#[derive(Component)]
pub struct DefaultCosmicTextEntity {
    pub text_data: CosmicTextData
}

impl DefaultCosmicTextEntity {
    pub fn new(text_data: CosmicTextData) -> Self {
        Self {
            text_data
        }
    }
}

#[derive(Component)]
pub struct DefaultTextSpanEntity {
    pub text: TextSpan,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

impl DefaultTextSpanEntity {
    pub fn new(
        text: TextSpan,
        text_font: TextFont,
        text_color: TextColor
    ) -> Self {
        Self {
            text,
            text_font,
            text_color,
        }
    }
}

#[derive(Component)]
pub struct IsFaWidgetRoot;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Component, PartialEq)]
pub struct WidgetStyle {
    pub color: Option<String>,     // for fa_text, fa_fps, Text color only
    pub font_size: Option<String>, // for fa_text, fa_fps, Text font_size only
    pub background_color: Option<String>,
    pub shadow_color: Option<String>,
    pub shadow_spread: Option<String>,
    pub shadow_blur: Option<String>,
    pub shadow_x_offset: Option<String>,
    pub shadow_y_offset: Option<String>,
    pub border_color: Option<String>,
    pub border_radius: Option<String>,
    pub visibility: Option<String>,
    pub z_index: Option<String>,
    pub display: Option<String>,
    pub position_type: Option<String>,
    pub overflow_x: Option<String>,
    pub overflow_y: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
    pub top: Option<String>,
    pub bottom: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub min_width: Option<String>,
    pub min_height: Option<String>,
    pub max_width: Option<String>,
    pub max_height: Option<String>,
    pub align_items: Option<String>,
    pub justify_items: Option<String>,
    pub align_self: Option<String>,
    pub justify_content: Option<String>,
    pub margin: Option<String>,
    pub padding: Option<String>,
    pub border: Option<String>,
    pub flex_direction: Option<String>,
    pub flex_wrap: Option<String>,
    pub flex_grow: Option<String>,
    pub flex_shrink: Option<String>,
    pub flex_basis: Option<String>,
    pub row_gap: Option<String>,
    pub column_gap: Option<String>,
    pub grid_auto_flow: Option<String>,
    pub margin_left: Option<String>,
    pub margin_right: Option<String>,
    pub margin_top: Option<String>,
    pub margin_bottom: Option<String>,
    pub padding_left: Option<String>,
    pub padding_right: Option<String>,
    pub padding_top: Option<String>,
    pub padding_bottom: Option<String>,
    pub border_left: Option<String>,
    pub border_right: Option<String>,
    pub border_top: Option<String>,
    pub border_bottom: Option<String>
}

impl WidgetStyle {
    // assign external to self no matter what
    pub fn from_external(&mut self, external: &WidgetStyle) {
        *self = external.clone();
    }

    // merge external into & overwrite fields in self if
    // - field in self is "null"
    // - field in both self & external are not "null"
    pub fn merge_external(&mut self, external: &WidgetStyle) -> bool {
        let mut has_changed = false;

        let mut self_map = serde_json::to_value(&mut *self).unwrap();
        let external_map = serde_json::to_value(external).unwrap();

        let merged_map = self_map.as_object_mut().unwrap();
        for (key, value) in external_map.as_object().unwrap() {

            let self_field = merged_map.get(key).unwrap();
            let external_field = external_map.get(key).unwrap();

            if (self_field.is_null() && !external_field.is_null()) ||
               (!self_field.is_null() && !external_field.is_null()) {
                    merged_map.insert(key.clone(), value.clone());
                    has_changed = true;
               }
        }

        *self = serde_json::from_value(serde_json::Value::Object(merged_map.clone())).unwrap();
        has_changed
    }

    // override fields self that are different from external fields
    pub fn update_from(&mut self, external: &WidgetStyle) -> bool {
        let mut has_changed = false;

        let self_json = serde_json::to_value(&mut *self).unwrap();
        let external_json = serde_json::to_value(external).unwrap();

        if let serde_json::Value::Object(mut self_map) = self_json {
            if let serde_json::Value::Object(external_map) = external_json {
                for (key, external_value) in external_map {
                    if self_map.get(&key) != Some(&external_value) {
                        // Update only if different
                        self_map.insert(key, external_value);
                        has_changed = true;
                    }
                }
            }
            *self = serde_json::from_value(serde_json::Value::Object(self_map)).unwrap();
        }

        has_changed
    }
}
