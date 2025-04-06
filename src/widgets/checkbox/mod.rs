pub mod styling;
use styling::*;

use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;

use crate::event_writer::FaValueChangeEvent;
use crate::plugin::{CursorType, CursorIcons};
use crate::utils::*;
use crate::widgets::color::*;
use crate::widgets::*;

#[derive(Component)]
pub struct IsFamiqCheckbox;

#[derive(Component)]
pub struct IsFamiqCheckboxItem;

#[derive(Component, Default)]
pub struct CheckBoxValues(pub Vec<String>);

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
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_main_container_node();

        if vertical {
            style_components.node.flex_direction = FlexDirection::Column;
        }

        let entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                DefaultWidgetEntity::from(style_components),
                CheckBoxValues::default(),
                IsFamiqMainWidget,
                IsFamiqCheckbox
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
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
        root_node: &'a mut EntityCommands,
        text: &str
    ) -> Entity {
        let text_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: 16.0,
            ..default()
        };

        let entity = root_node
            .commands()
            .spawn((
                Text::new(text),
                text_font.clone(),
                TextColor(get_color(&attributes.color)),
                TextLayout::default(),
                DefaultTextEntity::new(
                    Text::new(text),
                    text_font,
                    TextColor(get_color(&attributes.color)),
                    TextLayout::default()
                ),
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
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
            let choice_text = Self::build_choice_text(attributes, root_node, choice);
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
        mut item_box_q: Query<(&mut CheckBoxChoiceTicked, &mut BackgroundColor)>,
        mut checkbox_q: Query<(Entity, &mut CheckBoxValues, Option<&FamiqWidgetId>)>,
        item_q: Query<(&CheckBoxItemBoxEntity, &CheckBoxItemText, &CheckBoxMainContainerEntity)>,
        mut change_writer: EventWriter<FaValueChangeEvent>
    ) {
        if let Ok((box_entity, item_text, main_entity)) = item_q.get(trigger.entity()) {

            // change box background color
            if let Ok((mut box_ticked, mut bg_color)) = item_box_q.get_mut(box_entity.0) {
                box_ticked.0 = !box_ticked.0;

                if box_ticked.0 {
                    bg_color.0 = PRIMARY_DARK_COLOR;
                } else {
                    bg_color.0 = Color::NONE;
                }
            }

            // update CheckBoxValues
            if let Ok((checkbox_entity, mut checkbox_values, id)) = checkbox_q.get_mut(main_entity.0) {
                if checkbox_values.0.contains(&item_text.0) {
                    checkbox_values.0.retain(|value| *value != item_text.0);
                } else {
                    checkbox_values.0.push(item_text.0.clone());
                }

                change_writer.send(FaValueChangeEvent::new(
                    checkbox_entity,
                    id.map(|_id| _id.0.clone()),
                    String::new(),
                    checkbox_values.0.clone()
                ));
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
    pub fn new(font_handle: Handle<Font>, root_node: EntityCommands<'a>) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
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

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
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
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaCheckboxBuilder::new(
        font_handle,
        builder.ui_root_node.reborrow()
    )
}

#[macro_export]
macro_rules! fa_checkbox {
    (
        $builder:expr
        $(, $($rest:tt)+)?
    ) => {{
        let mut checkbox = fa_checkbox_builder($builder);
        $(
            $crate::fa_checkbox_attributes!(checkbox, $($rest)+);
        )?
        checkbox.build()
    }};
}

#[macro_export]
macro_rules! fa_checkbox_attributes {
    ($checkbox:ident, id: $id:expr $(, $($rest:tt)+)?) => {{
        $checkbox = $checkbox.id($id);
        $(
            $crate::fa_checkbox_attributes!($checkbox, $($rest)+);
        )?
    }};

    ($checkbox:ident, class: $class:expr $(, $($rest:tt)+)?) => {{
        $checkbox = $checkbox.class($class);
        $(
            $crate::fa_checkbox_attributes!($checkbox, $($rest)+);
        )?
    }};

    ($checkbox:ident, color: $color:expr $(, $($rest:tt)+)?) => {{
        $checkbox = $checkbox.color($color);
        $(
            $crate::fa_checkbox_attributes!($checkbox, $($rest)+);
        )?
    }};

    ($checkbox:ident, display: $display:expr $(, $($rest:tt)+)?) => {{
        $checkbox = $checkbox.display($display);
        $(
            $crate::fa_checkbox_attributes!($checkbox, $($rest)+);
        )?
    }};

    ($checkbox:ident, choices: $choices:expr $(, $($rest:tt)+)?) => {{
        $checkbox = $checkbox.choices($choices.into_iter());
        $(
            $crate::fa_checkbox_attributes!($checkbox, $($rest)+);
        )?
    }};

    ($checkbox:ident, vertical: $vertical:expr $(, $($rest:tt)+)?) => {{
        $checkbox = $checkbox.vertical($vertical);
        $(
            $crate::fa_checkbox_attributes!($checkbox, $($rest)+);
        )?
    }};
}


pub fn can_run_checkbox_systems(checkbox_q: Query<&IsFamiqCheckbox>) -> bool {
    !checkbox_q.is_empty()
}
