pub mod components;
pub mod styling;
pub mod systems;
pub mod tests;

use crate::resources::*;
use crate::utils::*;
use crate::widgets::*;
use bevy::ecs::system::EntityCommands;
use bevy::ui::FocusPolicy;
use bevy::prelude::*;

pub use components::*;
pub use styling::*;
pub use systems::*;

#[derive(Default)]
pub struct IsFamiqSelectionResource;
pub type FaSelectionResource = InputResource<IsFamiqSelectionResource>;

pub fn get_text_size(size: &WidgetSize) -> f32 {
    let size_small = 14.0;
    let size_normal = 18.0;
    let size_large = 22.0;

    let text_size = match size {
        WidgetSize::Small => size_small,
        WidgetSize::Large => size_large,
        _ => size_normal
    };
    text_size
}


pub struct FaSelection;

// Needs container
impl<'a> FaSelection {
    fn _build_container(root_node: &'a mut EntityCommands) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_selection_container_node();
        style_components.visibility = Visibility::Visible;

        root_node
            .commands()
            .spawn((style_components, IsFamiqSelectionContainer))
            .id()
    }

    fn _build_selector_placeholder(
        attributes: &WidgetAttributes,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let use_color = get_text_color(&attributes.color);
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };

        let entity = root_node
            .commands()
            .spawn((
                Text::new(placeholder),
                txt_font.clone(),
                TextColor(use_color),
                TextLayout::new_with_justify(JustifyText::Left),
                SelectorPlaceHolder,
                DefaultTextEntity::new(
                    Text::new(placeholder),
                    txt_font,
                    TextColor(use_color),
                    TextLayout::new_with_justify(JustifyText::Left)
                )
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    fn _build_selector_arrow_icon(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let use_color = get_text_color(&attributes.color);
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };

        root_node
            .commands()
            .spawn((
                Text::new("▼"),
                txt_font,
                TextColor(use_color),
                TextLayout::new_with_justify(JustifyText::Right),
                ArrowIcon
            ))
            .id()
    }

    fn _build_selector(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        placeholder: &str,
        placeholder_entity: Entity,
        arrow_icon_entity: Entity
    ) -> Entity {
        let selection_color = get_color(&attributes.color);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.border_color = BorderColor(selection_color);
        style_components.background_color = BackgroundColor(selection_color);
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));
        style_components.visibility = Visibility::Visible;

        let selector_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqSelectionSelector,
                DefaultWidgetEntity::from(style_components),
                Selection::new(placeholder.to_string()),
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity)
            ))
            .id();

        insert_id_and_class(root_node, selector_entity, &attributes.id, &attributes.class);
        selector_entity
    }

    fn _build_choices_panel(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
        selector_entity: Entity
    ) -> Entity {
        let mut choice_entities: Vec<Entity> = Vec::new();
        let mut all_choices = Vec::with_capacity(choices.len() + 1);
        all_choices.push("-/-".to_string());
        all_choices.extend_from_slice(choices);

        for choice in all_choices.iter() {
            let txt = Self::_build_choice_text(attributes, choice, root_node);
            let container = Self::_build_choice_container(root_node, txt, selector_entity);
            entity_add_child(root_node, txt, container);
            choice_entities.push(container);
        }
        let selection_color = get_color(&attributes.color);
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_selection_choices_panel_node();
        style_components.border_color = BorderColor(selection_color);
        style_components.background_color = BackgroundColor(selection_color);
        style_components.visibility = Visibility::Hidden;
        style_components.border_radius = BorderRadius::all(Val::Px(5.0));

        let panel = root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqSelectionChoicesPanel,
                FocusPolicy::Block,
                GlobalZIndex(2)
            ))
            .id();

        entity_add_children(root_node, &choice_entities, panel);
        panel
    }

    fn _build_choice_container(
        root_node: &'a mut EntityCommands,
        text_entity: Entity,
        selector_entity: Entity
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_choice_container_node();
        style_components.border_radius = BorderRadius::all(Val::Px(5.0));

        root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                SelectorEntity(selector_entity)
            ))
            .id()
    }

    fn _build_choice_text(
        attributes: &WidgetAttributes,
        choice: &str,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            ..default()
        };
        let use_color = get_text_color(&attributes.color);
        root_node
            .commands()
            .spawn((
                Text::new(choice),
                txt_font,
                TextColor(use_color),
                TextLayout::new_with_justify(JustifyText::Center),
                Visibility::Inherited,
            ))
            .id()
    }

    // return Entity of selector
    pub fn new(
        attributes: &WidgetAttributes,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
    ) -> Entity {
        let container = Self::_build_container(root_node);
        let placeholder_entity = Self::_build_selector_placeholder(
            attributes,
            placeholder,
            root_node,
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(attributes, root_node);
        let selector = Self::_build_selector(
            attributes,
            root_node,
            placeholder,
            placeholder_entity,
            arrow_icon_entity
        );
        let choices_panel = Self::_build_choices_panel(
            attributes,
            root_node,
            choices,
            selector
        );

        root_node.commands().entity(selector).insert(SelectionChoicesPanelEntity(choices_panel));
        entity_add_children(root_node, &vec![placeholder_entity, arrow_icon_entity], selector);
        entity_add_children(root_node, &vec![selector, choices_panel], container);

        selector
    }

    pub fn arrow_up(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
        if let Ok(mut text) = text_q.get_mut(arrow_entity) {
            text.0 = "▲".to_string()
        }
    }

    pub fn arrow_down(text_q: &mut Query<&mut Text, With<ArrowIcon>>, arrow_entity: Entity) {
        if let Ok(mut text) = text_q.get_mut(arrow_entity) {
            text.0 = "▼".to_string()
        }
    }

    pub fn show_choice_panel(
        panel_q: &mut Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,
        panel_entity: Entity
    ) {
        if let Ok(mut visibility) = panel_q.get_mut(panel_entity) {
            *visibility = Visibility::Visible;
        }
    }

    pub fn hide_choice_panel(
        panel_q: &mut Query<&mut Visibility, With<IsFamiqSelectionChoicesPanel>>,
        panel_entity: Entity
    ) {
        if let Ok(mut visibility) = panel_q.get_mut(panel_entity) {
            *visibility = Visibility::Hidden;
        }
    }
}

pub struct FaSelectionBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub placeholder: String,
    pub choices: Vec<String>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaSelectionBuilder<'a> {
    pub fn new(
        placeholder: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>
    ) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            placeholder,
            choices: Vec::new(),
            root_node
        }
    }

    pub fn choices<I>(mut self, choices: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.choices = choices.into_iter().map(Into::into).collect();
        self
    }

    pub fn build(&mut self) -> Entity {
        self._process_built_in_size_class();
        self._process_built_in_color_class();
        self._node();
        FaSelection::new(
            &self.attributes,
            self.placeholder.as_str(),
            &mut self.root_node,
            &self.choices
        )
    }
}

impl<'a> SetWidgetAttributes for FaSelectionBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_selector_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

pub fn fa_selection<'a>(
    builder: &'a mut FamiqBuilder,
    placeholder: &str
) -> FaSelectionBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaSelectionBuilder::new(
        placeholder.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionContainer>) -> bool {
    !selection_q.is_empty()
}
