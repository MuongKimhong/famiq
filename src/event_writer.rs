use crate::widgets::{
    selection::*,
    text_input::*,
    *,
};

use bevy::ecs::event::EventWriter;
use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub enum MouseEventType {
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
    pub event_type: MouseEventType,
    pub widget_type: WidgetType,
    pub entity: Entity,
    pub id: Option<String>,
}

impl FaMouseEvent {
    pub fn new(entity: Entity, id: Option<String>, event_type: MouseEventType, widget_type: WidgetType) -> Self {
        Self {
            entity,
            id,
            event_type,
            widget_type
        }
    }

    pub fn is_mouse_left_down(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == MouseEventType::DownLeft
    }

    pub fn is_mouse_right_down(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == MouseEventType::DownRight
    }

    pub fn is_mouse_up(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == MouseEventType::Up
    }

    pub fn is_mouse_over(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == MouseEventType::Over
    }

    pub fn is_mouse_out(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == MouseEventType::Out
    }

    pub fn is_mouse_scroll(&self, widget_type: WidgetType) -> bool {
        self.widget_type == widget_type && self.event_type == MouseEventType::Scroll
    }

    pub(crate) fn send_down_event(
        writer: &mut EventWriter<FaMouseEvent>,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>,
        right: bool
    ) {
        let mut event_type = MouseEventType::DownLeft;

        if right {
            event_type = MouseEventType::DownRight;
        }

        writer.send(FaMouseEvent {
            event_type,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
    }

    pub(crate) fn send_up_event(
        writer: &mut EventWriter<FaMouseEvent>,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>
    ) {
        writer.send(FaMouseEvent {
            event_type: MouseEventType::Up,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
    }

    pub(crate) fn send_over_event(
        writer: &mut EventWriter<FaMouseEvent>,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>
    ) {
        writer.send(FaMouseEvent {
            event_type: MouseEventType::Over,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
    }

    pub(crate) fn send_out_event(
        writer: &mut EventWriter<FaMouseEvent>,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>
    ) {
        writer.send(FaMouseEvent {
            event_type: MouseEventType::Out,
            widget_type,
            entity,
            id: id.map(|_id| _id.0.clone())
        });
    }

    pub(crate) fn send_scroll_event(
        writer: &mut EventWriter<FaMouseEvent>,
        widget_type: WidgetType,
        entity: Entity,
        id: Option<&FamiqWidgetId>
    ) {
        writer.send(FaMouseEvent {
            event_type: MouseEventType::Scroll,
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

pub fn text_input_value_change_system(
    text_input_q: Query<(Entity, Ref<TextInputValue>, Option<&FamiqWidgetId>)>,
    mut change_writer: EventWriter<FaValueChangeEvent>
) {
    for (entity, text_input_value, id) in text_input_q.iter() {
        if text_input_value.is_changed() && !text_input_value.is_added() {
            change_writer.send(
                FaValueChangeEvent::new(entity, id.map(|_id| _id.0.clone()), text_input_value.0.clone(), Vec::new())
            );
        }
    }
}

pub fn selection_value_change_system(
    selection_q: Query<(Entity, Ref<SelectionValue>, Option<&FamiqWidgetId>)>,
    mut change_writer: EventWriter<FaValueChangeEvent>
) {
    for (entity, selection_value, id) in selection_q.iter() {
        if selection_value.is_changed() && !selection_value.is_added() {
            change_writer.send(
                FaValueChangeEvent::new(entity, id.map(|_id| _id.0.clone()), selection_value.0.clone(), Vec::new())
            );
        }
    }
}
