use crate::widgets::{
    list_view::*,
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

/// Interaction Event: Hover, Press, Out (None)
#[derive(Event, Debug)]
pub struct FaInteractionEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub interaction: Interaction,
    pub widget: WidgetType
}

impl FaInteractionEvent {
    fn new(
        entity: Entity,
        widget_id: Option<String>,
        interaction: Interaction,
        widget: WidgetType,
    ) -> Self {
        Self {
            entity,
            widget_id,
            interaction,
            widget,
        }
    }

    pub fn send_event<T>(
        interaction_q: &mut Query<(Entity, &T, Option<&FamiqWidgetId>, &Interaction), Changed<Interaction>>,
        writer: &mut EventWriter<FaInteractionEvent>,
        widget: WidgetType,
    ) where
        T: Component,
    {
        for (entity, _, widget_id, interaction) in interaction_q {
            writer.send(FaInteractionEvent::new(
                entity,
                widget_id.map(|id| id.0.clone()), // return Option<String> or None
                *interaction,
                widget,
            ));
        }
    }

    /// true provided widget type is pressed
    pub fn is_pressed(&self, _type: WidgetType) -> bool {
        self.widget == _type && self.interaction == Interaction::Pressed
    }

    /// true provided widget type is hovered
    pub fn is_hovered(&self, _type: WidgetType) -> bool {
        self.widget == _type && self.interaction == Interaction::Hovered
    }

    /// true if nothing has happened
    pub fn is_left(&self, _type: WidgetType) -> bool {
        self.widget == _type && self.interaction == Interaction::None
    }
}

/// `fa_text_input` value change event
#[derive(Event, Debug)]
pub struct FaTextInputChangeEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub new_value: String
}

impl FaTextInputChangeEvent {
    pub fn new(entity: Entity, widget_id: Option<String>, new_value: String) -> Self {
        Self {
            entity,
            widget_id,
            new_value
        }
    }
}

/// `fa_selection` value change event
#[derive(Event, Debug)]
pub struct FaSelectionChangeEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub new_value: String
}

impl FaSelectionChangeEvent {
    pub fn new(entity: Entity, widget_id: Option<String>, new_value: String) -> Self {
        Self {
            entity,
            widget_id,
            new_value
        }
    }
}

pub fn text_input_value_change_system(
    text_input_q: Query<(Entity, Ref<TextInputValue>, Option<&FamiqWidgetId>)>,
    mut value_change_writer: EventWriter<FaTextInputChangeEvent>
) {
    for (entity, text_input_value, id) in text_input_q.iter() {
        if text_input_value.is_changed() && !text_input_value.is_added() {
            value_change_writer.send(
                FaTextInputChangeEvent::new(entity, id.map(|_id| _id.0.clone()), text_input_value.0.clone())
            );
        }
    }
}

pub fn selection_value_change_system(
    selection_q: Query<(Entity, Ref<SelectionValue>, Option<&FamiqWidgetId>)>,
    mut value_change_writer: EventWriter<FaSelectionChangeEvent>
) {
    for (entity, selection_value, id) in selection_q.iter() {
        if selection_value.is_changed() && !selection_value.is_added() {
            value_change_writer.send(
                FaSelectionChangeEvent::new(entity, id.map(|_id| _id.0.clone()), selection_value.0.clone())
            );
        }
    }
}

pub fn listview_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqListView, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::ListView);
}

// pub fn container_interaction_system(
//     mut interaction_q: Query<
//         (Entity, &IsFamiqContainer, Option<&FamiqWidgetId>, &Interaction),
//         Changed<Interaction>,
//     >,
//     mut writer: EventWriter<FaInteractionEvent>,
// ) {
//     FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Container);
// }
