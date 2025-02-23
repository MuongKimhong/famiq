pub mod helper;
pub mod tests;

use crate::utils;
use crate::resources::*;
use crate::widgets::*;
use crate::event_writer::FaInteractionEvent;
use bevy::ecs::system::EntityCommands;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use helper::*;

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
pub struct FaListViewChildren(pub Vec<Entity>);

#[derive(Component)]
pub struct ListViewMovePanelEntity(pub Entity);

#[derive(Component)]
pub struct ScrollList {
    pub position: f32,
    pub scroll_height: f32
}

impl ScrollList {
    fn new(scroll_height: f32) -> Self {
        Self {
            position: 0.0,
            scroll_height
        }
    }
}

/// only listview with entity inside this resource can be scrolled
#[derive(Resource)]
pub struct CanBeScrolledListView {
    pub entity: Option<Entity>,
}

#[derive(Default)]
pub struct IsFamiqListViewResource;
pub type FaListViewResource = ContainableResource<IsFamiqListViewResource>;

pub struct FaListView;

// Doesn't need container
impl<'a> FaListView {
    fn _build_move_panel(
        id: &Option<String>,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands,
        scroll_height: f32
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_move_panel_node();

        let panel_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqListViewMovePanel,
                DefaultWidgetEntity::from(style_components),
                ScrollList::new(scroll_height)
            ))
            .id();

        if let Some(id) = id {
            // root_node.commands().entity(move_panel_entity).insert(FamiqWidgetId(format!("{id}_move_panel")));
            root_node.commands().entity(panel_entity).insert(FamiqWidgetId(id.to_owned()));
        }

        // insert IsFamiqListViewItem component into user provided items's entities
        for (_index, item_entity) in items.iter().enumerate() {
            let cloned = item_entity.clone();
            root_node
                .commands()
                .entity(cloned)
                .insert((IsFamiqListViewItem,));
        }
        utils::entity_add_children(root_node, items, panel_entity);
        panel_entity
    }

    fn _build_listview(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        panel_entity: Entity,
        items: &Vec<Entity>
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.visibility = Visibility::Visible;

        let listview_entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                IsFamiqListView,
                DefaultWidgetEntity::from(style_components),
                ListViewMovePanelEntity(panel_entity),
                FaListViewChildren(items.clone())
            ))
            .id();

        utils::insert_id_and_class(root_node, listview_entity, &attributes.id, &attributes.class);
        listview_entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        items: &Vec<Entity>,
        scroll_height: f32
    ) -> Entity {
        let move_panel = Self::_build_move_panel(&attributes.id, items, root_node, scroll_height);
        let listview = Self::_build_listview(attributes, root_node, move_panel, items);

        utils::entity_add_child(root_node, move_panel, listview);
        root_node.add_child(listview);
        listview
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

    /// System to track hover interactions on ListView widgets.
    ///
    /// # Parameters
    /// - `interaction_events`: A reader for `FaInteractionEvent` events.
    /// - `can_be_scrolled_listview`: A mutable resource tracking the currently hovered ListView entity.
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
                            MouseScrollUnit::Line => e.y * scroll_list.scroll_height,
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

    pub fn detect_new_listview_system(
        mut commands: Commands,
        mut listview_res: ResMut<FaListViewResource>,
        listview_q: Query<
            (Entity, Option<&FamiqWidgetId>, &FaListViewChildren, &ListViewMovePanelEntity),
            Added<IsFamiqListView>
        >,
        mut panel_q: Query<&mut Node, With<IsFamiqListViewMovePanel>>
    ) {
        for (entity, id, children, panel_entity) in listview_q.iter() {
            if let Some(_id) = id {
                if listview_res.containers.get(&_id.0).is_none() {
                    listview_res.containers.insert(_id.0.clone(), ContainableData {
                        entity: Some(entity),
                        children: children.0.clone()
                    });
                    commands.entity(entity).remove::<FaListViewChildren>();
                }
            }
            if let Ok(mut panel_node) = panel_q.get_mut(panel_entity.0) {
                panel_node.padding = UiRect::all(Val::Px(0.0));
            }
        }
    }

    pub(crate) fn detect_listview_resource_change(
        mut commands: Commands,
        listview_res: Res<FaListViewResource>,
        listview_q: Query<&ListViewMovePanelEntity>,
        mut child_q: Query<
            (
                &mut Node,
                &mut DefaultWidgetEntity,
                Option<&FamiqWidgetId>,
                Option<&FamiqWidgetClasses>,
            )
        >,
        mut styles: ResMut<StylesKeyValueResource>
    ) {
        if listview_res.is_changed() && !listview_res.is_added() {
            if listview_res.changed_container.is_none() {
                return;
            }

            let changed_listview = listview_res.changed_container.unwrap();

            let panel_entity = match listview_q.get(changed_listview) {
                Ok(v) => v.0,
                Err(_) => return,
            };

            match listview_res.method_called {
                ContainableMethodCall::AddChildren => {
                    commands
                        .entity(panel_entity)
                        .add_children(&listview_res.to_use_children);
                }
                ContainableMethodCall::InsertChildren => {
                    commands
                        .entity(panel_entity)
                        .insert_children(listview_res.insert_index, &listview_res.to_use_children);
                }
                ContainableMethodCall::RemoveChildren => {
                    commands
                        .entity(panel_entity)
                        .remove_children(&listview_res.to_use_children);

                    for child in listview_res.to_use_children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            }

            let mut changed_json_style_keys: Vec<String> = Vec::new();
            for child in listview_res.to_use_children.iter() {
                if let Ok((mut node, mut default_widget, id, class)) = child_q.get_mut(*child) {
                    if node.display == Display::None {
                        node.display = Display::Flex;
                        default_widget.node.display = Display::Flex;
                    }
                    if let Some(id) = id {
                        changed_json_style_keys.push(id.0.clone());
                    }
                    if let Some(classes) = class {
                        let classes_split: Vec<&str> = classes.0.split_whitespace().collect();
                        for class_name in classes_split {
                            let formatted = format!(".{class_name}");
                            if !changed_json_style_keys.contains(&formatted) {
                                changed_json_style_keys.push(formatted);
                            }
                        }
                    }
                }
            }
            styles.changed_keys = changed_json_style_keys;
        }
    }
}

/// Builder for creating `FaListView` entities with customizable options.
pub struct FaListViewBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub children: Vec<Entity>,
    pub root_node: EntityCommands<'a>,
    pub scroll_height: f32
}

impl<'a> FaListViewBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            children: Vec::new(),
            root_node,
            scroll_height: 15.0
        }
    }

    ///' Method to set scroll height.
    pub fn scroll_height(mut self, scroll_height: f32) -> Self {
        self.scroll_height = scroll_height;
        self
    }

    /// Adds child entities to the ListView.
    pub fn children<I: IntoIterator<Item = Entity>>(mut self, children: I) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Spawn listview into UI World.
    pub fn build(&mut self) -> Entity {
        self._node();
        FaListView::new(
            &self.attributes,
            &mut self.root_node,
            &self.children,
            self.scroll_height
        )
    }
}

impl<'a> SetWidgetAttributes for FaListViewBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_listview_node();
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        utils::process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaListViewBuilder`.
pub fn fa_listview<'a>(builder: &'a mut FamiqBuilder) -> FaListViewBuilder<'a> {
    FaListViewBuilder::new(builder.ui_root_node.reborrow())
}

/// Determines if ListView internal system(s) can run.
///
/// True only if there is a listview widget created.
pub fn can_run_list_view_systems(listview_q: Query<&IsFamiqListView>) -> bool {
    !listview_q.is_empty()
}
