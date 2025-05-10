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

    /// mouse's left button is clicked
    pub fn is_mouse_left_down(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::DownLeft
    }

    /// mouse's right button is clicked
    pub fn is_mouse_right_down(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::DownRight
    }

    /// mouse's left or right button is released
    pub fn is_mouse_up(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Up
    }

    /// mouse's cursor is over a widget
    pub fn is_mouse_over(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Over
    }

    /// mouse's cursor has left widget
    pub fn is_mouse_out(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Out
    }

    /// mouse is scrolling
    pub fn is_mouse_scroll(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == EventType::Scroll
    }

    /// button with provided id is pressed
    pub fn is_button_pressed(&self, id: &str) -> bool {
        if self.id.is_none() {
            return false;
        }

        self.widget_type == WidgetType::Button &&
        self.event_type == EventType::DownLeft &&
        self.id.as_ref().unwrap().as_str() == id
    }

    /// a button is pressed.
    /// returns the button's ID if this is a left-click on a button.
    pub fn button_press(&self) -> Option<&String> {
        if self.widget_type == WidgetType::Button &&
            self.event_type == EventType::DownLeft &&
            self.id.is_some()
        {
            return self.id.as_ref();
        }
        None
    }

    pub(crate) fn send_event(
        writer: &mut EventWriter<FaMouseEvent>,
        event_type: EventType,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&WidgetId>
    ) {
        writer.write(FaMouseEvent {
            event_type,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
    }
}
