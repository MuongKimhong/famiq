use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::resources::*;
use crate::widgets::style::*;
use crate::widgets::selection::*;
use crate::widgets::*;

pub type Subscriber = HashMap<Entity, WidgetBuilder>; // String is serialized fields

#[derive(Resource, Default, Debug)]
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

    pub fn to_string(&self) -> String {
        match self {
            RVal::None => "None".to_string(),
            RVal::Num(v) => v.to_string(),
            RVal::FNum(v) => v.to_string(),
            RVal::Str(v) => v.clone(),
            RVal::List(v) => format!("[{}]", v.join(", ")),
            RVal::Bool(v) => v.to_string(),
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
    pub builder: WidgetBuilder
}

impl UpdateReactiveSubscriberEvent {
    pub fn new(keys: Vec<String>, entity: Entity, builder: WidgetBuilder) -> Self {
        Self {
            keys,
            entity,
            builder
        }
    }
}

pub fn on_update_subscriber_event(
    mut events: EventReader<UpdateReactiveSubscriberEvent>,
    mut reactive_subscriber: ResMut<RSubscriber>
) {
    for e in events.read() {
        let mut keys = e.keys.clone();
        keys.sort();
        keys.dedup();

        keys.iter().for_each(|key| {
            if let Some(subscribers) = reactive_subscriber.data.get_mut(key) {
                subscribers.insert(e.entity, e.builder.clone());
            }
            else {
                let mut subscribers: Subscriber= HashMap::new();
                subscribers.insert(e.entity, e.builder.clone());
                reactive_subscriber.data.insert(key.to_string(), subscribers);
            }
        });
    }
}

pub(crate) fn detect_reactive_data_change(
    mut commands: Commands,
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
    styles: Res<StylesKeyValueResource>,
    r_widgets_q: Query<&ChildOf, With<ReactiveWidget>>,
    children_q: Query<&Children>,
) {
    if fa_query.reactive_data.is_changed() && !fa_query.reactive_data.is_added() {
        FamiqBuilder::new(&mut fa_query, &mut famiq_res).inject();

        // Entity - (index withtin its parent, builder)
        let mut to_remove_subscribers: HashMap<Entity, (usize, WidgetBuilder)> = HashMap::new();
        // Entity - parent's Entity
        let mut parent_map: HashMap<Entity, Entity> = HashMap::new();

        for key in fa_query.reactive_data.changed_keys.iter() {
            let subscribers = fa_query.reactive_subscriber.data.get_mut(key);
            if subscribers.is_none() {
                continue;
            }
            let subscribers = subscribers.unwrap();

            for (entity, widget_builder) in subscribers.iter() {
                if to_remove_subscribers.contains_key(entity) {
                    continue;
                }
                if let Ok(child_of) = r_widgets_q.get(*entity) {
                    let mut child_index = 0 as usize;
                    if let Ok(children) = children_q.get(child_of.parent()) {
                        for (i, child) in children.iter().enumerate() {
                            if child == *entity {
                                child_index = i;
                                continue;
                            }
                        }
                    }
                    to_remove_subscribers.insert(
                        *entity,
                        (child_index, widget_builder.to_owned())
                    );
                    parent_map.insert(*entity, child_of.parent());
                }
            }
            subscribers.retain(|k, _| !to_remove_subscribers.contains_key(k));
        }

        let r_data = fa_query.reactive_data.data.clone();
        let style_res = styles.values.clone();

        commands.queue(move |world: &mut World| {
            to_remove_subscribers.into_iter().for_each(|(entity, (child_index, widget_builder))| {
                match widget_builder.builder {
                    BuilderType::Button(mut builder) => {
                        let new = builder.build_with_world(&r_data, world).unwrap();
                        rebuild_none_containable(world, &parent_map, entity, new, child_index);
                    }
                    BuilderType::Text(mut builder) => {
                        let new = builder.build_with_world(&r_data, world).unwrap();
                        rebuild_none_containable(world, &parent_map, entity, new, child_index);
                    }
                    BuilderType::Checkbox(mut builder) => {
                        let new = builder.build_with_world(&r_data, world).unwrap();
                        rebuild_none_containable(world, &parent_map, entity, new, child_index);
                    }
                    BuilderType::Circular(mut builder) => {
                        let new = builder.build_with_world(&r_data, world);
                        rebuild_none_containable(world, &parent_map, entity, new.unwrap(), child_index);
                    }
                    BuilderType::ProgressBar(mut builder) => {
                        let new = builder.build_with_world(&r_data, world);
                        rebuild_none_containable(world, &parent_map, entity, new.unwrap(), child_index);
                    }
                    BuilderType::Fps(mut builder) => {
                        let new = builder.build_with_world(&r_data, world);
                        rebuild_none_containable(world, &parent_map, entity, new.unwrap(), child_index);
                    }
                    BuilderType::Image(mut builder) => {
                        let new = builder.build_with_world(&r_data, world);
                        rebuild_none_containable(world, &parent_map, entity, new.unwrap(), child_index);
                    }
                    BuilderType::Selection(mut builder) => {
                        let new = builder.build_with_world(&r_data, world).unwrap();

                        let mut ph_q = world.query::<(&mut Text, &SelectorEntity)>();
                        for (mut text, selector_entity) in ph_q.iter_mut(world) {
                            if selector_entity.0 == new {
                                if let Some(key) = builder.cloned_attrs.model_key.to_owned() {
                                    if let Some(r_value) = r_data.get(&key) {
                                        match r_value {
                                            RVal::Str(v) => text.0 = v.to_owned(),
                                            _ => {}
                                        }
                                    }
                                }
                                break;
                            }
                        }
                        rebuild_none_containable(world, &parent_map, entity, new, child_index);
                    }
                    BuilderType::Container(mut builder) => {
                        builder.children = world.resource::<ContainableChildren>().get_children(entity).unwrap();
                        let new = builder.build_with_world(&r_data, world);
                        rebuild_containable(world, &mut parent_map, entity, new.unwrap(), child_index);
                    },
                    BuilderType::Modal(mut builder) => {
                        builder.children = world.resource::<ContainableChildren>().get_children(entity).unwrap();
                        let new = builder.build_with_world(&r_data, world);
                        rebuild_containable(world, &mut parent_map, entity, new.unwrap(), child_index);
                    }
                }
            });
            reset_external_style(world, &style_res);
            reset_external_text_style(world, &style_res);
        });
        fa_query.reactive_data.changed_keys.clear();
    }
}

pub(crate) fn rebuild_none_containable(
    world: &mut World,
    parent_map: &HashMap<Entity, Entity>,
    old_entity: Entity,
    new_entity: Entity,
    index: usize
) {
    world
        .resource_mut::<ContainableChildren>()
        .update_child(*parent_map.get(&old_entity).unwrap(), new_entity, old_entity);

    world
        .entity_mut(*parent_map.get(&old_entity).unwrap())
        .insert_children(index, &[new_entity]);

    world.entity_mut(old_entity).despawn();
}

pub(crate) fn rebuild_containable(
    world: &mut World,
    parent_map: &mut HashMap<Entity, Entity>,
    old_entity: Entity,
    new_entity: Entity,
    index: usize
) {
    world.entity_mut(old_entity).despawn();
    world
        .resource_mut::<ContainableChildren>()
        .update_containable(old_entity, new_entity);

    world
        .resource_mut::<ContainableChildren>()
        .update_child(*parent_map.get(&old_entity).unwrap(), new_entity, old_entity);

    world
        .entity_mut(*parent_map.get(&old_entity).unwrap())
        .insert_children(index, &[new_entity]);

    parent_map.iter_mut().for_each(|(_, value)| {
        if *value == old_entity {
            *value = new_entity;
        }
    });
}

pub(crate) fn reset_external_style(
    world: &mut World,
    style_res: &HashMap<String, WidgetStyle>,
) {
    let mut style_q = world.query::<StyleQuery>();

    style_q.par_iter_mut(world).for_each(|mut style| {
        if style.id.is_none() && style.class.is_none() {
            return;
        }
        let mut empty_style = WidgetStyle::default();

        if let Some(id) = style.id {
            if let Some(external_style) = style_res.get(&id.0) {
                empty_style.update_from(external_style);
            }
        }
        if let Some(classes) = style.class {
            let class_split: Vec<&str> = classes.0.split_whitespace().collect();
            let mut formatted = String::with_capacity(64);

            for class_name in class_split.iter() {
                formatted.clear();
                formatted.push('.');
                formatted.push_str(class_name);
                if let Some(external_style) = style_res.get(&formatted) {
                    empty_style.merge_external(external_style);
                }
            }
        }
        apply_styles_from_external_json(
            &mut style.background_color,
            &mut style.border_color,
            &mut style.border_radius,
            &mut style.visibility,
            &mut style.z_index,
            &mut style.node,
            &mut style.box_shadow,
            &empty_style,
            &mut style.default_style
        );
    });
}

pub fn reset_external_text_style(
    world: &mut World,
    style_res: &HashMap<String, WidgetStyle>
) {
    let mut text_style_q = world.query::<(
        Option<&mut TextFont>,
        Option<&mut TextColor>,
        Option<&WidgetId>,
        Option<&WidgetClasses>,
        Option<&DefaultTextConfig>,
        Option<&DefaultTextSpanConfig>,
    )>();

    text_style_q.par_iter_mut(world).for_each(|(text_font, text_color, id, class, default_text, default_text_span)| {
        let mut empty_style = WidgetStyle::default();

        if let Some(id) = id {
            if let Some(external_style) = style_res.get(&id.0) {
                empty_style.update_from(external_style);
            }
        }
        if let Some(classes) = class {
            let class_split: Vec<&str> = classes.0.split_whitespace().collect();
            let mut formatted = String::with_capacity(64);

            for class_name in class_split.iter() {
                formatted.clear();
                formatted.push('.');
                formatted.push_str(class_name);
                if let Some(external_style) = style_res.get(&formatted) {
                    empty_style.merge_external(external_style);
                }
            }
        }
        apply_text_styles_from_external_json(
            &empty_style,
            default_text,
            default_text_span,
            text_font,
            text_color
        );
    });
}
