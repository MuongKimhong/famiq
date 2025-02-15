pub mod helper;
pub mod tests;

use crate::utils::{self, insert_id_and_class};
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetId,
    WidgetType, WidgetStyle, ExternalStyleHasChanged
};
use crate::event_writer::FaInteractionEvent;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use helper::*;

use super::{FamiqWidgetClasses, IsFaWidgetRoot};

/// Marker component indentifying Famiq Listview widget.
#[derive(Component)]
pub struct IsFamiqListView;

/// Marker component indentifying Famiq Listview's items.
#[derive(Component)]
pub struct IsFamiqListViewItem;

/// Marker component identifying Listview move panel.
#[derive(Component)]
pub struct IsFamiqListViewMovePanel;

#[derive(Component)]
pub struct ListViewMovePanelEntity(pub Entity);

#[derive(Component, Default)]
pub struct ScrollList {
    pub position: f32,
}

/// only listview with entity inside this resource can be scrolled
#[derive(Resource)]
pub struct CanBeScrolledListView {
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct FaListviewChildren(pub Vec<Entity>);

pub struct FaListView;

// Doesn't need container
impl FaListView {
    fn _build_move_panel(
        id: &Option<String>,
        items: &Vec<Entity>,
        commands: &mut Commands,
    ) -> Entity {
        let move_panel_entity = commands
            .spawn((
                default_move_panel_node(),
                BackgroundColor::default(),
                BorderColor::default(),
                BorderRadius::default(),
                ZIndex::default(),
                Visibility::Inherited,
                IsFamiqListViewMovePanel,
                DefaultWidgetEntity::new(
                    default_move_panel_node(),
                    BorderColor::default(),
                    BorderRadius::default(),
                    BackgroundColor::default(),
                    ZIndex::default(),
                    Visibility::Inherited,
                ),
                ScrollList::default()
            ))
            .id();

        if let Some(id) = id {
            commands.entity(move_panel_entity).insert(FamiqWidgetId(format!("{id}_move_panel")));
        }

        // insert IsFamiqListViewItem component into user provided items's entities
        for (_index, item_entity) in items.iter().enumerate() {
            let cloned = item_entity.clone();
            commands
                .entity(cloned)
                .insert(IsFamiqListViewItem);
        }
        utils::entity_add_children(commands, items, move_panel_entity);
        move_panel_entity
    }

    // (top & bottom)
    fn _calculate_listview_padding_y(listview_style: &Node) -> Option<(f32, f32)> {
        let padding_top = utils::extract_val(listview_style.padding.top);
        let padding_bottom = utils::extract_val(listview_style.padding.bottom);

        if padding_top == None || padding_bottom == None {
            return None;
        }
        Some((padding_top.unwrap(), padding_bottom.unwrap()))
    }

    fn _calculate_max_scroll(
        panel_node: &ComputedNode,
        listview_node: &ComputedNode,
        // listview_style: &Node,
    ) -> f32 {
        let panel_height = panel_node.size().y;
        let container_height = listview_node.size().y;

        let max_scroll = panel_height - container_height;
        max_scroll.max(0.0)
    }

    /// Internal system to track hover interactions on ListView widgets.
    pub fn on_hover_system(
        mut interaction_events: EventReader<FaInteractionEvent>,
        mut can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
    ) {
        for e in interaction_events.read() {
            if e.widget == WidgetType::ListView && e.interaction == Interaction::Hovered {
                can_be_scrolled_listview.entity = Some(e.entity);
                break;
            }
        }
    }


    /// Internal system to handle scrolling interactions on ListView widgets.
    pub fn on_scroll_system(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut listview_q: Query<(&mut Node, &ComputedNode, &ListViewMovePanelEntity), Without<ScrollList>>,
        mut panel_q: Query<(&mut Node, &ComputedNode, &mut ScrollList, &mut DefaultWidgetEntity)>,
        can_be_scrolled_listview: ResMut<CanBeScrolledListView>,
    ) {
        let scroll_height: f32 = 20.0;

        // for (mut listview_node, _, _) in listview_q.iter_mut() {
        //     // always set paddings to 0 as ListView can't have paddings.
        //     listview_node.padding = UiRect::all(Val::Px(0.0));
        // }

        for e in mouse_wheel_events.read() {
            if let Some(hovered_listview) = can_be_scrolled_listview.entity {

                // get hovered listview
                if let Ok((_, listview_c_node, panel_entity)) = listview_q.get_mut(hovered_listview) {

                    // get panel
                    if let Ok((mut panel_node, panel_c_node, mut scroll_list, mut default_style)) = panel_q.get_mut(panel_entity.0) {

                        let dy = match e.unit {
                            MouseScrollUnit::Line => e.y * scroll_height,
                            MouseScrollUnit::Pixel => e.y,
                        };
                        let max_scroll = Self::_calculate_max_scroll(panel_c_node, listview_c_node);

                        scroll_list.position = (scroll_list.position + dy).clamp(-max_scroll, 0.0);

                        panel_node.top = Val::Px(scroll_list.position);
                        default_style.node.top = Val::Px(scroll_list.position);

                    }
                }
            }
        }
    }

    pub fn _detect_fa_listview_creation_system(
        mut commands: Commands,
        root_q: Query<Entity, With<IsFaWidgetRoot>>,
        listview_q: Query<
            (Entity, &FaListviewChildren, Option<&FamiqWidgetId>, Option<&FamiqWidgetClasses>),
            Added<IsFamiqListView>
        >
    ) {
        for (entity, children, id, class) in listview_q.iter() {
            let id_ref = id.map(|s| s.0.clone());
            let class_ref = class.map(|s| s.0.clone());
            let panel_entity = FaListView::_build_move_panel(&id_ref, &children.0, &mut commands);

            let mut node = default_listview_node();
            utils::process_spacing_built_in_class(&mut node, &class_ref);

            commands
                .entity(entity)
                .add_child(panel_entity)
                .insert((
                    node.clone(),
                    BorderColor::default(),
                    BorderRadius::default(),
                    BackgroundColor::default(),
                    ZIndex::default(),
                    Visibility::Visible,
                    DefaultWidgetEntity::new(
                        node,
                        BorderColor::default(),
                        BorderRadius::default(),
                        BackgroundColor::default(),
                        ZIndex::default(),
                        Visibility::Visible,
                    ),
                    Interaction::default(),
                    ListViewMovePanelEntity(panel_entity),
                    WidgetStyle::default(),
                    ExternalStyleHasChanged(false)
                ));

            if let Ok(root_entity) = root_q.get_single() {
                commands.entity(root_entity).add_child(entity);
            }
        }
    }
}

/// Builder for creating `FaListView` entities with customizable options.
pub struct FaListViewBuilder<'w, 's> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: Vec<Entity>,
    pub commands: Commands<'w, 's>
}

impl<'w, 's> FaListViewBuilder<'w, 's> {
    pub fn new(commands: Commands<'w, 's>) -> Self {
        Self {
            id: None,
            class: None,
            children: Vec::new(),
            commands
        }
    }

    /// Method to add class to listview.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to listview.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Adds child entities to the ListView.
    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Spawn listview into UI World.
    pub fn build(&mut self) -> Entity {
        let entity = self.commands.spawn((
            IsFamiqListView,
            FaListviewChildren(self.children.clone())
        )).id();
        insert_id_and_class(&mut self.commands, entity, &self.id, &self.class);
        entity
    }
}

/// API to create `FaListViewBuilder`.
pub fn fa_listview<'w, 's>(commands: &'w mut Commands) -> FaListViewBuilder<'w, 's>
where
    'w: 's
{
    FaListViewBuilder::new(commands.reborrow())
}

/// Determines if ListView internal system(s) can run.
///
/// True only if there is a listview widget created.
pub fn can_run_list_view_systems(listview_q: Query<&IsFamiqListView>) -> bool {
    listview_q.iter().count() > 0
}
