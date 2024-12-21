pub mod helper;

use bevy::utils::HashMap;
use helper::*;

use crate::utils;
use crate::widgets::{DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetId};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use smol_str::SmolStr;

#[derive(Component)]
pub struct TextInput {
    pub text: String,
    pub placeholder: String,
    pub focused: bool,
}

impl TextInput {
    pub fn new(text: &str, placeholder: &str) -> Self {
        Self {
            text: text.to_string(),
            placeholder: placeholder.to_string(),
            focused: false,
        }
    }
}

#[derive(Resource, Debug)]
pub struct FaTextInputResource {
    pub inputs: HashMap<String, String>,
}

impl FaTextInputResource {
    pub fn update_or_insert(&mut self, id: String, new_value: String) {
        if let Some(old_value) = self.inputs.get_mut(&id) {
            *old_value = new_value;
        } else {
            self.inputs.insert(id, "".to_string());
        }
    }
}

#[derive(Component)]
pub struct IsFamiqTextInput;

pub enum TextInputVariant {
    Default,
    Outlined,
    Underlined,
}

pub enum TextInputSize {
    Small,
    Normal,
    Large,
}

pub struct FaTextInput;

impl<'a> FaTextInput {
    fn _build_placeholder(
        id: &str,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: &TextInputSize,
    ) -> Entity {
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: asset_server.load(utils::strip_assets_prefix(font_path).unwrap()),
            font_size: get_text_size(size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                FamiqWidgetId(format!("{id}_placeholder")),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
            ))
            .id()
    }

    fn _build_input(
        id: &str,
        root_node: &'a mut EntityCommands,
        variant: TextInputVariant,
    ) -> Entity {
        let mut border_width = outlined_border_width();
        let mut border_radius = outlined_border_radius();

        match variant {
            TextInputVariant::Underlined => {
                border_width = underlined_border_width();
                border_radius = underlined_border_radius();
            }
            _ => (),
        }
        let node = default_input_node(border_width);
        let border_color = BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922));
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                bg_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                FamiqWidgetId(id.to_string()),
                IsFamiqTextInput,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
            ))
            .id()
    }

    pub fn new(
        id: &str,
        ph: &str, // placeholder
        root_node: &'a mut EntityCommands,
        asset_server: &'a ResMut<'a, AssetServer>,
        font_path: &String,
        size: TextInputSize,
        variant: TextInputVariant,
    ) -> Entity {
        let input = Self::_build_input(id, root_node, variant);
        let ph = Self::_build_placeholder(id, ph, root_node, asset_server, font_path, &size);

        utils::entity_add_child(root_node, ph, input);
        input
    }
}

// // need container
// impl<'a> FaTextInput {
//     fn _build_text_input(
//         id: &str,
//         placeholder: &str,
//         root_node: &'a mut EntityCommands,
//         asset_server: &'a ResMut<'a, AssetServer>,
//         font_path: &String,
//         size: Option<TextInputSize>,
//         border_width: UiRect,
//         border_radius: BorderRadius,
//     ) -> Entity {
//         let input_bundle = default_text_input_bundle(border_width, border_radius, &size);
//         let text_bundle = create_text_input_value(placeholder, &size, asset_server, font_path);

//         let text_entity = root_node
//             .commands()
//             .spawn((
//                 text_bundle,
//                 FamiqWidgetId(format!("{id}_text_input_value")),
//                 DefaultTextBundle(create_text_input_value(
//                     placeholder,
//                     &size,
//                     asset_server,
//                     font_path,
//                 )),
//             ))
//             .id();

//         let input_entity = root_node
//             .commands()
//             .spawn((
//                 input_bundle.clone(),
//                 FamiqWidgetId(id.to_string()),
//                 TextInput::new("", placeholder),
//                 IsFamiqTextInput,
//                 DefaultWidgetBundle(input_bundle),
//             ))
//             .id();

//         utils::entity_add_child(root_node, text_entity, input_entity);
//         input_entity
//     }

//     pub fn fa_text_input(
//         id: &str,
//         placeholder: &str,
//         root_node: &'a mut EntityCommands,
//         asset_server: &'a ResMut<'a, AssetServer>,
//         font_path: &String,
//         size: Option<TextInputSize>,
//         variant: TextInputVariant,
//     ) -> Entity {
//         let mut border_width = outlined_border_width();
//         let mut border_radius = outlined_border_radius();

//         match variant {
//             TextInputVariant::Underlined => {
//                 border_width = underlined_border_width();
//                 border_radius = underlined_border_radius();
//             }
//             _ => (),
//         }

//         Self::_build_text_input(
//             id,
//             placeholder,
//             root_node,
//             asset_server,
//             font_path,
//             size,
//             border_width,
//             border_radius,
//         )
//     }

//     fn update_input_text_bundle_color(
//         children: &Children,
//         text_query: &mut Query<&mut Text>,
//         color: Color,
//     ) {
//         for &child in children.iter() {
//             if let Ok(mut text) = text_query.get_mut(child) {
//                 text.sections[0].style.color = color;
//             }
//         }
//     }

//     pub fn update_input_text_bundle_value(
//         children: &Children,
//         text_query: &mut Query<&mut Text>,
//         text_input: &Mut<TextInput>,
//     ) {
//         for &child in children.iter() {
//             if let Ok(mut text) = text_query.get_mut(child) {
//                 if text_input.text == "" {
//                     text.sections[0].value = text_input.placeholder.clone();
//                 } else {
//                     text.sections[0].value = text_input.text.clone();
//                 }
//             }
//         }
//     }

//     pub fn set_unfocused_all(
//         text_input_q: &mut Query<(&Children, &mut TextInput, &FamiqWidgetId)>,
//         text_q: &mut Query<&mut Text>,
//         input_resource: &mut ResMut<TextInputResource>,
//     ) {
//         for (children, mut input, id) in text_input_q.iter_mut() {
//             input.focused = false;
//             input_resource.update_or_insert(id.0.clone(), input.text.clone());
//             Self::update_input_text_bundle_color(&children, text_q, PLACEHOLDER_COLOR);
//         }
//     }

//     pub fn set_focus(
//         text_input_q: &mut Query<(&Children, &mut TextInput, &FamiqWidgetId)>,
//         text_input_entity: Entity,
//         text_q: &mut Query<&mut Text>,
//         input_resource: &mut ResMut<TextInputResource>,
//     ) {
//         if let Ok((children, mut input, id)) = text_input_q.get_mut(text_input_entity) {
//             input.focused = true;
//             input_resource.update_or_insert(id.0.clone(), input.text.clone());
//             Self::update_input_text_bundle_color(&children, text_q, TEXT_INPUT_VALUE_COLOR);
//         }
//     }

//     pub fn add_text(
//         text_input_q: &mut Query<(&Children, &mut TextInput, &FamiqWidgetId)>,
//         text_q: &mut Query<&mut Text>,
//         input: &SmolStr,
//         input_resource: &mut ResMut<TextInputResource>,
//     ) {
//         for (children, mut text_input, id) in text_input_q.iter_mut() {
//             if text_input.focused {
//                 text_input.text.push_str(input);
//                 input_resource.update_or_insert(id.0.clone(), text_input.text.clone());
//                 Self::update_input_text_bundle_value(&children, text_q, &text_input);
//                 break;
//             }
//         }
//     }

//     pub fn delete_text(
//         text_input_q: &mut Query<(&Children, &mut TextInput, &FamiqWidgetId)>,
//         text_q: &mut Query<&mut Text>,
//         input_resource: &mut ResMut<TextInputResource>,
//     ) {
//         for (children, mut text_input, id) in text_input_q.iter_mut() {
//             if text_input.focused {
//                 text_input.text.pop();
//                 input_resource.update_or_insert(id.0.clone(), text_input.text.clone());
//                 Self::update_input_text_bundle_value(&children, text_q, &text_input);
//                 break;
//             }
//         }
//     }
// }
