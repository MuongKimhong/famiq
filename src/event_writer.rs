use crate::widgets::*;

use bevy::ecs::event::EventWriter;
use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub enum EventType {
    Over,
    DownLeft, // Left click
    DownRight, // Right click
    Up,
    Out,
    Scroll
}

/// Mouse events on widget. Over, Down, Up, Out (Leave)
#[derive(Event, Debug)]
pub struct FaMouseEvent {
    pub event_type: EventType,
    pub widget_type: WidgetType,
    pub entity: Entity,
    pub id: Option<String>,
}

impl FaMouseEvent {
    pub fn new(entity: Entity, id: Option<String>, event_type: EventType, widget_type: WidgetType) -> Self {
        Self {
            entity,
            id,
            event_type,
            widget_type
        }
    }

    pub fn is_mouse_left_down(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::DownLeft
    }

    pub fn is_mouse_right_down(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::DownRight
    }

    pub fn is_mouse_up(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Up
    }

    pub fn is_mouse_over(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Over
    }

    pub fn is_mouse_out(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Out
    }

    pub fn is_mouse_scroll(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Scroll
    }

    pub fn is_button_pressed(&self, id: &str) -> bool {
        if self.id.is_none() {
            return false;
        }

        self.widget_type == WidgetType::Button &&
        self.event_type == EventType::DownLeft &&
        self.id.as_ref().unwrap().as_str() == id
    }

    pub(crate) fn send_event(
        writer: &mut EventWriter<FaMouseEvent>,
        event_type: EventType,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>
    ) {
        writer.send(FaMouseEvent {
            event_type,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
    }
}

/// Value change event for `fa_text_input`, `fa_selection`
/// `fa_checkbox`.
#[derive(Event, Debug)]
pub struct FaValueChangeEvent {
    /// widget entity.
    pub entity: Entity,

    /// widget id.
    pub widget_id: Option<String>,

    pub model_key: String
}

impl FaValueChangeEvent {
    pub fn new(entity: Entity, widget_id: Option<String>, model_key: String) -> Self {
        Self {
            entity,
            widget_id,
            model_key
        }
    }
}

// pub enum RVal<T> {
//     List(Vec<T>),
//     Custom(T),
//     Num(f32),
//     Str(String)
// }
// use bevy::utils::HashMap;
// #[derive(Resource)]
// pub struct MyResource<T>(pub HashMap<String, RVal<T>>);
