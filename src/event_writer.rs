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

    pub fn is_button_pressed(&self) -> bool {
        self.widget_type == WidgetType::Button && self.event_type == EventType::DownLeft
    }

    pub(crate) fn send_scroll_event(
        writer: &mut EventWriter<FaMouseEvent>,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>
    ) {
        writer.send(FaMouseEvent {
            event_type: EventType::Scroll,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
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

    /// widget new value, for `fa_text_input`, `fa_selection`.
    pub new_value: String,

    /// widget new values, for `fa_checkbox` only.
    pub new_values: Vec<String>
}

impl FaValueChangeEvent {
    pub fn new(entity: Entity, widget_id: Option<String>, new_value: String, new_values: Vec<String>) -> Self {
        Self {
            entity,
            widget_id,
            new_value,
            new_values
        }
    }
}
