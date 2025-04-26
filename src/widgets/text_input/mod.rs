/// Suport single line only. Bugs are waiting for you somewhere.

pub mod styling;
pub mod systems;
pub mod components;
pub mod tests;
pub mod text_edit;
pub mod helper;
pub mod system_params;

use macros::set_widget_attributes;
use styling::*;
use system_params::*;
pub(crate) use systems::*;
pub(crate) use components::*;
pub(crate) use text_edit::*;

use crate::widgets::container::base_container::*;
use crate::event_writer::*;
use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::resources::*;
use crate::widgets::*;

use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::image::ImageSampler;
use bevy::asset::RenderAssetUsages;
use bevy::input::ButtonState;
use bevy::prelude::*;
use cosmic_text::{
    Attrs, Metrics, Buffer, Editor, Family, Edit, Shaping, Weight, Cursor, Selection, Action
};
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use arboard::Clipboard;


// TODO:
// 1. make text input reactive
// 2. clipboard support on wasm
// 3. on macos, use commands + c or v, instead of ctrls

#[derive(Event, Debug)]
pub struct RequestRedrawBuffer {
    pub input_entity: Entity
}

impl RequestRedrawBuffer {
    pub fn new(input_entity: Entity) -> Self {
        Self {
            input_entity
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct TextInputMaterial {
    #[uniform(1)]
    color: Vec4,
    #[texture(2)]
    #[sampler(3)]
    texture: Handle<Image>,
}

impl UiMaterial for TextInputMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/text_input.wgsl").into()
    }
}

#[set_widget_attributes]
pub struct TextInputBuilder {
    pub placeholder: String
}

impl TextInputBuilder {
    pub fn new(placeholder: String, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            attributes,
            placeholder,
            cloned_attrs: WidgetAttributes::default()
        }
    }
}

impl SetupWidget for TextInputBuilder {
    fn components(&mut self) -> impl Bundle {
        self._process_built_in_size_class();
        let placeholder_color = get_text_color(&self.cloned_attrs.color);
        let text_data = CosmicTextData {
            handle: self.cloned_attrs.font_handle.clone().unwrap(),
            size: get_text_size(&self.cloned_attrs.size),
            color: placeholder_color
        };
        (
            IsFamiqTextInput,
            MainWidget,
            CosmicDataColor::new(placeholder_color),
            CosmicData::default(),
            CursorBlinkTimer::default(),
            text_data.clone(),
            DefaultCosmicTextEntity {
                text_data
            }
        )
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.node = default_input_node();
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let reactive_keys = get_reactive_key(&self.placeholder);
        let parsed_placeholder = replace_reactive_keys(&self.placeholder, &reactive_keys, r_data);
        all_reactive_keys.extend_from_slice(&reactive_keys);

        let mut input = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let input_entity = input.build(r_data, commands);

        commands
            .entity(input_entity)
            .insert(self.components())
            .insert(FaTextEdit::new(&parsed_placeholder))
            .observe(on_mouse_up)
            .observe(on_mouse_down)
            .observe(on_mouse_over)
            .observe(on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, input_entity);
        }

        insert_class_id(commands, input_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);
        insert_model(commands, input_entity, &self.cloned_attrs.model_key);
        input_entity
    }

    fn rebuild(
        &mut self,
        _reactive_data: &HashMap<String, RVal>,
        _old_entity: Entity,
        _world: &mut World
    ) {}
}

#[macro_export]
macro_rules! text_input {
    ( placeholder: $placeholder:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let t_builder = &mut TextInputBuilder::new(
            $placeholder.to_string(),
            &famiq_builder.get_font_handle()
        );
        $(
            $crate::selection_attributes!(t_builder, $key : $value);
        )*
        t_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! text_input_attributes {
    ($t_builder:ident, model: $model:expr) => {{
        $t_builder.set_model($model);
    }};
    ($t_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($t_builder, $key : $value);
    }};
}
