use bevy::prelude::*;
use crate::widgets::*;

pub struct FaBaseContainer {
    pub attributes: WidgetAttributes,
    pub cloned_attrs: WidgetAttributes
}

impl FaBaseContainer {
    pub fn new() -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: WidgetAttributes::default()
        }
    }

    pub fn new_with_attributes(attr: &WidgetAttributes) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: attr.clone()
        }
    }
}

impl SetWidgetAttributes for FaBaseContainer {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn cloned_attrs(&mut self) -> &mut WidgetAttributes {
        &mut self.cloned_attrs
    }
}

impl SetupWidget for FaBaseContainer {
    fn components(&mut self) -> impl Bundle {
        self._process_built_in_color_class();
        self._process_built_in_size_class();
        self._process_built_in_alignment_class();
        self._process_built_in_spacing_class();
        self._process_built_in_border_radius_class();
        let mut base_style = BaseStyleComponents::default();
        base_style.node = self.cloned_attrs.node.clone();
        base_style.border_radius = self.cloned_attrs.border_radius;
        base_style.visibility = self.cloned_attrs.default_visibility;
        base_style.z_index = self.cloned_attrs.default_z_index;

        if let Some(bg_color) = self.cloned_attrs.overrided_background_color {
            base_style.background_color = bg_color.into();
        } else {
            base_style.background_color = get_color(&self.cloned_attrs.color).into();
        }

        if let Some(bd_color) = self.cloned_attrs.overrided_border_color {
            base_style.border_color = bd_color.into();
        } else {
            base_style.border_color = get_color(&self.cloned_attrs.color).into();
        }

        (base_style.clone(), DefaultWidgetConfig::from(base_style))
    }

    fn build(&mut self, _r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        commands.spawn(self.components()).id()
    }

    fn rebuild(&mut self, _r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        world.entity_mut(old_entity).insert(self.components());
    }
}
