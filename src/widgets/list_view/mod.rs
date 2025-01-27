pub mod helper;

// use crate::event_writer::FaInteractionEvent;
use crate::utils;
use crate::widgets::{
    DefaultWidgetEntity, FamiqWidgetId, FamiqWidgetResource,
    FamiqWidgetClasses, WidgetType, FamiqWidgetBuilder,
    WidgetStyle, ExternalStyleHasChanged
};
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

/// Represents Famiq Image widget.
pub struct FaListView;

// Doesn't need container
impl<'a> FaListView {
    fn _build_move_panel(
        id: &Option<String>,
        items: &Vec<Entity>,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let node = default_move_panel_node();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Inherited;

        let move_panel_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                bg_color.clone(),
                border_color.clone(),
                border_radius.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqListViewMovePanel,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                ScrollList::default()
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(move_panel_entity).insert(FamiqWidgetId(format!("{id}_move_panel")));
        }

        // insert IsFamiqListViewItem component into user provided items's entities
        for (_index, item_entity) in items.iter().enumerate() {
            let cloned = item_entity.clone();
            root_node
                .commands()
                .entity(cloned)
                .insert((IsFamiqListViewItem,));
        }
        utils::entity_add_children(root_node, items, move_panel_entity);
        move_panel_entity
    }

    fn _build_listview(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        panel_entity: Entity,
    ) -> Entity {
        let node = default_listview_node();
        let bg_color = BackgroundColor::default();
        let border_color = BorderColor::default();
        let border_radius = BorderRadius::default();
        let z_index = ZIndex::default();
        let visibility = Visibility::Visible;

        let listview_entity = root_node
            .commands()
            .spawn((
                node.clone(),
                border_color.clone(),
                border_radius.clone(),
                bg_color.clone(),
                z_index.clone(),
                visibility.clone(),
                IsFamiqListView,
                DefaultWidgetEntity::new(
                    node,
                    border_color,
                    border_radius,
                    bg_color,
                    z_index,
                    visibility,
                ),
                Interaction::default(),
                ListViewMovePanelEntity(panel_entity),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false)
            ))
            .id();

        if let Some(id) = id {
            root_node.commands().entity(listview_entity).insert(FamiqWidgetId(id));
        }
        if let Some(class) = class {
            root_node.commands().entity(listview_entity).insert(FamiqWidgetClasses(class));
        }
        listview_entity
    }

    pub fn new(
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        items: &Vec<Entity>
    ) -> Entity {
        let move_panel = Self::_build_move_panel(&id, items, root_node);
        let listview = Self::_build_listview(id, class, root_node, move_panel);

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


    /// System to handle scrolling interactions on ListView widgets.
    ///
    /// # Parameters
    /// - `mouse_wheel_events`: A reader for mouse wheel scroll events.
    /// - `listview_q`: A query for ListView components.
    /// - `panel_q`: A query for move panel components.
    /// - `can_be_scrolled_listview`: A resource tracking the currently hovered ListView entity.
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
}

/// Builder for creating `FaListView` entities with customizable options.
pub struct FaListViewBuilder<'a> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub children: Option<Vec<Entity>>,
    pub root_node: EntityCommands<'a>
}

impl<'a> FaListViewBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            children: Some(Vec::new()),
            root_node
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
        self.children = Some(children.into_iter().collect());
        self
    }

    /// Spawn listview into UI World.
    pub fn build(&mut self) -> Entity {
        FaListView::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_node,
            self.children.as_ref().unwrap()
        )
    }
}

/// API to create `FaListViewBuilder`.
pub fn fa_listview<'a>(builder: &'a mut FamiqWidgetBuilder) -> FaListViewBuilder<'a> {
    FaListViewBuilder::new(builder.ui_root_node.reborrow())
}

/// Determines if ListView internal system(s) can run.
///
/// True only if there is a listview widget created.
pub fn can_run_list_view_systems(listview_q: Query<&IsFamiqListView>) -> bool {
    listview_q.iter().count() > 0
}

#[cfg(test)]
mod tests {
    use crate::plugin::FamiqPlugin;
    use crate::widgets::button::fa_button;
    use bevy::input::InputPlugin;
    use super::*;

    fn setup_test_default_listview(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        fa_listview(&mut builder).id("#test-listview").build();
    }

    fn setup_test_listview_with_children(
        mut commands: Commands,
        asset_server: ResMut<AssetServer>,
        mut builder_res: ResMut<FamiqWidgetResource>,
    ) {
        let mut builder = FamiqWidgetBuilder::new(&mut commands, &mut builder_res, &asset_server);
        let button_one = fa_button(&mut builder, "Button 1").build();
        let button_two = fa_button(&mut builder, "Button 2").build();

        fa_listview(&mut builder)
            .children(vec![button_one, button_two])
            .build();
    }

    #[test]
    fn test_create_default_listview() {
        let mut app = utils::create_test_app();
        app.add_plugins(InputPlugin::default());
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_default_listview);

        app.update();

        let listview_q = app.world_mut()
            .query::<(&FamiqWidgetId, &Children, &IsFamiqListView)>()
            .get_single(app.world());

        assert!(listview_q.is_ok(), "There should be only 1 listview");

        let listview_id = listview_q.as_ref().unwrap().0;
        assert_eq!("#test-listview".to_string(), listview_id.0, "Should match");

        // The listview itself has only 1 child which is move_panel.
        // All the children provided by users belong to move_panel
        assert_eq!(1 as usize, listview_q.unwrap().1.len(), "Should be 1");
    }

    #[test]
    fn test_create_listview_with_children() {
        let mut app = utils::create_test_app();
        app.add_plugins(InputPlugin::default());
        app.add_plugins(FamiqPlugin);
        app.insert_resource(FamiqWidgetResource::default());
        app.add_systems(Startup, setup_test_listview_with_children);
        app.update();

        let move_panel_q = app.world_mut()
            .query::<(&Children, &IsFamiqListViewMovePanel)>()
            .get_single(app.world());

        assert_eq!(2 as usize, move_panel_q.unwrap().0.len());
    }
}
