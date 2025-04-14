pub mod tests;

use bevy::prelude::*;
use crate::event_writer::*;
use crate::utils::*;
use crate::widgets::*;

/// Marker component identifyijng Famiq Image widget.
#[derive(Component)]
pub struct IsFamiqImage;

pub struct FaImage;

impl<'a> FaImage {
    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();

        let image_entity = root_node
            .commands()
            .spawn((
                ImageNode::new(attributes.image_handle.clone().unwrap()),
                style_components.clone(),
                IsFamiqImage,
                MainWidget,
                DefaultWidgetConfig::from(style_components)
            ))
            .observe(FaImage::handle_on_mouse_over)
            .observe(FaImage::handle_on_mouse_out)
            .observe(FaImage::handle_on_mouse_down)
            .observe(FaImage::handle_on_mouse_up)
            .id();

        if attributes.has_tooltip {
            build_tooltip_node(attributes, root_node, image_entity);
        }
        insert_id_and_class(root_node, image_entity, &attributes.id, &attributes.class);
        image_entity
    }

    fn handle_on_mouse_over(
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

    fn handle_on_mouse_out(
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

    fn handle_on_mouse_down(
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

    fn handle_on_mouse_up(
        mut trigger: Trigger<Pointer<Up>>,
        mut writer: EventWriter<FaMouseEvent>,
        image_q: Query<Option<&WidgetId>, With<IsFamiqImage>>
    ) {
        if let Ok(id) = image_q.get(trigger.entity()) {
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Image, trigger.entity(), id);
        }
        trigger.propagate(false);
    }
}

/// Builder for creating image widget.
pub struct FaImageBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaImageBuilder<'a> {
    pub fn new(image_handle: Handle<Image>, font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.image_handle = Some(image_handle);
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            root_node
        }
    }

    /// set custom size for image
    pub fn set_size(mut self, size: (Val, Val)) -> Self {
        self.attributes.node.width = size.0;
        self.attributes.node.height = size.1;
        self
    }

    /// Spawn image into UI World.
    pub fn build(&mut self) -> Entity {
        self._node();
        FaImage::new(&self.attributes, &mut self.root_node)
    }
}

impl<'a> SetWidgetAttributes for FaImageBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaImageBuilder`
pub fn fa_image_builder<'a>(builder: &'a mut FamiqBuilder, path: &str) -> FaImageBuilder<'a> {
    let image_handle = builder.asset_server.load(path);
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaImageBuilder::new(
        image_handle,
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

#[macro_export]
macro_rules! fa_image {
    ( path: $path:expr $(, $key:ident : $value:tt )* $(,)? ) => {{
        let builder = builder_mut();
        let mut image = fa_image_builder(builder, $path);
        $(
            $crate::fa_image_attributes!(image, $key : $value);
        )*
        image.build()
    }};
}

#[macro_export]
macro_rules! fa_image_attributes {
    ($image:ident, tooltip: $tooltip:expr) => {{
        $image = $image.tooltip($tooltip);
    }};

    ($image:ident, set_size: $set_size:expr) => {{
        $image = $image.set_size($set_size);
    }};

    // common attributes
    ($image:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($image, $key : $value);
    }};
}

/// a system to check if Image internal system(s) can run.
///
/// True only if image widget is created.
pub fn can_run_image_systems(image_q: Query<&IsFamiqImage>) -> bool {
    !image_q.is_empty()
}
