use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::resources::*;
use crate::widgets::text::*;
use crate::utils::*;
use crate::widgets::*;

pub type Subscriber = HashMap<Entity, String>; // String is serialized fields

#[derive(Resource, Debug, Default)]
pub struct RSubscriber {
    pub data: HashMap<String, Subscriber> // String is Reactivy data key
}

#[derive(Debug, Default, Clone)]
pub enum RVal {
    #[default]
    None,
    Num(i32),
    FNum(f32),
    Str(String),
    List(Vec<String>),
    Bool(bool)
}

impl RVal {
    /// Get inner value of Num as i32.
    pub fn as_num(&self) -> i32 {
        match self {
            RVal::Num(v) => *v,
            _ => panic!("\n[FamiqError]: calling as_num() on none RVal::Num\n")
        }
    }

    /// Get inner value of FNum as f32.
    pub fn as_f_num(&self) -> f32 {
        match self {
            RVal::FNum(v) => *v,
            _ => panic!("\n[FamiqError]: calling as_f_num() on none RVal::FNum\n"),
        }
    }

    /// Get inner value of Str as &str.
    pub fn as_str(&self) -> &str {
        match self {
            RVal::Str(v) => v.as_str(),
            _ => panic!("\n[FamiqError]: calling as_str() on none RVal::Str\n"),
        }
    }

    /// Get inner value of List as Vec<String>
    pub fn as_vec(&self) -> &Vec<String> {
        match self {
            RVal::List(v) => v,
            _ => panic!("\n[FamiqError]: calling as_vec() on none RVal::List\n")
        }
    }

    /// Get inner value of Bool as bool
    pub fn as_bool(&self) -> bool {
        match self {
            RVal::Bool(v) => *v,
            _ => panic!("\n[FamiqError]: calling as_bool() on none RVal::Bool\n")
        }
    }
}

/// Reactive data
#[derive(Resource, Debug, Default)]
pub struct RData {
    pub data: HashMap<String, RVal>,
    pub changed_keys: Vec<String>,
}

impl RData {
    pub fn type_match(old_val: &RVal, new_val: &RVal) -> bool {
        match (old_val, new_val) {
            (RVal::Num(_), RVal::Num(_)) => true,
            (RVal::FNum(_), RVal::FNum(_)) => true,
            (RVal::Str(_), RVal::Str(_)) => true,
            _ => false
        }
    }
}

#[derive(Event, Debug)]
pub struct UpdateReactiveSubscriberEvent {
    pub keys: Vec<String>,
    pub entity: Entity,
    pub serialized_fields: String
}

impl<'a> UpdateReactiveSubscriberEvent {
    pub fn new(keys: Vec<String>, entity: Entity, serialized_fields: String) -> Self {
        Self {
            keys,
            entity,
            serialized_fields
        }
    }
}

pub fn on_update_subscriber_event(
    mut events: EventReader<UpdateReactiveSubscriberEvent>,
    mut reactive_subscriber: ResMut<RSubscriber>
) {
    for e in events.read() {
        for key in e.keys.iter() {
            if let Some(subscribers) = reactive_subscriber.data.get_mut(key) {
                if let Some(fields) = subscribers.get_mut(&e.entity) {
                    *fields = e.serialized_fields.clone();
                }
                else {
                    subscribers.insert(e.entity, e.serialized_fields.clone());
                }
            }
            else {
                let mut subscribers: Subscriber = HashMap::new();
                subscribers.insert(e.entity, e.serialized_fields.clone());
                reactive_subscriber.data.insert(key.to_string(), subscribers);
            }
        }
    }
}

pub(crate) fn detect_reactive_data_change(
    mut commands: Commands,
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    styles: Res<StylesKeyValueResource>,
    r_widgets_q: Query<
        (&Parent, &WidgetType, Option<&WidgetId>, Option<&WidgetClasses>),
        With<ReactiveWidget>
    >,
    children_q: Query<&Children>,
) {
    if fa_query.reactive_data.is_changed() && !fa_query.reactive_data.is_added() {
        let mut all_style_keys: Vec<String> = Vec::new();
        let mut bd = FamiqBuilder::new(&mut fa_query, &mut famiq_res).hot_reload();
        inject_builder(&mut bd);

        use std::time::Instant;
        let now = Instant::now();
        for key in fa_query.reactive_data.changed_keys.iter() {
            if let Some(subscribers) = fa_query.reactive_subscriber.data.get(key) {

                for (entity, serialized_fields) in subscribers {
                    if let Ok((parent, widget_type, widget_id, widget_classes)) = r_widgets_q.get(*entity) {
                        match widget_type {
                            WidgetType::Text => {
                                let field: FaTextFields = serde_json::from_str(serialized_fields).unwrap();
                                let template_id = &field.common.id.clone().unwrap();
                                let template_class = &field.common.class.clone().unwrap();

                                let new = crate::test_text!(
                                    text: field.text.as_str(),
                                    id: template_id,
                                    class: template_class
                                );
                                let mut child_index = 0 as usize;

                                if let Ok(children) = children_q.get(parent.get()) {
                                    for (i, child) in children.iter().enumerate() {
                                        if child == entity {
                                            child_index = i;
                                            break;
                                        }
                                    }
                                }
                                commands
                                    .entity(parent.get())
                                    .insert_children(child_index, &[new]);

                                commands.entity(*entity).despawn();
                            }
                            _ => {}
                        }

                        if let Some(id) = widget_id {
                            if !all_style_keys.contains(&id.0) {
                                all_style_keys.push(id.0.clone());
                            }
                        }
                        if let Some(classes) = widget_classes {
                            let _class_split: Vec<&str> = classes.0.split_whitespace().collect();

                            for class_name in &_class_split {
                                let formatted = format!(".{class_name}");
                                if !all_style_keys.contains(&formatted) {
                                    all_style_keys.push(formatted);
                                }
                            }
                        }
                    }
                }
            }
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
