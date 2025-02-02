use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::utils::entity_add_child;

use super::color::{BLACK_COLOR, WHITE_COLOR};

#[derive(Component)]
pub struct IsFamiqToolTipContainer;

#[derive(Component)]
pub struct IsFamiqToolTipText;

#[derive(Component)]
pub struct FamiqToolTipTextEntity(pub Entity);

#[derive(Resource, Default)]
pub struct FaToolTipResource {
    pub visible: bool,
    pub text: String,
    pub hovered_widget_height: f32
}

pub struct FaToolTip;

impl<'a> FaToolTip {
    fn _build_container(root_node: &'a mut EntityCommands, txt_entity: Entity) -> Entity {
        let node = Node {
            width: Val::Auto,
            height: Val::Auto,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        };
        root_node
            .commands()
            .spawn((
                node,
                BorderColor(WHITE_COLOR),
                BackgroundColor(WHITE_COLOR),
                BorderRadius::all(Val::Px(5.0)),
                ZIndex::default(),
                Visibility::Hidden,
                IsFamiqToolTipContainer,
                GlobalZIndex(4),
                FamiqToolTipTextEntity(txt_entity)
            ))
            .id()
    }

    fn _build_text(
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>
    ) -> Entity {
        root_node
            .commands()
            .spawn((
                Text::new(""),
                TextFont {
                    font: font_handle,
                    font_size: 20.,
                    ..default()
                },
                TextColor(BLACK_COLOR),
                TextLayout::new_with_justify(JustifyText::Center),
                Visibility::Inherited,
                IsFamiqToolTipText
            ))
            .id()
    }

    pub fn new(root_node: &'a mut EntityCommands, font_handle: Handle<Font>) -> Entity {
        let txt = Self::_build_text(root_node, font_handle);
        let container = Self::_build_container(root_node, txt);
        entity_add_child(root_node, txt, container);
        container
    }

    pub fn handle_show_hide_tooltip_system(
        tooltip_res: Res<FaToolTipResource>,
        window_q: Query<&Window, With<PrimaryWindow>>,
        mut tooltip_q: Query<(
            &mut Visibility,
            &mut Node,
            &ComputedNode,
            &FamiqToolTipTextEntity
        )>,
        mut tooltip_text_q: Query<&mut Text, With<IsFamiqToolTipText>>
    ) {
        if tooltip_res.is_changed() {
            let Some(cursor_position) = window_q.single().cursor_position() else { return; };

            if let Ok((mut visibility, mut node, computed_node, text_entity)) = tooltip_q.get_single_mut() {

                if !tooltip_res.visible {
                    *visibility = Visibility::Hidden;
                    return;
                }

                if tooltip_res.text.trim().is_empty() {
                    return;
                }

                if let Ok(mut tooltip_text) = tooltip_text_q.get_mut(text_entity.0) {
                    tooltip_text.0 = tooltip_res.text.clone();
                }

                let offset: f32 = 25.0;
                let screen_height = window_q.single().height();
                let screen_width = window_q.single().width();

                let top_pos = (cursor_position.y - tooltip_res.hovered_widget_height - offset).clamp(0.0, screen_height);
                let left_pos = (cursor_position.x - (computed_node.size().x / 2.0)).clamp(0.0, screen_width);

                node.top = Val::Px(top_pos);
                node.left = Val::Px(left_pos);

                *visibility = Visibility::Visible;
            }
        }
    }
}
