pub mod components;
pub mod styling;
pub mod tests;

pub use components::*;
use styling::*;

use crate::plugin::{CursorIcons, CursorType};
use crate::utils::*;
use crate::widgets::*;
use crate::event_writer::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct FaButton;

impl<'a> FaButton {
    fn _build_text(
        attributes: &WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let txt = Text::new(text);
        let txt_font = TextFont {
            font: attributes.font_handle.clone().unwrap(),
            font_size: get_text_size(&attributes.size),
            ..default()
        };
        let txt_color = TextColor(get_text_color(&attributes.color));
        let txt_layout = TextLayout::new_with_justify(JustifyText::Center);

        let entity = root_node
            .commands()
            .spawn((
                txt.clone(),
                txt_font.clone(),
                txt_color.clone(),
                txt_layout.clone(),
                DefaultTextEntity::new(txt, txt_font, txt_color, txt_layout),
                IsFamiqButtonText,
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);

        if !attributes.bind_keys.is_empty() {
            root_node
                .commands()
                .entity(entity)
                .insert((
                    ReactiveKeys(attributes.bind_keys.to_owned()),
                    ReactiveText(text.to_string())
                ));
        }
        entity
    }

    pub fn new(
        attributes: WidgetAttributes,
        text: &str,
        root_node: &'a mut EntityCommands
    ) -> Entity {
        let txt_entity = Self::_build_text(&attributes, text, root_node);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.border_color = get_color(&attributes.color).into();
        style_components.background_color = get_color(&attributes.color).into();
        style_components.border_radius = BorderRadius::all(Val::Px(6.0));

        let btn_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqButton,
                DefaultWidgetEntity::from(style_components),
                ButtonTextEntity(txt_entity),
                ButtonColorBeforePressed(None),
                IsFamiqMainWidget
            ))
            .observe(FaButton::handle_on_mouse_over)
            .observe(FaButton::handle_on_mouse_down)
            .observe(FaButton::handle_on_mouse_out)
            .observe(FaButton::handle_on_mouse_up)
            .id();

        if attributes.has_tooltip {
            build_tooltip_node(&attributes, root_node, btn_entity);
        }
        insert_id_and_class(root_node, btn_entity, &attributes.id, &attributes.class);
        entity_add_child(root_node, txt_entity, btn_entity);
        btn_entity
    }

    fn handle_on_mouse_over(
        mut over: Trigger<Pointer<Over>>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        button_q: Query<
            (&GlobalTransform, Option<&FamiqTooltipEntity>, Option<&FamiqWidgetId>),
            With<IsFamiqButton>
        >,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Pointer);

        if let Ok((transform, tooltip_entity, id)) = button_q.get(over.entity()) {
            show_tooltip(
                tooltip_entity,
                &mut tooltip_q,
                transform.translation()
            );
            FaMouseEvent::send_event(&mut writer, EventType::Over, WidgetType::Button, over.entity(), id);
        }
        over.propagate(false);
    }

    fn handle_on_mouse_down(
        mut down: Trigger<Pointer<Down>>,
        mut famiq_res: ResMut<FamiqResource>,
        mut button_q: Query<
            (&mut BackgroundColor, &mut ButtonColorBeforePressed, Option<&FamiqWidgetId>),
            With<IsFamiqButton>
        >,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok((mut bg_color, mut before_pressed_color, id)) = button_q.get_mut(down.entity()) {
            before_pressed_color.0 = Some(bg_color.0);
            famiq_res.update_all_focus_states(false);
            famiq_res.update_or_insert_focus_state(down.entity(), true);

            if let Some(darkened_color) = darken_color(20.0, &bg_color.0) {
                bg_color.0 = darkened_color;
            }
            if down.event().button == PointerButton::Secondary {
                FaMouseEvent::send_event(&mut writer, EventType::DownRight, WidgetType::Button, down.entity(), id);
            } else {
                FaMouseEvent::send_event(&mut writer, EventType::DownLeft, WidgetType::Button, down.entity(), id);
            }
        }
        down.propagate(false);
    }

    fn handle_on_mouse_up(
        mut up: Trigger<Pointer<Up>>,
        mut button_q: Query<(&mut BackgroundColor, &ButtonColorBeforePressed, Option<&FamiqWidgetId>), With<IsFamiqButton>>,
        mut writer: EventWriter<FaMouseEvent>
    ) {
        if let Ok((mut bg_color, before_pressed_color, id)) = button_q.get_mut(up.entity()) {
            if let Some(color) = before_pressed_color.0 {
                bg_color.0 = color;
            }
            FaMouseEvent::send_event(&mut writer, EventType::Up, WidgetType::Button, up.entity(), id);
        }
        up.propagate(false);
    }

    fn handle_on_mouse_out(
        mut out: Trigger<Pointer<Out>>,
        mut tooltip_q: Query<(&mut Node, &mut Transform), With<IsFamiqTooltip>>,
        mut button_q: Query<
            (Option<&FamiqTooltipEntity>, Option<&FamiqWidgetId>, &mut BackgroundColor, &ButtonColorBeforePressed),
            With<IsFamiqButton>
        >,
        mut commands: Commands,
        mut writer: EventWriter<FaMouseEvent>,
        window: Single<Entity, With<Window>>,
        cursor_icons: Res<CursorIcons>,
    ) {
        _change_cursor_icon(&mut commands, &cursor_icons, *window, CursorType::Default);

        if let Ok((tooltip_entity, id, mut bg_color, before_pressed_color)) = button_q.get_mut(out.entity()) {
            if let Some(color) = before_pressed_color.0 {
                bg_color.0 = color;
            }
            hide_tooltip(tooltip_entity, &mut tooltip_q);
            FaMouseEvent::send_event(&mut writer, EventType::Out, WidgetType::Button, out.entity(), id);
        }
        out.propagate(false);
    }
}

/// Builder for creating `fa_button`.
pub struct FaButtonBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub text: String,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaButtonBuilder<'a> {
    pub fn new(
        text: String,
        font_handle: Handle<Font>,
        root_node: EntityCommands<'a>,
    ) -> Self {
        let mut attributes = WidgetAttributes::default();
        attributes.font_handle = Some(font_handle);
        Self {
            attributes,
            text,
            root_node
        }
    }

    /// Spawn the button to UI world.
    pub fn build(&mut self) -> Entity {
        self._process_built_in_color_class();
        self._process_built_in_size_class();
        self._node();
        FaButton::new(
            self.attributes.clone(),
            self.text.as_str(),
            &mut self.root_node
        )
    }
}

impl<'a> SetWidgetAttributes for FaButtonBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_button_node();

        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }

        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create a `FaButtonBuilder`.
pub fn fa_button_builder<'a>(builder: &'a mut FamiqBuilder, text: &str) -> FaButtonBuilder<'a> {
    let font_handle = builder.asset_server.load(&builder.resource.font_path);
    FaButtonBuilder::new(
        text.to_string(),
        font_handle,
        builder.ui_root_node.reborrow(),
    )
}

#[macro_export]
macro_rules! fa_button {
    (
        $builder:expr,
        text: $text:expr
        $(, $($rest:tt)+)?
    ) => {{
        let mut button = fa_button_builder($builder, $text);
        $(
            $crate::fa_button_attributes!(button, $($rest)+);
        )?
        button.build()
    }};
}

#[macro_export]
macro_rules! fa_button_attributes {
    ($button:ident, id: $id:expr $(, $($rest:tt)+)?) => {{
        $button = $button.id($id);
        $(
            $crate::fa_button_attributes!($button, $($rest)+);
        )?
    }};

    ($button:ident, class: $class:expr $(, $($rest:tt)+)?) => {{
        $button = $button.class($class);
        $(
            $crate::fa_button_attributes!($button, $($rest)+);
        )?
    }};

    ($button:ident, color: $color:expr $(, $($rest:tt)+)?) => {{
        $button = $button.color($color);
        $(
            $crate::fa_button_attributes!($button, $($rest)+);
        )?
    }};

    ($button:ident, tooltip: $tooltip:expr $(, $($rest:tt)+)?) => {{
        $button = $button.tooltip($tooltip);
        $(
            $crate::fa_button_attributes!($button, $($rest)+);
        )?
    }};

    ($button:ident, display: $display:expr $(, $($rest:tt)+)?) => {{
        $button = $button.display($display);
        $(
            $crate::fa_button_attributes!($button, $($rest)+);
        )?
    }};

    ($button:ident, bind: $bind:expr $(, $($rest:tt)+)?) => {{
        $button = $button.bind($bind);
        $(
            $crate::fa_button_attributes!($button, $($rest)+);
        )?
    }};
}


/// Checks if the button internal system(s) can run.
///
/// `True` only if there is a button widget created.
pub fn can_run_button_systems(button_q: Query<&IsFamiqButton>) -> bool {
    !button_q.is_empty()
}
