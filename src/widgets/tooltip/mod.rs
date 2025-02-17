use bevy::prelude::*;

use super::color::BLACK_COLOR;

const TOP_OFFSET: f32 = 20.0;
const BOTTOM_OFFSET: f32 = 15.0;

#[derive(Component)]
pub struct IsFamiqToolTipText;

#[derive(Resource, Default)]
pub struct FaToolTipResource {
    pub visible: bool,
    pub text: String,
    pub widget_size: Vec2,
    pub widget_translation: Vec3,
}

impl FaToolTipResource {
    pub fn show(&mut self, text: String, widget_size: Vec2, widget_translation: Vec3) {
        self.text = text;
        self.widget_size = widget_size;
        self.widget_translation = widget_translation;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn get_top_pos(&self, size: Vec2) -> f32 {
        let mut top_pos = self.widget_translation.y
            - (self.widget_size.y / 2.0)
            - (size.y / 2.0)
            - TOP_OFFSET;

        if top_pos < 0.0 {
            top_pos += size.y + self.widget_size.y + BOTTOM_OFFSET;
        }
        top_pos
    }

    pub fn get_left_pos(&self, size: Vec2) -> f32 {
        let mut left_pos = self.widget_translation.x - (size.x / 2.0);

        if left_pos < 0.0 {
            left_pos = 0.0;
        }
        left_pos
    }
}

pub struct FaToolTip;

impl<'a> FaToolTip {
    fn _build_tooltip(
        root_node: &'a mut EntityCommands,
        font_handle: Handle<Font>
    ) -> Entity {
        let node = Node {
            width: Val::Auto,
            height: Val::Auto,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            padding: UiRect {
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                top: Val::Px(2.0),
                bottom: Val::Px(0.0),
            },
            ..default()
        };
        let color = Color::srgba(1.0, 1.0, 1.0, 0.8);

        root_node
            .commands()
            .spawn((
                Text::new(""),
                TextFont {
                    font: font_handle,
                    font_size: 15.,
                    ..default()
                },
                TextColor(BLACK_COLOR),
                TextLayout::new_with_justify(JustifyText::Center),
                IsFamiqToolTipText
            ))
            .insert((
                node,
                BorderColor(color),
                BackgroundColor(color),
                BorderRadius::all(Val::Px(5.0)),
                ZIndex::default(),
                Visibility::Hidden,
                GlobalZIndex(4)
            ))
            .id()
    }

    pub fn new(root_node: &'a mut EntityCommands, font_handle: Handle<Font>) -> Entity {
        Self::_build_tooltip(root_node, font_handle)
    }

    pub fn _update_toolitp_text(
        new_text: &str,
        tooltip_text_q: &mut Query<&mut Text, With<IsFamiqToolTipText>>
    ) {
        if new_text.is_empty() {
            return;
        }
        if let Ok(mut text) = tooltip_text_q.get_single_mut() {
            text.0 = new_text.to_string();
        }
    }

    pub fn handle_show_hide_tooltip_system(
        tooltip_res: Res<FaToolTipResource>,
        mut tooltip_q: Query<(
            &mut Visibility,
            &mut Node,
            &ComputedNode
        ), With<IsFamiqToolTipText>>,
    ) {
        if tooltip_res.is_changed() {
            if let Ok((mut visibility, mut node, computed_node)) = tooltip_q.get_single_mut() {
                if !tooltip_res.visible {
                    *visibility = Visibility::Hidden;
                    return;
                }
                let size = computed_node.size();
                node.top = Val::Px(tooltip_res.get_top_pos(size));
                node.left = Val::Px(tooltip_res.get_left_pos(size));
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn can_run_tooltip_systems(tooltip_q: Query<&IsFamiqToolTipText>) -> bool {
    !tooltip_q.is_empty()
}
