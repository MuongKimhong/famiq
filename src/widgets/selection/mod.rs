pub mod components;
pub mod helper;
pub mod styling;
pub mod systems;

use crate::utils;
use crate::widgets::{
    DefaultTextEntity, DefaultWidgetEntity, FamiqWidgetResource,
    FamiqWidgetId, FamiqWidgetClasses, FamiqWidgetBuilder, WidgetStyle,
    ExternalStyleHasChanged
};
use bevy::ecs::system::EntityCommands;
use bevy::ui::FocusPolicy;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use components::*;
pub use helper::*;

use super::color::{BLACK_COLOR, WHITE_COLOR};
pub use styling::*;
pub use systems::*;

#[derive(Resource, Debug)]
pub struct SelectedChoicesResource {
    pub choices: HashMap<String, String>, // id - choice
}

impl SelectedChoicesResource {
    pub fn update_or_insert(&mut self, id: String, selected_choice: String) {
        if let Some(item_value) = self.choices.get_mut(&id) {
            *item_value = selected_choice;
        } else {
            self.choices.insert(id, selected_choice);
        }
    }
}

pub enum SelectorVariant {
    Outlined,
    Default,
    Underlined,
}

#[derive(PartialEq)]
pub enum SelectorColor {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

pub enum SelectorShape {
    Default,
    Round,
    Rectangle
}

pub enum SelectionSize {
    Small,
    Normal,
    Large,
}

pub struct FaSelection;

// Needs container
impl<'a> FaSelection {
    fn _build_container(
        id: &Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let node = default_selection_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqSelectionContainer,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(id.to_string()));
        }
        if let Some(class) = class {
            root_node.commands().entity(entity).insert(FamiqWidgetClasses(class));
        }
        entity
    }

    fn _build_selector_placeholder(
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &SelectionSize,
    ) -> Entity {
        let txt = Text::new(placeholder);
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR_UNFOCUSED);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Left);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                // FamiqWidgetId(format!("{id}_selection_placeholder")),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
            ))
            .id()
    }

    fn _build_selector_arrow_icon(
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        size: &SelectionSize,
    ) -> Entity {
        let txt = Text::new("▼");
        let txt_font = TextFont {
            font: font_handle,
            font_size: get_text_size(&size),
            ..default()
        };
        let txt_color = TextColor(PLACEHOLDER_COLOR_UNFOCUSED);
        let txt_layout = TextLayout::new_with_justify(JustifyText::Right);

        root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                ArrowIcon
            ))
            .id()
    }

    fn _build_selector(
        id: &Option<String>,
        root_node: &'a mut EntityCommands,
        variant: &SelectorVariant,
        shape: &SelectorShape,
        color: &SelectorColor,
        placeholder: &str,
        placeholder_entity: Entity,
        arrow_icon_entity: Entity,
        choices_panel_entity: Entity
    ) -> Entity {
        let mut border_width = UiRect::all(Val::Px(2.0));
        let mut border_radius = outlined_border_radius();

        match shape {
            SelectorShape::Round => border_radius = round_border_radius(),
            SelectorShape::Rectangle => border_radius = rectangle_border_radius(),
            _ => ()
        }
        match variant {
            SelectorVariant::Underlined => {
                border_radius = underlined_border_radius();
                border_width = UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(2.0),
                }
            }
            _ => ()
        }
        let node = default_selector_node(border_width);
        let border_color = get_selector_border_color(color);
        let bg_color = get_selector_background_color(color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let selector_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                BoxShadow {
                    color: Color::NONE,
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(0.5),
                    blur_radius: Val::Px(1.0)
                },
                IsFamiqSelectionSelector,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                Selection::new(placeholder.to_string()),
                SelectorPlaceHolderEntity(placeholder_entity),
                SelectorArrowIconEntity(arrow_icon_entity),
                SelectionChoicesPanelEntity(choices_panel_entity)
            ))
            .insert((WidgetStyle::default(), ExternalStyleHasChanged(false)))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(selector_entity).insert(FamiqWidgetId(format!("{id}_selection_selector")));
        }
        selector_entity
    }

    fn _build_choices_panel(
        id: &Option<String>,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
        font_handle: Handle<Font>,
        container_entity: Entity,
        color: &SelectorColor
    ) -> Entity {
        let node = default_selection_choices_panel_node();
        let border_radius = BorderRadius::all(Val::Px(5.0));
        let border_color = get_choice_panel_border_color(color);
        let bg_color = get_choice_panel_background_color(color);
        let z_index = ZIndex::default();
        let visibility = Visibility::Hidden;

        let mut choice_entities: Vec<Entity> = Vec::new();
        let mut all_choices = Vec::with_capacity(choices.len() + 1);
        all_choices.push("-/-".to_string());
        all_choices.extend_from_slice(choices);

        for choice in all_choices.iter() {
            let txt = Self::_build_choice_text(id, choice, root_node, &font_handle, color);
            let container = Self::_build_choice_container(id, root_node, txt);
            utils::entity_add_child(root_node, txt, container);
            choice_entities.push(container);
        }

        let panel = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqSelectionChoicesPanel,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                SelectionContainerEntity(container_entity),
                GlobalZIndex(2),
                FocusPolicy::Block
            ))
            .id();
        if let Some(id) = id {
            root_node.commands().entity(panel).insert(FamiqWidgetId(format!("{id}_selection_choice_panel")));
        }
        utils::entity_add_children(root_node, &choice_entities, panel);
        panel
    }

    fn _build_choice_container(id: &Option<String>, root_node: &'a mut EntityCommands, text_entity: Entity) -> Entity {
        let node = default_choice_container_node();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::all(Val::Px(5.0));
        let bg_color = BackgroundColor::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        let entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                IsFamiqSelectionChoice,
                SelectionChoiceTextEntity(text_entity),
                Interaction::default(),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(format!("{id}_selection_choice_container")));
        }
        entity
    }

    fn _build_choice_text(
        id: &Option<String>,
        choice: &str,
        root_node: &'a mut EntityCommands,
        font_handle: &Handle<Font>,
        color: &SelectorColor
    ) -> Entity {
        let txt = Text::new(choice);
        let txt_font = TextFont {
            font: font_handle.clone(),
            ..default()
        };
        let mut txt_color = TextColor(WHITE_COLOR);
        if *color == SelectorColor::Default {
            txt_color = TextColor(BLACK_COLOR);
        }
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                Visibility::Inherited,
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(entity).insert(FamiqWidgetId(format!("{id}_selection_choice_text")));
        }
        entity
    }

    // return Entity of Container (Selection refers to container itself)
    pub fn new(
        id: Option<String>,
        class: Option<String>,
        placeholder: &str,
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>,
        variant: SelectorVariant,
        color: SelectorColor,
        size: SelectionSize,
        shape: SelectorShape,
        choices: &Vec<String>,
    ) -> Entity {
        let container = Self::_build_container(&id, class, root_node);

        let choices_panel = Self::_build_choices_panel(
            &id,
            root_node,
            choices,
            font_handle.clone(),
            container,
            &color
        );

        let placeholder_entity = Self::_build_selector_placeholder(
            placeholder,
            root_node,
            font_handle.clone(),
            &size,
        );
        let arrow_icon_entity = Self::_build_selector_arrow_icon(root_node, font_handle.clone(), &size);
        let selector = Self::_build_selector(
            &id,
            root_node,
            &variant,
            &shape,
            &color,
            placeholder,
            placeholder_entity,
            arrow_icon_entity,
            choices_panel,
        );
        utils::entity_add_children(root_node, &vec![placeholder_entity, arrow_icon_entity], selector);
        utils::entity_add_children(root_node, &vec![selector, choices_panel], container);

        container
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
        panel_q: &mut Query<
            (&mut Visibility, &mut DefaultWidgetEntity),
            With<IsFamiqSelectionChoicesPanel>
        >,
        panel_entity: Entity
    ) {
        if let Ok((mut visibility, mut default_widget)) = panel_q.get_mut(panel_entity) {
            *visibility = Visibility::Visible;
            default_widget.visibility = Visibility::Visible;
        }
    }

    pub fn hide_choice_panel(
        panel_q: &mut Query<
            (&mut Visibility, &mut DefaultWidgetEntity),
            With<IsFamiqSelectionChoicesPanel>
        >,
        panel_entity: Entity
    ) {
        if let Ok((mut visibility, mut default_widget)) = panel_q.get_mut(panel_entity) {
            *visibility = Visibility::Hidden;
            default_widget.visibility = Visibility::Hidden;
        }
    }
}

pub struct FaSelectionBuilder<'a> {
    pub id: Option<String>,
    pub placeholder: String,
    pub class: Option<String>,
    pub choices: Option<Vec<String>>,
    pub font_handle: Handle<Font>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaSelectionBuilder<'a> {
    pub fn new(
        placeholder: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>
    ) -> Self {
        Self {
            id: None,
            placeholder,
            class: None,
            choices: Some(Vec::new()),
            font_handle,
            root_node
        }
    }

    fn _process_built_in_classes(&self) -> (SelectorColor, SelectorVariant, SelectorShape, SelectionSize) {
        let mut use_color = SelectorColor::Default;
        let mut use_size = SelectionSize::Normal;
        let mut use_shape = SelectorShape::Default;
        let mut use_variant = SelectorVariant::Default;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    "is-underlined" => use_variant = SelectorVariant::Underlined,
                    "is-outlined" => use_variant = SelectorVariant::Outlined,

                    "is-small" => use_size = SelectionSize::Small,
                    "is-large" => use_size = SelectionSize::Large,

                    "is-round" => use_shape = SelectorShape::Round,
                    "is-rectangle" => use_shape = SelectorShape::Rectangle,

                    "is-primary" => use_color = SelectorColor::Primary,
                    "is-secondary" => use_color = SelectorColor::Secondary,
                    "is-danger" => use_color = SelectorColor::Danger,
                    "is-success" => use_color = SelectorColor::Success,
                    "is-warning" => use_color = SelectorColor::Warning,
                    "is-info" => use_color = SelectorColor::Info,

                    _ => ()
                }
            }
        }
        (use_color, use_variant, use_shape, use_size)
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn choices<I>(mut self, choices: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.choices = Some(choices.into_iter().map(Into::into).collect());
        self
    }

    pub fn build(&mut self) -> Entity {
        let (color, variant, shape, size) = self._process_built_in_classes();
        FaSelection::new(
            self.id.clone(),
            self.class.clone(),
            self.placeholder.as_str(),
            &mut self.root_node,
            self.font_handle.clone(),
            variant,
            color,
            size,
            shape,
            self.choices.as_ref().unwrap()
        )
    }
}

pub fn fa_selection<'a>(
    builder: &'a mut FamiqWidgetBuilder,
    placeholder: &str
) -> FaSelectionBuilder<'a> {
    let font_handle = builder.asset_server.load(builder.font_path.as_ref().unwrap());
    FaSelectionBuilder::new(
        placeholder.to_string(),
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_selection_systems(selection_q: Query<&IsFamiqSelectionContainer>) -> bool {
    selection_q.iter().count() > 0
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::widgets::color::PRIMARY_DARK_COLOR;
    use super::*;

    fn setup_test_default_selection(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_selection(&mut builder, "Test select choice").id("#test-selection").build();
    }

    fn setup_test_selection_with_built_in_class_color(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_selection(&mut builder, "Test select choice")
            .class("is-primary")
            .build();
    }

    fn setup_test_selection_with_built_in_class_shape(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_selection(&mut builder, "Test select choice")
            .class("is-rectangle")
            .build();
    }

    fn setup_test_selection_with_choices(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_selection(&mut builder, "Test select choice")
            .choices(vec!["Test one", "Test two"])
            .build();
    }

    #[test]
    fn test_create_default_selection() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_selection);
        app.update();

        let selection_q = app.world_mut()
            .query::<(&FamiqWidgetId, &IsFamiqSelectionContainer)>()
            .get_single(app.world());

        assert!(selection_q.is_ok(), "There should be only 1 selection");

        let selection_id = selection_q.unwrap().0;
        assert_eq!("#test-selection".to_string(), selection_id.0);
    }

    #[test]
    fn test_create_selection_with_built_in_class_color() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_selection_with_built_in_class_color);
        app.update();

        let selector_q = app.world_mut()
            .query::<(&BackgroundColor, &IsFamiqSelectionSelector)>()
            .get_single(app.world());

        let selector_bg = selector_q.unwrap().0;
        assert_eq!(
            BackgroundColor(PRIMARY_DARK_COLOR),
            *selector_bg
        );
    }

    #[test]
    fn test_create_selection_with_built_in_class_shape() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_selection_with_built_in_class_shape);
        app.update();

        let selector_q = app.world_mut()
            .query::<(&BorderRadius, &IsFamiqSelectionSelector)>()
            .get_single(app.world());

        let selector_border = selector_q.unwrap().0;
        assert_eq!(
            BorderRadius::all(Val::Px(0.0)),
            *selector_border
        );
    }

    #[test]
    fn test_create_selection_with_choices() {
        let mut app = utils::create_test_app();
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_selection_with_choices);
        app.update();

        let panel_q = app.world_mut()
            .query::<(&Children, &IsFamiqSelectionChoicesPanel)>()
            .get_single(app.world());

        // 2 provided choices, 1 default "-/-"
        assert_eq!(3 as usize, panel_q.unwrap().0.len());
    }
}
