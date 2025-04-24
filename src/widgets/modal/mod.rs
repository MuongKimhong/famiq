pub mod helper;
pub mod components;
pub mod systems;
pub mod tests;

pub(crate) use components::*;
pub(crate) use systems::*;

use macros::set_widget_attributes;
use crate::widgets::container::base_container::*;
use crate::widgets::*;
use crate::utils::*;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;
use helper::*;

/// Use to define show/hide state for modal
/// by id or entity.
#[derive(Resource, Default, Debug)]
pub struct FaModalState {
    pub entity_states: HashMap<Entity, bool>,
    pub state_changed: bool
}

impl FaModalState {
    fn _update_or_insert_entity(&mut self, entity: Entity, new_state: bool) {
        self.entity_states.entry(entity).or_insert(false);
        self.entity_states.insert(entity, new_state);
    }

    fn _hide_all(&mut self) {
        self.entity_states.values_mut().for_each(|v| *v = false);
    }

    /// Show modal by entity (Only one can be `true`)
    pub fn show_by_entity(&mut self, entity: Entity) {
        self._hide_all();
        self._update_or_insert_entity(entity, true);
        self.state_changed = true;
    }

    /// Hide modal by entity
    pub fn hide_by_entity(&mut self, entity: Entity) {
        self._update_or_insert_entity(entity, false);
        self.state_changed = true;
    }

    pub fn get_state_by_entity(&self, entity: Entity) -> Option<&bool> {
        self.entity_states.get(&entity)
    }
}

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ModalBuilder {
    pub all_reactive_keys: Vec<String>,
    pub children: Vec<Entity>,
    pub root_node: Entity,
    pub clear_bg: RVal
}

impl ModalBuilder {
    pub fn new(root_node: Entity) -> Self {
        Self {
            all_reactive_keys: Vec::new(),
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            children: Vec::new(),
            clear_bg: RVal::Bool(false),
            root_node
        }
    }

    pub(crate) fn set_clear_bg(&mut self, state: bool) {
        if state {
            self.cloned_attrs.overrided_background_color = Some(Color::NONE);
            self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        }
        else {
            self.cloned_attrs.color = WidgetColor::CustomSrgba((0.0, 0.0, 0.0, 0.8));
        }
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_modal_background_node();
        self.cloned_attrs.default_visibility = Visibility::Visible;

        match self.clear_bg.to_owned() {
            RVal::Bool(v) => self.set_clear_bg(v),
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);

                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::Bool(state) => self.set_clear_bg(*state),
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
    }
}

impl SetupWidget for ModalBuilder {
    fn components(&mut self) -> impl Bundle {
        (
            IsFamiqModal, MainWidget, IsFamiqContainableWidget,
            FocusPolicy::Block, GlobalZIndex(5), ReactiveWidget,
            AnimationProgress(0.0)
        )
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.prepare_attrs(r_data);
        let mut modal_bg = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let modal_bg_entity = modal_bg.build(r_data, commands);

        commands.entity(modal_bg_entity).add_children(&self.children).insert(self.components());
        commands.entity(self.root_node).add_child(modal_bg_entity);

        insert_class_id(
            commands,
            modal_bg_entity,
            &self.cloned_attrs.id,
            &self.cloned_attrs.class
        );
        insert_model(commands, modal_bg_entity, &self.cloned_attrs.model_key);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                modal_bg_entity,
                WidgetBuilder {
                    builder: BuilderType::Modal(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        modal_bg_entity
    }

    fn build_with_world(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Option<Entity> {
        self.prepare_attrs(r_data);
        let mut modal_bg = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let modal_bg_entity = modal_bg.build_with_world(r_data, world);

        world.entity_mut(modal_bg_entity.unwrap()).add_children(&self.children).insert(self.components());
        world.entity_mut(self.root_node).add_child(modal_bg_entity.unwrap());

        insert_class_id_world(
            world,
            modal_bg_entity.unwrap(),
            &self.cloned_attrs.id,
            &self.cloned_attrs.class
        );
        insert_model_world(world, modal_bg_entity.unwrap(), &self.cloned_attrs.model_key);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            modal_bg_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::Modal(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
        modal_bg_entity
    }
}

#[macro_export]
macro_rules! modal {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();

        #[allow(unused_mut)]
        let mut children_vec: Vec<Entity> = Vec::new();
        $(
            $crate::extract_children!(children_vec, $key : $value);
        )*

        let root_entity = famiq_builder.resource.root_node_entity.unwrap();
        let m_builder = &mut ModalBuilder::new(root_entity);

        $(
            $crate::modal_attributes!(m_builder, $key : $value);
        )*
        m_builder.children = children_vec.clone();
        let bg_entity = m_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        );
        famiq_builder.containable_children.insert(bg_entity, children_vec);
        bg_entity
    }};
}

#[macro_export]
macro_rules! modal_attributes {
    // skip children
    ($m_builder:ident, children: $children_vec:tt) => {{}};

    ($m_builder:ident, clear_bg: $clear_bg:expr) => {{
        match to_rval($clear_bg) {
            Ok(v) => $m_builder.clear_bg = v,
            Err(_) => panic!("\nclear_bg attribute accepts only boolean and reactive string\n")
        }
    }};
    ($m_builder:ident, model: $model:expr) => {{
        $m_builder.set_model($model);
    }};
    ($m_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($m_builder, $key : $value);
    }};
}

/// Determines if modal internal system(s) can run.
///
/// True only if there is a modal widget created.
pub fn can_run_modal_systems(modal_q: Query<&IsFamiqModal>) -> bool {
    !modal_q.is_empty()
}
