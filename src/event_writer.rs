use crate::prelude::IsFamiqContainer;
use crate::widgets::{
    button::*,
    list_view::*,
    selection::*,
    text::*,
    text_input::*,
    image::*,
    circular::*,
    progress_bar::*,
    fps::*,
    *,
};

use bevy::ecs::event::EventWriter;
use bevy::prelude::*;

/// Interaction Event: Hover, Press, Leave (None)
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

pub fn btn_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqButton, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Button);
}

pub fn fps_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqFPSTextLabel, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::FpsText);
}

pub fn image_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqImage, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Image);
}

pub fn text_input_interaction_and_change_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqTextInput, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    text_input_q: Query<(Entity, Ref<TextInputValue>, Option<&FamiqWidgetId>)>,
    mut interaction_writer: EventWriter<FaInteractionEvent>,
    mut value_change_writer: EventWriter<FaTextInputChangeEvent>
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut interaction_writer, WidgetType::TextInput);

    for (entity, text_input_value, id) in text_input_q.iter() {
        if text_input_value.is_changed() && !text_input_value.is_added() {
            value_change_writer.send(
                FaTextInputChangeEvent::new(entity, id.map(|_id| _id.0.clone()), text_input_value.0.clone())
            );
        }
    }
}

pub fn selection_interaction_and_change_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqSelectionSelector, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    selection_q: Query<(Entity, Ref<SelectionValue>, Option<&FamiqWidgetId>)>,
    mut interaction_writer: EventWriter<FaInteractionEvent>,
    mut value_change_writer: EventWriter<FaSelectionChangeEvent>
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut interaction_writer, WidgetType::Selection);

    for (entity, selection_value, id) in selection_q.iter() {
        if selection_value.is_changed() && !selection_value.is_added() {
            value_change_writer.send(
                FaSelectionChangeEvent::new(entity, id.map(|_id| _id.0.clone()), selection_value.0.clone())
            );
        }
    }
}

pub fn text_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqText, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Text);
}

pub fn circular_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqCircular, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Circular);
}

pub fn progress_bar_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqProgressBar, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::ProgressBar);
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

pub fn container_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqContainer, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Container);
}

pub fn listview_item_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqListViewItem, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::ListViewItem);
}
