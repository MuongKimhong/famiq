pub mod helper;
pub mod components;
pub mod systems;
pub mod tests;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use macros::set_widget_attributes;

use crate::widgets::container::base_container::*;
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::*;

pub(crate) use components::*;
pub(crate) use systems::*;
use helper::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    u_time: Vec4,
    #[uniform(1)]
    u_color: Vec4,
    #[uniform(2)]
    u_blend: Vec4,
    #[uniform(3)]
    u_size: Vec4
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/progress_bar.wgsl").into()
    }
}

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ProgressBarBuilder {
    pub size: RVal,
    pub all_reactive_keys: Vec<String>,
    pub old_value_entity: Option<Entity>
}

impl ProgressBarBuilder {
    pub fn new() -> Self {
        Self {
            size: RVal::FNum(0.0),
            all_reactive_keys: Vec::new(),
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            old_value_entity: None
        }
    }

    pub fn build_progress_value(
        &mut self,
        bar_entity: Entity,
        commands: &mut Commands,
        r_data: &HashMap<String, RVal>
    ) -> Entity {
        let mut value = FaBaseContainer::new();
        value.cloned_attrs.node = default_progress_value_node(None);
        value.cloned_attrs.overrided_background_color = Some(Color::NONE);
        value.cloned_attrs.overrided_border_color = Some(Color::NONE);
        let value_entity = value.build(r_data, commands);

        commands
            .entity(value_entity)
            .insert((
                IsFamiqProgressValue,
                ProgressValueColor(get_color(&self.cloned_attrs.color)),
                ProgressBarEntity(bar_entity),
                ProgressValuePercentage(None)
            ));
        self.old_value_entity = Some(value_entity);
        value_entity
    }

    pub fn rebuild_progress_value(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) {
        let mut value = FaBaseContainer::new();
        value.cloned_attrs.node = default_progress_value_node(None);
        value.cloned_attrs.overrided_background_color = Some(Color::NONE);
        value.cloned_attrs.overrided_border_color = Some(Color::NONE);
        value.rebuild(r_data, self.old_value_entity.unwrap(), world);

        world
            .entity_mut(self.old_value_entity.unwrap())
            .insert(ProgressValueColor(get_color(&self.cloned_attrs.color)));
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        match self.size.to_owned() {
            RVal::FNum(v) => {
                if v > 0.0 {
                    self.cloned_attrs.size = WidgetSize::Custom(v);
                }
            }
            RVal::Str(v) => {
                let reactive_keys = get_reactive_key(&v);
                for key in reactive_keys.iter() {
                    if let Some(r_v) = r_data.get(key) {
                        match r_v {
                            RVal::FNum(size) => self.cloned_attrs.size = WidgetSize::Custom(*size),
                            _ => {}
                        }
                    }
                }
                self.all_reactive_keys.extend_from_slice(&reactive_keys);
            }
            _ => {}
        }
        self._process_built_in_size_class();
        self.cloned_attrs.node = default_progress_bar_node(&self.cloned_attrs.size);
        self.cloned_attrs.default_visibility = Visibility::Visible;
        self.cloned_attrs.overrided_border_color = Some(Color::srgba(0.6, 0.6, 0.6, 0.2));
        self.cloned_attrs.overrided_background_color = Some(Color::srgba(0.6, 0.6, 0.6, 0.2));
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
    }
}

impl SetupWidget for ProgressBarBuilder {
    fn components(&mut self) -> impl Bundle {
        (MainWidget, ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.prepare_attrs(r_data);
        let mut bar = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let bar_entity = bar.build(r_data, commands);
        let value_entity = self.build_progress_value(bar_entity, commands, r_data);

        commands
            .entity(bar_entity)
            .insert((self.components(), ProgressValueEntity(value_entity), IsFamiqProgressBar(0)))
            .add_child(value_entity)
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, bar_entity);
        }
        insert_class_id(commands, bar_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model(commands, bar_entity, &self.cloned_attrs.model_key);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                bar_entity,
                WidgetBuilder {
                    builder: BuilderType::ProgressBar(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        bar_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        self.prepare_attrs(r_data);
        let mut bar = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        bar.rebuild(r_data, old_entity, world);
        self.rebuild_progress_value(r_data, world);

        let mut query = world.query::<&mut IsFamiqProgressBar>();
        if let Ok(mut c) = query.get_mut(world, old_entity) {
            c.0 += 1;
        }
        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::ProgressBar(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

#[macro_export]
macro_rules! progress_bar {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let p_builder = &mut ProgressBarBuilder::new();
        $(
            $crate::progress_bar_attributes!(p_builder, $key : $value);
        )*
        p_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! progress_bar_attributes {
    ($p_builder:ident, size: $size:expr) => {{
        match to_rval($size) {
            Ok(v) => $p_builder.size = v,
            Err(_) => panic!("\nsize attribute accepts only f32 and reactive string\n")
        }
    }};
    ($p_builder:ident, model: $model:expr) => {{
        $p_builder.set_model($model);
    }};
    ($p_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($p_builder, $key : $value);
    }};
}

pub fn can_run_fa_progress_bar_systems(bar_q: Query<&IsFamiqProgressBar>) -> bool {
    !bar_q.is_empty()
}
