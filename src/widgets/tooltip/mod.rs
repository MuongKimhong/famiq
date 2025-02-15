use bevy::prelude::*;

use super::{color::{BLACK_COLOR, WHITE_COLOR}, FamiqWidgetResource, IsFaWidgetRoot};

const TOP_OFFSET: f32 = 10.0;
const BOTTOM_OFFSET: f32 = 15.0;

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
    pub widget_size: Vec2,
    pub widget_translation: Vec3
}

impl FaToolTipResource {
    pub fn show(&mut self, text: String, widget_size: Vec2, widget_translation: Vec3) {
        self.visible = true;
        self.text = text;
        self.widget_size = widget_size;
        self.widget_translation = widget_translation;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn get_top_pos(&self, size: Vec2) -> f32 {
        let mut top_pos = self.widget_translation.y
            - (self.widget_size.y / 2.0)
            - size.y
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
    fn _build_text(
        commands: &mut Commands,
        font_handle: Handle<Font>
    ) -> Entity {
        commands
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

    pub fn handle_show_hide_tooltip_system(
        tooltip_res: Res<FaToolTipResource>,
        mut tooltip_q: Query<(
            &mut Visibility,
            &mut Node,
            &ComputedNode,
            &FamiqToolTipTextEntity
        )>,
        mut tooltip_text_q: Query<&mut Text, With<IsFamiqToolTipText>>
    ) {
        if tooltip_res.is_changed() {
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
                let size = computed_node.size();
                node.top = Val::Px(tooltip_res.get_top_pos(size));
                node.left = Val::Px(tooltip_res.get_left_pos(size));
                *visibility = Visibility::Visible;
            }
        }
    }

    pub fn detect_tooltip_creation_system(
        mut commands: Commands,
        famiq_res: Res<FamiqWidgetResource>,
        asset_server: Res<AssetServer>,
        tooltip_q: Query<Entity, Added<IsFamiqToolTipContainer>>,
        root_q: Query<Entity, With<IsFaWidgetRoot>>
    ) {
        // despawn if tooltip widget is already created
        if tooltip_q.iter().count() > 1 {
            let entity = tooltip_q.get_single().unwrap();
            commands.entity(entity).despawn();
        }
        let container_entity = tooltip_q.get_single().unwrap();
        let font_handle = asset_server.load(&famiq_res.font_path);
        let text_entity = FaToolTip::_build_text(&mut commands, font_handle);

        commands
            .entity(container_entity)
            .add_child(text_entity)
            .insert((
                Node {
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
                },
                BorderColor(WHITE_COLOR),
                BackgroundColor(WHITE_COLOR),
                BorderRadius::all(Val::Px(5.0)),
                ZIndex::default(),
                Visibility::Hidden,
                IsFamiqToolTipContainer,
                GlobalZIndex(4),
                FamiqToolTipTextEntity(text_entity)
            ));

        if let Ok(root_entity) = root_q.get_single() {
            commands.entity(root_entity).add_child(container_entity);
        }
    }
}
