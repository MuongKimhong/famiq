use bevy::prelude::*;
use super::*;
use crate::widgets::*;

#[derive(Clone, Debug)]
pub struct FaBaseText {
    pub layout: TextLayout,
    pub use_get_color: bool,
    pub value: String,
    pub attributes: WidgetAttributes,
    pub cloned_attrs: WidgetAttributes
}

impl FaBaseText {
    pub fn new(value: &str, font_handle: &Handle<Font>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle.clone());
        Self {
            attributes,
            cloned_attrs: WidgetAttributes::default(),
            layout: TextLayout::new_with_justify(JustifyText::Center),
            value: value.to_string(),
            use_get_color: false
        }
    }

    pub fn new_with_attributes(value: &str, attributes: &WidgetAttributes) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            cloned_attrs: attributes.clone(),
            layout: TextLayout::new_with_justify(JustifyText::Center),
            value: value.to_string(),
            use_get_color: false
        }
    }

    fn process_text_size_class(&self) -> TextSize {
        let mut use_size = TextSize::Default;

        if let Some(class) = self.cloned_attrs.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "h1" => use_size = TextSize::TitleH1,
                    "h2" => use_size = TextSize::TitleH2,
                    "h3" => use_size = TextSize::TitleH3,
                    "h4" => use_size = TextSize::TitleH4,
                    "h5" => use_size = TextSize::TitleH5,
                    _ => {}
                }
            }
        }
        use_size
    }
}

impl SetWidgetAttributes for FaBaseText {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn cloned_attrs(&mut self) -> &mut WidgetAttributes {
        &mut self.cloned_attrs
    }
}

impl SetupWidget for FaBaseText {
    fn components(&mut self) -> impl Bundle {
        self._process_built_in_size_class();
        self._process_built_in_color_class();

        let text = Text::new(self.value.as_str());
        let mut text_font = TextFont {
            font: self.cloned_attrs.font_handle.clone().unwrap(),
            font_size: get_text_size(&self.cloned_attrs.size),
            ..default()
        };
        match self.process_text_size_class() {
            TextSize::TitleH1 => text_font.font_size = 40.0,
            TextSize::TitleH2 => text_font.font_size = 32.0,
            TextSize::TitleH3 => text_font.font_size = 28.0,
            TextSize::TitleH4 => text_font.font_size = 24.0,
            TextSize::TitleH5 => text_font.font_size = 20.0,
            TextSize::TitleH6 => text_font.font_size = 16.0,
            _ => {}
        }
        if let Some(overrided_size) = self.cloned_attrs.override_text_size {
            text_font.font_size = overrided_size;
        }
        let text_color = match self.use_get_color {
            true => TextColor(get_color(&self.cloned_attrs.color)),
            false => TextColor(get_text_color(&self.cloned_attrs.color))
        };
        let default_config = DefaultTextConfig::new_with_refs(
            &text,
            &text_font,
            &text_color,
            &self.layout
        );
        (text, text_font, text_color, self.layout, default_config, ReactiveWidget)
    }

    fn build(&mut self, _r_data: &HashMap<String, RVal>, commands: &mut Commands) -> Entity {
        commands.spawn(self.components()).id()
    }

    fn rebuild(&mut self, _r_data: &HashMap<String, RVal>, old_entity: Entity, world: &mut World) {
        world.entity_mut(old_entity).insert(self.components());
    }
}
