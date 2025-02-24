pub mod helper;
pub mod components;
pub mod tests;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::hashbrown::HashSet;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::utils::*;
use crate::widgets::*;

pub use components::*;
use helper::*;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    u_time: f32,
    #[uniform(1)]
    u_color: Vec3,
    #[uniform(2)]
    u_blend: f32,
    #[uniform(3)]
    u_size: Vec2
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        get_embedded_asset_path("embedded_assets/shaders/progress_bar.wgsl").into()
    }
}

/// Stores the progress percetages in a `HashMap` where keys are IDs of the progress bar.
#[derive(Resource, Default, Debug)]
pub struct FaProgressBarResource {
    /// Store progress bars as id-value pairs
    pub bars: HashMap<String, Option<f32>>,

    /// Store progress bars as entity-value pairs
    pub entity_bars: HashMap<Entity, Option<f32>>,

    /// Track changed keys (for ID-based bars)
    changed_bars: HashSet<String>,
    /// Track changed keys (for Entity-based bars)
    changed_entity_bars: HashSet<Entity>,
}

impl FaProgressBarResource {
    /// Insert a progress value by ID (ensuring non-negative values)
    fn _insert_by_id(&mut self, id: String, percentage: Option<f32>) {
        if percentage.map_or(true, |v| v >= 0.0) {
            let old_value = self.bars.get(&id);
            if old_value != Some(&percentage) {
                self.bars.insert(id.clone(), percentage);
                self.changed_bars.insert(id);
            }
        }
    }

    /// Insert a progress value by Entity (ensuring non-negative values)
    fn _insert_by_entity(&mut self, entity: Entity, percentage: Option<f32>) {
        if percentage.map_or(true, |v| v >= 0.0) { // Allow None or non-negative values
            let old_value = self.entity_bars.get(&entity);
            if old_value != Some(&percentage) {
                self.entity_bars.insert(entity, percentage);
                self.changed_entity_bars.insert(entity);
            }
        }
    }

    /// Retrieve a percentage value by ID
    pub fn get_percentage_by_id(&self, id: &str) -> Option<f32> {
        self.bars.get(id).copied().flatten()
    }

    /// Retrieve a percentage value by Entity
    pub fn get_percentage_by_entity(&self, entity: Entity) -> Option<f32> {
        self.entity_bars.get(&entity).copied().flatten()
    }

    /// Set a progress value by ID
    pub fn set_percentage_by_id(&mut self, id: &str, percentage: Option<f32>) {
        self._insert_by_id(id.to_string(), percentage);
    }

    /// Set a progress value by Entity
    pub fn set_percentage_by_entity(&mut self, entity: Entity, percentage: Option<f32>) {
        self._insert_by_entity(entity, percentage);
    }

    /// Check which IDs have changed
    pub fn get_changed_ids(&self) -> Vec<String> {
        self.changed_bars.iter().cloned().collect()
    }

    /// Check which Entities have changed
    pub fn get_changed_entities(&self) -> Vec<Entity> {
        self.changed_entity_bars.iter().cloned().collect()
    }

    /// Check if an ID exists
    pub fn exists_by_id(&self, id: &str) -> bool {
        self.bars.contains_key(id)
    }

    /// Check if an Entity exists
    pub fn exists_by_entity(&self, entity: Entity) -> bool {
        self.entity_bars.contains_key(&entity)
    }

    /// Clear change-list of ids
    pub fn clear_changes_id(&mut self) {
        self.changed_bars.clear();
    }

    /// Clear change-list of entities
    pub fn clear_changes_entity(&mut self) {
        self.changed_entity_bars.clear();
    }
}


/// Size options for progress bar.
#[derive(PartialEq, Debug)]
pub enum ProgressBarSize {
    Normal,
    Small,
    Large
}

/// Color options for progress bar.
#[derive(PartialEq, Debug)]
pub enum ProgressBarColor {
    Default,
    Primary,
    PrimaryDark,
    Secondary,
    Success,
    SuccessDark,
    Danger,
    DangerDark,
    Warning,
    WarningDark,
    Info,
    InfoDark
}

/// Represents the Famiq progress bar widget, which includes the bar and its value.
pub struct FaProgressBar;

impl<'a> FaProgressBar {
    fn _build_progress_bar(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
    ) -> Entity {
        let color = Color::srgba(0.6, 0.6, 0.6, 0.2);

        let mut style_components = BaseStyleComponents::default();
        style_components.node = attributes.node.clone();
        style_components.visibility = Visibility::Visible;
        style_components.background_color = BackgroundColor(color);
        style_components.border_color = BorderColor(color);
        style_components.border_radius = BorderRadius::all(Val::Px(5.0));

        let entity = root_node
            .commands()
            .spawn((
                style_components.clone(),
                DefaultWidgetEntity::from(style_components),
                IsFamiqProgressBar
            ))
            .id();

        insert_id_and_class(root_node, entity, &attributes.id, &attributes.class);
        entity
    }

    fn _build_progress_value(
        root_node: &'a mut EntityCommands,
        percentage: Option<f32>,
        color: &WidgetColor,
        bar_entity: Entity
    ) -> Entity {
        let mut style_components = BaseStyleComponents::default();
        style_components.node = default_progress_value_node(percentage);

        let entity = root_node
            .commands()
            .spawn((
                style_components,
                IsFamiqProgressValue,
                FamiqProgressBarEntity(bar_entity),
                ProgressValueColor(get_color(color))
            ))
            .id();

        if percentage.is_some() {
            root_node
                .commands()
                .entity(entity)
                .insert(FaProgressValuePercentage(percentage.unwrap()));
        }
        entity
    }

    pub fn new(
        attributes: &WidgetAttributes,
        root_node: &'a mut EntityCommands,
        percentage: Option<f32>,
    ) -> Entity {
        let bar = Self::_build_progress_bar(attributes, root_node);
        let value = Self::_build_progress_value(root_node, percentage, &attributes.color, bar);

        root_node.commands().entity(bar).insert(FamiqProgressValueEntity(value));
        entity_add_child(root_node, value, bar);
        bar
    }

    fn _set_to_percentage(
        commands: &mut Commands,
        node: &mut Node,
        percentage: Option<Mut<'_, FaProgressValuePercentage>>,
        new_percentage: f32,
        value_entity: Entity
    ) {
        if let Some(mut old_percentage) = percentage {
            old_percentage.0 = new_percentage;
        } else {
            commands.entity(value_entity).insert(FaProgressValuePercentage(new_percentage));
        }
        node.width = Val::Percent(new_percentage);
        node.left = Val::Px(0.0);
    }

    fn _set_to_indeterminate(
        commands: &mut Commands,
        node: &mut Node,
        percentage: Option<Mut<'_, FaProgressValuePercentage>>,
        value_entity: Entity
    ) {
        if percentage.is_some() {
            commands.entity(value_entity).remove::<FaProgressValuePercentage>();
        }
        node.width = Val::Percent(100.0);
        node.left = Val::Px(0.0);
    }

    /// Internal system to reflect on percentage changes via `FaProgressBarResource` by id.
    pub fn handle_progress_value_change_by_id(
        mut commands: Commands,
        bar_q: Query<(&FamiqWidgetId, &FamiqProgressValueEntity)>,
        mut value_q: Query<
            (
                &mut Node,
                Option<&mut FaProgressValuePercentage>
            ),
            With<IsFamiqProgressValue>
        >,
        mut progress_bar_res: ResMut<FaProgressBarResource>,
    ) {
        if !progress_bar_res.is_changed() {
            return;
        }

        for (id, value_entity) in bar_q.iter() {
            if !progress_bar_res.exists_by_id(id.0.as_str()) {
                continue;
            }

            if let Ok((mut node, percentage)) = value_q.get_mut(value_entity.0) {
                if progress_bar_res.get_changed_ids().contains(&id.0) {
                    match progress_bar_res.get_percentage_by_id(id.0.as_str()) {
                        Some(new_percentage) => {
                            Self::_set_to_percentage(
                                &mut commands,
                                &mut node,
                                percentage,
                                new_percentage,
                                value_entity.0
                            );
                        },
                        None => {
                            Self::_set_to_indeterminate(&mut commands, &mut node, percentage, value_entity.0);
                        }
                    }
                }
            }
        }
        progress_bar_res.clear_changes_id();
    }

    /// Internal system to reflect on percentage changes via `FaProgressBarResource` by entity.
    pub fn handle_progress_value_change_by_entity(
        mut commands: Commands,
        bar_q: Query<(Entity, &FamiqProgressValueEntity)>,
        mut value_q: Query<
            (
                &mut Node,
                Option<&mut FaProgressValuePercentage>
            ),
            With<IsFamiqProgressValue>
        >,
        mut progress_bar_res: ResMut<FaProgressBarResource>,
    ) {
        if !progress_bar_res.is_changed() {
            return;
        }

        for (bar_entity, value_entity) in bar_q.iter() {
            if !progress_bar_res.exists_by_entity(bar_entity) {
                continue;
            }

            if let Ok((mut node, percentage)) = value_q.get_mut(value_entity.0) {

                if progress_bar_res.get_changed_entities().contains(&bar_entity) {
                    match progress_bar_res.get_percentage_by_entity(bar_entity) {
                        Some(new_percentage) => {
                            Self::_set_to_percentage(
                                &mut commands,
                                &mut node,
                                percentage,
                                new_percentage,
                                value_entity.0
                            );
                        },
                        None => {
                            Self::_set_to_indeterminate(&mut commands, &mut node, percentage, value_entity.0);
                        }
                    }
                    progress_bar_res.clear_changes_entity();
                }
            }
        }
    }

    /// Internal system to detect new progress bars bing created.
    pub fn detect_new_progress_bar_widget_system(
        mut commands: Commands,
        mut progress_materials: ResMut<Assets<ProgressBarMaterial>>,
        bar_q: Query<
            (Entity, &ComputedNode, Option<&FamiqWidgetId>, &FamiqProgressValueEntity),
            Or<(Added<IsFamiqProgressBar>, Changed<ComputedNode>)>
        >,
        value_q: Query<(&ProgressValueColor, Option<&FaProgressValuePercentage>)>,
        mut bar_res: ResMut<FaProgressBarResource>
    ) {
        for (entity, computed_node, id, value_entity) in bar_q.iter() {
            if let Ok((value_color, percentage)) = value_q.get(value_entity.0) {

                if let Color::Srgba(value) = value_color.0 {
                    let u_blend = if percentage.is_some() {
                        0.0
                    } else {
                        1.0
                    };
                    commands
                        .entity(value_entity.0)
                        .insert(
                            MaterialNode(progress_materials.add(ProgressBarMaterial {
                                u_time: 0.0,
                                u_color: Vec3::new(value.red, value.green, value.blue),
                                u_blend,
                                u_size: computed_node.size()
                            }))
                        );
                }

                if let Some(id) = id {
                    if !bar_res.exists_by_id(id.0.as_str()) {
                        if let Some(percent) = percentage {
                            bar_res.set_percentage_by_id(id.0.as_str(), Some(percent.0));
                        } else {
                            bar_res.set_percentage_by_id(id.0.as_str(), None);
                        }
                    }
                }

                if !bar_res.exists_by_entity(entity) {
                    if let Some(percent) = percentage {
                        bar_res.set_percentage_by_entity(entity, Some(percent.0));
                    } else {
                        bar_res.set_percentage_by_entity(entity, None);
                    }
                }

                bar_res.clear_changes_id();
                bar_res.clear_changes_entity();
            }
        }
    }

    pub fn _update_progress_bar_material_u_time(
        time: Res<Time>,
        mut materials: ResMut<Assets<ProgressBarMaterial>>,
        query: Query<(&MaterialNode<ProgressBarMaterial>, Option<&FaProgressValuePercentage>)>
    ) {
        for (material_handle, percentage) in query.iter() {
            if let Some(material) = materials.get_mut(material_handle) {
                if percentage.is_none() {
                    material.u_time = -time.elapsed_secs();
                    material.u_blend = 1.0;
                } else {
                    material.u_time = 0.0;
                    material.u_blend = 0.0;
                }
            }
        }
    }
}

/// Builder for creating `FaProgressBar` widget.
pub struct FaProgressBarBuilder<'a> {
    pub attributes: WidgetAttributes,
    pub root_node: EntityCommands<'a>,
    pub percentage: Option<f32>,
}

impl<'a> FaProgressBarBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            attributes: WidgetAttributes::default(),
            root_node,
            percentage: None,
        }
    }

    /// Method to set percentage value
    pub fn percentage(mut self, percentage: f32) -> Self {
        self.percentage = Some(percentage);
        self
    }

    /// Spawn progress bar into UI World.
    pub fn build(&mut self) -> Entity {
        self._process_built_in_color_class();
        self._process_built_in_size_class();
        self._node();
        FaProgressBar::new(
            &self.attributes,
            &mut self.root_node,
            self.percentage.clone(),
        )
    }
}

impl<'a> SetWidgetAttributes for FaProgressBarBuilder<'a> {
    fn attributes(&mut self) -> &mut WidgetAttributes {
        &mut self.attributes
    }

    fn _node(&mut self) {
        self.attributes.node = default_progress_bar_node(&self.attributes.size);
        if self.attributes.default_display_changed {
            self.attributes.node.display = self.attributes.default_display;
        }
        process_spacing_built_in_class(&mut self.attributes.node, &self.attributes.class);
    }
}

/// API to create `FaProgressBar`.
pub fn fa_progress_bar<'a>(builder: &'a mut FamiqBuilder) -> FaProgressBarBuilder<'a> {
    FaProgressBarBuilder::new(
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_fa_progress_bar_systems(bar_q: Query<&IsFamiqProgressBar>) -> bool {
    !bar_q.is_empty()
}
