pub mod tests;

use bevy::prelude::*;
use macros::set_widget_attributes;
use crate::widgets::container::base_container::*;
use crate::event_writer::*;
use crate::utils::*;
use crate::widgets::*;
use crate::widgets::style_parse::parse_val;

/// Marker component identifyijng Famiq Image widget.
#[derive(Component)]
pub struct IsFamiqImage;

#[set_widget_attributes]
#[derive(Clone, Debug)]
pub struct ImageBuilder {
    pub path: String,
    pub width: Option<String>,
    pub height: Option<String>
}

impl ImageBuilder {
    pub fn new(path: String) -> Self {
        Self {
            path,
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default(),
            width: None,
            height: None
        }
    }

    fn on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        image_q: Query<(Option<&WidgetId>, &GlobalTransform, Option<&TooltipEntity>), With<IsFamiqImage>>
    ) {
        if let Ok((id, transform, tooltip_entity)) = image_q.get(trigger.entity()) {
            show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Image, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut writer: EventWriter<FaMouseEvent>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        image_q: Query<(Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqImage>>
    ) {
        if let Ok((id, tooltip_entity)) = image_q.get(trigger.entity()) {
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Image, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    fn on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        mut writer: EventWriter<FaMouseEvent>,
        image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
    ) {
        if let Ok(id) = image_q.get(trigger.entity()) {
            if trigger.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Image, trigger.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Image, trigger.entity(), id);
            }
        }
        trigger.propagate(false);
    }

    fn on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut writer: EventWriter<FaMouseEvent>,
        image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
    ) {
        if let Ok(id) = image_q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Image, trigger.entity(), id);
        }
        trigger.propagate(false);
    }

    pub fn width_height(&mut self) {
        if let Some(w) = self.width.as_ref() {
            if let Some(parsed_width) = parse_val(&w) {
                self.cloned_attrs.node.width = parsed_width;
            }
        }
        if let Some(h) = self.height.as_ref() {
            if let Some(parsed_height) = parse_val(&h) {
                self.cloned_attrs.node.height = parsed_height;
            }
        }
    }
}

impl SetupWidget for ImageBuilder {
    fn components(&mut self) -> impl Bundle {
        ( IsFamiqImage, MainWidget, ReactiveWidget )
    }

    fn build(&mut self, r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        self.width_height();
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let reactive_keys = get_reactive_key(&self.path);
        let parsed_path = replace_reactive_keys(&self.path, &reactive_keys, r_data);
        all_reactive_keys.extend_from_slice(&reactive_keys);

        let mut image = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let image_entity = image.build(r_data, commands);

        commands
            .entity(image_entity)
            .insert(self.components())
            .observe(ImageBuilder::on_mouse_up)
            .observe(ImageBuilder::on_mouse_down)
            .observe(ImageBuilder::on_mouse_over)
            .observe(ImageBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, commands, image_entity);
        }
        insert_class_id(commands, image_entity, &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        commands.queue(move |w: &mut World| {
            let image_handle: Handle<Image> = w.resource::<AssetServer>().load(parsed_path);
            w.entity_mut(image_entity).insert(ImageNode::new(image_handle));
            w.send_event(UpdateReactiveSubscriberEvent::new(
                all_reactive_keys,
                image_entity,
                WidgetBuilder {
                    builder: BuilderType::Image(cloned_builder)
                }
            ));
        });
        image_entity
    }

    fn build_with_world(
        &mut self,
        r_data: &HashMap<String, RVal>,
        world: &mut World
    ) -> Option<Entity> {
        let mut all_reactive_keys: Vec<String> = Vec::new();
        self.cloned_attrs = self.attributes.clone();
        self.cloned_attrs.overrided_border_color = Some(Color::NONE);
        self.cloned_attrs.overrided_background_color = Some(Color::NONE);
        self.width_height();
        replace_reactive_keys_common_attrs(&mut self.cloned_attrs, r_data, &mut all_reactive_keys);

        let reactive_keys = get_reactive_key(&self.path);
        let parsed_path = replace_reactive_keys(&self.path, &reactive_keys, r_data);
        all_reactive_keys.extend_from_slice(&reactive_keys);

        let mut image = FaBaseContainer::new_with_attributes(&self.cloned_attrs);
        let image_entity = image.build_with_world(r_data, world);

        world
            .entity_mut(image_entity.unwrap())
            .insert(self.components())
            .observe(ImageBuilder::on_mouse_up)
            .observe(ImageBuilder::on_mouse_down)
            .observe(ImageBuilder::on_mouse_over)
            .observe(ImageBuilder::on_mouse_out);

        if self.attributes.has_tooltip {
            build_tooltip_node(&self.cloned_attrs, &mut world.commands(), image_entity.unwrap());
        }
        insert_class_id_world(world, image_entity.unwrap(), &self.cloned_attrs.id, &self.cloned_attrs.class);

        let cloned_builder = self.clone();
        let image_handle: Handle<Image> = world.resource::<AssetServer>().load(parsed_path);
        world.entity_mut(image_entity.unwrap()).insert(ImageNode::new(image_handle));
        world.send_event(UpdateReactiveSubscriberEvent::new(
            all_reactive_keys,
            image_entity.unwrap(),
            WidgetBuilder {
                builder: BuilderType::Image(cloned_builder)
            }
        ));
        image_entity
    }
}

#[macro_export]
macro_rules! image {
    ( path: $path:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let famiq_builder = builder_mut();
        let i_builder = &mut ImageBuilder::new($path.to_string());
        $(
            $crate::image_attributes!(i_builder, $key : $value);
        )*
        i_builder.build(
            &famiq_builder.reactive_data.data,
            &mut famiq_builder.ui_root_node.commands()
        )
    }};
}

#[macro_export]
macro_rules! image_attributes {
    ($i_builder:ident, width: $width:expr) => {{
        $i_builder.width = Some($width.to_string());
    }};

    ($i_builder:ident, height: $height:expr) => {{
        $i_builder.height = Some($height.to_string());
    }};

    ($i_builder:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($i_builder, $key : $value);
    }};
}


// pub struct FaImage;

// impl<'a> FaImage {
//     pub fn new(
//         attributes: &WidgetAttributes,
//         root_node: &'a mut EntityCommands,
//     ) -> Entity {
//         let mut style_components = BaseStyleComponents::default();
//         style_components.node = attributes.node.clone();

//         let image_entity = root_node
//             .commands()
//             .spawn((
//                 ImageNode::new(attributes.image_handle.clone().unwrap()),
//                 style_components.clone(),
//                 IsFamiqImage,
//                 MainWidget,
//                 DefaultWidgetConfig::from(style_components)
//             ))
//             .observe(FaImage::handle_on_mouse_over)
//             .observe(FaImage::handle_on_mouse_out)
//             .observe(FaImage::handle_on_mouse_down)
//             .observe(FaImage::handle_on_mouse_up)
//             .id();

//         if attributes.has_tooltip {
//             build_tooltip_node(attributes, root_node, image_entity);
//         }
//         insert_id_and_class(root_node, image_entity, &attributes.id, &attributes.class);
//         image_entity
//     }

//     fn handle_on_mouse_over(
//         mut trigger: Trigger<Pointer<Over>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         image_q: Query<(Option<&WidgetId>, &GlobalTransform, Option<&TooltipEntity>), With<IsFamiqImage>>
//     ) {
//         if let Ok((id, transform, tooltip_entity)) = image_q.get(trigger.entity()) {
//             show_tooltip(tooltip_entity, &mut tooltip_q, transform.translation());
//             FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Image, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_out(
//         mut trigger: Trigger<Pointer<Out>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
//         image_q: Query<(Option<&WidgetId>, Option<&TooltipEntity>), With<IsFamiqImage>>
//     ) {
//         if let Ok((id, tooltip_entity)) = image_q.get(trigger.entity()) {
//             hide_tooltip(tooltip_entity, &mut tooltip_q);
//             FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Image, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_down(
//         mut trigger: Trigger<Pointer<Down>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
//     ) {
//         if let Ok(id) = image_q.get(trigger.entity()) {
//             if trigger.event().button == PointerButton::Secondary {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Image, trigger.entity(), id);
//             } else {
//                 FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Image, trigger.entity(), id);
//             }
//         }
//         trigger.propagate(false);
//     }

//     fn handle_on_mouse_up(
//         mut trigger: Trigger<Pointer<Up>>,
//         mut writer: EventWriter<FaMouseEvent>,
//         image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
//     ) {
//         if let Ok(id) = image_q.get(trigger.entity()) {
//             FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Image, trigger.entity(), id);
//         }
//         trigger.propagate(false);
//     }
// }

// /// Builder for creating image widget.
// pub struct FaImageBuilder<'a> {
//     pub attributes: WidgetAttributes,
//     pub root_node: EntityCommands<'a>
// }

// impl<'a> FaImageBuilder<'a> {
//     pub fn new(image_handle: Handle<Image>, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
//         let mut attributes = WidgetAttributes::default();
//         attributes.image_handle = Some(image_handle);
//         attributes.font_handle = Some(font_handle);
//         Self {
//             attributes,
//             root_node
//         }
//     }

//     /// set custom size for image
//     pub fn set_size(mut self, size: (Val, Val)) -> Self {
//         self.attributes.node.width = size.0;
//         self.attributes.node.height = size.1;
//         self
//     }

//     /// Spawn image into UI World.
//     pub fn build(&mut self) -> Entity {
//         self._node();
//         FaImage::new(&self.attributes, &mut self.root_node)
//     }
// }

// impl<'a> SetWidgetAttributes for FaImageBuilder<'a> {
//     fn attributes(&mut self) -> &mut WidgetAttributes {
//         &mut self.attributes
//     }

//     fn _node(&mut self) {
//         if self.attributes.default_display_changed {
//             self.attributes.node.display = self.attributes.default_display;
//         }
//         process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
//     }
// }

// /// API to create `FaImageBuilder`
// pub fn fa_image_builder<'a>(builder: &'a mut FamiqBuilder, path: &str) -> FaImageBuilder<'a> {
//     let image_handle = builder.asset_server.load(path);
//     let font_handle = builder.asset_server.load(&builder.resource.font_path);
//     FaImageBuilder::new(
//         image_handle,
//         font_handle,
//         builder.ui_root_node.reborrow()
//     )
// }

// #[macro_export]
// macro_rules! fa_image {
//     ( path: $path:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
//         let builder = builder_mut();
//         let mut image = fa_image_builder(builder, $path);
//         $(
//             $crate::fa_image_attributes!(image, $key : $value);
//         )*
//         image.build()
//     }};
// }

// #[macro_export]
// macro_rules! fa_image_attributes {
//     ($image:ident, tooltip: $tooltip:expr) => {{
//         $image = $image.tooltip($tooltip);
//     }};

//     ($image:ident, set_size: $set_size:expr) => {{
//         $image = $image.set_size($set_size);
//     }};

//     // common attributes
//     ($image:ident, $key:ident : $value:expr) => {{
//         $crate::common_attributes!($image, $key : $value);
//     }};
// }

/// a system to check if Image internal system(s) can run.
///
/// True only if image widget is created.
pub fn can_run_image_systems(image_q: Query<&IsFamiqImage>) -> bool {
    !image_q.is_empty()
}
