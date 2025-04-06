use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::resources::*;
use super::*;

// params that used in systems related to Buffer Texture picking
#[derive(SystemParam)]
pub(crate) struct BufTexturePickingParam<'w, 's> {
    pub input_q: Query<
        'w, 's,
        (&'static GlobalTransform, &'static ComputedNode, &'static mut FaTextEdit, &'static mut CosmicData),
        With<IsFamiqTextInput>
    >,
    pub texture_q: Query<'w, 's, (&'static Node, &'static FaTextInputEntity), With<IsFamiqTextInputBufferTexture>>,
    pub famiq_res: ResMut<'w, FamiqResource>,
    pub request_redraw: EventWriter<'w, RequestRedrawBuffer>,
    pub font_system: ResMut<'w, CosmicFontSystem>
}

// params that used in systems related to fa_text_input picking
#[derive(SystemParam)]
pub(crate) struct InputPickingParam<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub mouse_writer: EventWriter<'w, FaMouseEvent>,
    pub tooltip_q: Query<'w, 's, (&'static mut Node, &'static mut Transform), With<IsFamiqTooltip>>,
    pub window: Single<'w, Entity, With<Window>>,
    pub cursor_icons: Res<'w, CursorIcons>,
}

#[derive(SystemParam)]
pub(crate) struct RequestRedrawBufferParam<'w, 's> {
    pub request_redraw: EventReader<'w, 's, RequestRedrawBuffer>,
    pub input_q: Query<'w, 's,
        (
            &'static mut CosmicData,
            &'static CosmicDataColor,
            &'static FaTextInputBufferTextureEntity
        )
    >,
    pub font_system: ResMut<'w, CosmicFontSystem>,
    pub swash_cache: ResMut<'w, CosmicSwashCache>,
    pub image_asset: ResMut<'w, Assets<Image>>,
    pub materials: ResMut<'w, Assets<TextInputMaterial>>,
    pub texture_q: Query<
        'w, 's,
        (
            &'static MaterialNode<TextInputMaterial>,
            &'static ImageNode
        ),
        With<IsFamiqTextInputBufferTexture>
        >,
}

#[derive(SystemParam)]
pub(crate) struct DetectTextStyleChangeParam<'w, 's> {
    pub input_q: Query<
        'w, 's,
        (
            Entity,
            &'static mut CosmicData,
            &'static mut CosmicDataColor,
            &'static mut FaTextEdit,
            &'static CosmicTextData,
        ),
        Changed<CosmicTextData>,
    >,
    pub request_redraw: EventWriter<'w, RequestRedrawBuffer>,
    pub font_system: ResMut<'w, CosmicFontSystem>,
    pub famiq_res: Res<'w, FamiqResource>,
    pub window: Single<'w, &'static Window>
}

#[derive(SystemParam)]
pub(crate) struct TypingParam<'w, 's> {
    pub input_q: Query<
        'w, 's,
        (
            Entity,
            &'static ComputedNode,
            &'static FaTextInputBufferTextureEntity,
            &'static mut CursorBlinkTimer,
            &'static mut CosmicData,
            &'static mut FaTextEdit,
            &'static ReactiveModelKey
        ),
        With<IsFamiqTextInput>
    >,
    pub texture_q: Query<
        'w, 's, &'static mut Node,
        (With<IsFamiqTextInputBufferTexture>, Without<IsFamiqMainWidget>)
    >,
    pub fa_query: FaQuery<'w, 's>,
    pub evr_kbd: EventReader<'w, 's, KeyboardInput>,
    pub famiq_res: ResMut<'w, FamiqResource>,
    pub font_system: ResMut<'w, CosmicFontSystem>,
    pub request_redraw: EventWriter<'w, RequestRedrawBuffer>,
    pub keys: Res<'w, ButtonInput<KeyCode>>,
}
