pub mod components;
pub mod helper;
pub mod tests;
pub mod systems;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use macros::*;
use crate::widgets::*;
use crate::widgets::container::base_container::*;
use crate::event_writer::*;
use crate::utils::*;

pub(crate) use components::*;
pub(crate) use systems::*;
use helper::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CircularMaterial {
    #[uniform(0)]
    u_color: Vec4,
    #[uniform(1)]
    u_time: Vec4
}

impl UiMaterial for CircularMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/circular.wgsl").into()
    }
}

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct CircularBuilder {
    pub size: RVal,
    pub all_reactive_keys: Vec<String>
}

impl CircularBuilder {
    pub fn new() -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            all_reactive_keys: Vec::new(),
            size: RVal::FNum(0.0)
        }
    }

    pub(crate) fn handle_size_val(&mut self, r_data: &HashMap<String, RVal>) {
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
    }

    pub(crate) fn prepare_attrs(&mut self, r_data: &HashMap<String, RVal>) {
        self.cloned_attrs = self.attributes.clone();
        self.handle_size_val(r_data);
        self._process_built_in_size_class();
        self.cloned_attrs.node = default_circular_node(&self.cloned_attrs.size);
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut self.all_reactive_keys);
    }
}

impl SetupWidget for CircularBuilder {
    fn components(&mut self) -> impl Bundle {
        let color = get_color(&self.cloned_attrs.color);
        (MainWidget, SpinnerColor(color), ReactiveWidget)
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        self.prepare_attrs(r_data);

        let mut circular = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let circular_entity = circular.build(r_data, commands);

        commands
            .entity(circular_entity)
            .insert(self.components())
            .insert(IsFamiqCircular(0))
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, circular_entity);
        }
        insert_class_id(commands, circular_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        commands.queue(move |w: &mut World| {
            w.send_event(UpdateReactiveSubscriberEvent::new(
                ar_keys,
                circular_entity,
                WidgetBuilder {
                    builder: BuilderType::Circular(cloned_builder)
                }
            ));
        });
        self.all_reactive_keys.clear();
        circular_entity
    }

    fn rebuild(&mut self, r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        self.prepare_attrs(r_data);

        let mut circular = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        circular.rebuild(r_data, old_entity, world);

        let mut query = world.query::<&mut IsFamiqCircular>();
        if let Ok(mut c) = query.get_mut(world, old_entity) {
            c.0 += 1;
        }
        world.entity_mut(old_entity).insert(self.components());
        insert_class_id_world(world, old_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let ar_keys = self.all_reactive_keys.clone();
        world.send_event(UpdateReactiveSubscriberEvent::new(
            ar_keys,
            old_entity,
            WidgetBuilder {
                builder: BuilderType::Circular(cloned_builder)
            }
        ));
        self.all_reactive_keys.clear();
    }
}

/// Macro for creating a spinning circular.
#[macro_export]
macro_rules! circular {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let c_builder = &mut CircularBuilder::new();
        $(
            $crate::circular_attributes!(c_builder, $key : $value);
        )*
        c_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! circular_attributes {
    ($c_builder:ident, size: $size:expr) => {{
        match to_rval($size) {
            Ok(v) => $c_builder.size = v,
            Err(_) => panic!("\nsize attribute accepts only f32 and reactive string\n")
        }
    }};
    ($c_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($c_builder, $key : $value);
    }};
}

/// Determines if circular internal system(s) can run.
///
/// True only if circular widget is created.
pub fn can_run_circular_systems(circular_q: Query<&IsFamiqCircular>) -> bool {
    !circular_q.is_empty()
}
