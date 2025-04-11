pub mod styling;
use styling::*;

use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;

use crate::plugin::{CursorType, CursorIcons};
use crate::utils::*;
use crate::widgets::*;
use crate::fa_text;
use crate::fa_container;

#[derive(Component)]
pub struct IsFamiqCheckbox;

#[derive(Component)]
pub struct IsFamiqCheckboxItem;

#[derive(Component)]
pub struct CheckBoxMainContainerEntity(pub Entity);

#[derive(Component)]
pub struct CheckBoxItemText(pub String);

#[derive(Component)]
pub struct CheckBoxItemBoxEntity(pub Entity);

#[derive(Component)]
pub struct CheckBoxChoiceTicked(pub bool);

pub struct FaCheckbox;

impl<'a> FaCheckbox {
    fn build_main_container(attributes: &WidgetAttributes, root_node: &'a mut EntityCommands, vertical: bool) -> Entity {
        let class = &attributes.class.clone().unwrap_or("".into());
        let id = &attributes.id.clone().unwrap_or("".into());
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_main_container_node();

        if vertical {
            style_components.node.flex_direction = FlexDirection::Column;
        }
        let entity = fa_container!(has_observer: false, id: id, class: class);
        root_node.commands().entity(entity).insert((
            style_components.clone(),
            DefaultWidgetEntity::from(style_components),
            IsFamiqCheckbox
        ));

        if attributes.model_key.is_some() {
            root_node.commands().entity(entity).insert(ReactiveModelKey(attributes.model_key.clone().unwrap()));
        }
        entity
    }

    fn build_choice_container(
        root_node: &'a mut EntityCommands,
        text: &str,
        main_container_entity: Entity,
        box_entity: Entity
    ) -> Entity {
        root_node
            .commands()
            .spawn((
                default_choice_container_node(),
                CheckBoxItemText(text.to_string()),
                CheckBoxMainContainerEntity(main_container_entity),
                CheckBoxItemBoxEntity(box_entity),
                IsFamiqCheckboxItem
            ))
            .observe(FaCheckbox::handle_on_mouse_over)
            .observe(FaCheckbox::handle_on_mouse_out)
            .observe(FaCheckbox::handle_on_mouse_down)
            .id()
    }

    fn build_choice_text(
        attributes: &WidgetAttributes,
        text: &str
    ) -> Entity {
        let class = &attributes.class.clone().unwrap_or("".into());
        let id = &attributes.id.clone().unwrap_or("".into());
        fa_text!(text: text, id: id, class: class, has_node: false, has_observer: false)
    }

    fn build_choice_box(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        root_node
            .commands()
            .spawn((
                default_choice_box_node(),
                CheckBoxChoiceTicked(false),
                BackgroundColor::default(),
                BorderRadius::all(Val::Px(4.0)),
                BorderColor(get_color(&attributes.color))
            ))
            .id()
    }

    fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        choices: &Vec<String>,
        vertical: bool
    ) -> Entity {
        let container = Self::build_main_container(attributes, root_node, vertical);
        let mut all_choices: Vec<Entity> = Vec::with_capacity(choices.len() + 1);

        for choice in choices.iter() {
            let choice_text = Self::build_choice_text(attributes, choice);
            let choice_box = Self::build_choice_box(attributes, root_node);
            let choice_container = Self::build_choice_container(root_node, choice, container, choice_box);
            entity_add_children(root_node, &vec![choice_box, choice_text], choice_container);
            all_choices.push(choice_container);
        }

        entity_add_children(root_node, &all_choices, container);
        container
    }

    fn handle_on_mouse_over(
        mut trigger: Trigger<Pointer<Over>>,
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);
        trigger.propagate(false);
    }

    fn handle_on_mouse_out(
        mut trigger: Trigger<Pointer<Out>>,
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);
        trigger.propagate(false);
    }

    fn handle_on_mouse_down(
        mut trigger: Trigger<Pointer<Down>>,
        checkbox_q: Query<&ReactiveModelKey, With<IsFamiqCheckbox>>,
        item_q: Query<(&CheckBoxItemText, &CheckBoxMainContainerEntity)>,
        mut fa_query: FaQuery,
    ) {
        if let Ok((item_text, main_entity)) = item_q.get(trigger.entity()) {
            if let Ok(model_key) = checkbox_q.get(main_entity.0) {
                if let Some(value) = fa_query.get_data_mut(&*model_key.0) {
                    match value {
                        RVal::List(v) => {
                            if v.contains(&item_text.0) {
                                v.retain(|value| *value != item_text.0);
                            } else {
                                v.push(item_text.0.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        trigger.propagate(false);
    }
}

pub struct FaCheckboxBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub choices: Vec<String>,
    pub root_node: EntityCommands<'a>,
    pub vertical: bool // align items vertically
}

impl<'a> FaCheckboxBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            choices: Vec::new(),
            root_node,
            vertical: false
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
        self._process_built_in_color_class();
        self._node();
        FaCheckbox::new(
            &self.attributes,
            &mut self.root_node,
            &self.choices,
            self.vertical
        )
    }
}

impl<'a> SetWidgetAttributes for FaCheckboxBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_main_container_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

pub fn fa_checkbox_builder<'a>(builder: &'a mut FamiqBuilder) -> FaCheckboxBuilder<'a> {
    FaCheckboxBuilder::new(builder.ui_root_node.reborrow())
}

#[macro_export]
macro_rules! fa_checkbox {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let builder = builder_mut();
        let mut checkbox = fa_checkbox_builder(builder);
        $(
            $crate::fa_checkbox_attributes!(checkbox, $key : $value);
        )*
        checkbox.build()
    }};
}

#[macro_export]
macro_rules! fa_checkbox_attributes {
    ($checkbox:ident, model: $model:expr) => {{
        $checkbox = $checkbox.model($model);
    }};

    ($checkbox:ident, color: $color:expr) => {{
        $checkbox = $checkbox.color($color);
    }};

    ($checkbox:ident, choices: $choices:expr) => {{
        $checkbox = $checkbox.choices($choices);
    }};

    ($checkbox:ident, vertical: $vertical:expr) => {{
        $checkbox.vertical = $vertical;
    }};

    // common attributes
    ($checkbox:ident, $key:ident : $value:expr) => {{
        $crate::common_attributes!($checkbox, $key : $value);
    }};
}

pub fn can_run_checkbox_systems(checkbox_q: Query<&IsFamiqCheckbox>) -> bool {
    !checkbox_q.is_empty()
}
