pub mod helper;
pub mod components;
pub mod tests;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::hashbrown::HashSet;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use crate::utils::{
    entity_add_child, process_spacing_built_in_class, insert_id_and_class,
    get_embedded_asset_path
};
use crate::widgets::color::built_in_color_parser;
pub use components::*;
use helper::*;

use super::{
    DefaultWidgetEntity, ExternalStyleHasChanged,
    FamiqBuilder, FamiqWidgetId, WidgetStyle
};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    u_time: f32,
    #[uniform(1)]
    u_color: Vec3,
    #[uniform(2)]
    u_blend: f32
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
        id: &Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        size: ProgressBarSize
    ) -> Entity {
        let color = Color::srgba(0.6, 0.6, 0.6, 0.7);

        let mut node = default_progress_bar_node(&size);
        process_spacing_built_in_class(&mut node, &class);

        let entity = root_node
            .commands()
            .spawn((
                node.clone(),
                BorderColor(color),
                BackgroundColor(color),
                BorderRadius::all(Val::Px(5.0)),
                ZIndex::default(),
                Visibility::Visible,
                WidgetStyle::default(),
                ExternalStyleHasChanged(false),
                DefaultWidgetEntity::new(
                    node,
                    BorderColor(color),
                    BorderRadius::all(Val::Px(5.0)),
                    BackgroundColor(color),
                    ZIndex::default(),
                    Visibility::Visible
                ),
                IsFamiqProgressBar
            ))
            .id();

        insert_id_and_class(root_node, entity, id, &class);
        entity
    }

    fn _build_progress_value(
        root_node: &'a mut EntityCommands,
        percentage: Option<f32>,
        color: Color,
        bar_entity: Entity
    ) -> Entity {
        let entity = root_node
            .commands()
            .spawn((
                default_progress_value_node(percentage),
                BorderColor::default(),
                BackgroundColor::default(),
                BorderRadius::default(),
                ZIndex::default(),
                Visibility::Inherited,
                IsFamiqProgressValue,
                FamiqProgressBarEntity(bar_entity),
                ProgressValueColor(color)
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
        id: Option<String>,
        class: Option<String>,
        root_node: &'a mut EntityCommands,
        percentage: Option<f32>,
        size: ProgressBarSize,
        color: Color
    ) -> Entity {
        let bar = Self::_build_progress_bar(&id, class, root_node, size);
        let value = Self::_build_progress_value(root_node, percentage, color, bar);

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
        bar_q: Query<(Entity, Option<&FamiqWidgetId>, &FamiqProgressValueEntity), Added<IsFamiqProgressBar>>,
        value_q: Query<(&ProgressValueColor, Option<&FaProgressValuePercentage>)>,
        mut bar_res: ResMut<FaProgressBarResource>
    ) {
        for (entity, id, value_entity) in bar_q.iter() {
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
                                u_blend
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
    pub id: Option<String>,
    pub class: Option<String>,
    pub root_node: EntityCommands<'a>,
    pub percentage: Option<f32>,
    pub color: Option<Color>

}

impl<'a> FaProgressBarBuilder<'a> {
    pub fn new(root_node: EntityCommands<'a>) -> Self {
        Self {
            id: None,
            class: None,
            root_node,
            percentage: None,
            color: None
        }
    }

    fn _process_built_in_classes(&mut self) -> ProgressBarSize {
        let mut use_color = ProgressBarColor::Default;
        let mut use_size = ProgressBarSize::Normal;

        if let Some(class) = self.class.as_ref() {
            let class_split: Vec<&str> = class.split_whitespace().collect();

            for class_name in class_split {
                match class_name {
                    // Check for colors
                    "is-primary" => use_color = ProgressBarColor::Primary,
                    "is-primary-dark" => use_color = ProgressBarColor::PrimaryDark,
                    "is-secondary" => use_color = ProgressBarColor::Secondary,
                    "is-danger" => use_color = ProgressBarColor::Danger,
                    "is-danger-dark" => use_color = ProgressBarColor::DangerDark,
                    "is-success" => use_color = ProgressBarColor::Success,
                    "is-success-dark" => use_color = ProgressBarColor::SuccessDark,
                    "is-warning" => use_color = ProgressBarColor::Warning,
                    "is-warning-dark" => use_color = ProgressBarColor::WarningDark,
                    "is-info" => use_color = ProgressBarColor::Info,
                    "is-info-dark" => use_color = ProgressBarColor::InfoDark,

                    // Check for sizes
                    "is-small" => use_size = ProgressBarSize::Small,
                    "is-large" => use_size = ProgressBarSize::Large,
                    "is-normal" => use_size = ProgressBarSize::Normal,
                        _ => (),
                }
            }
        }
        if self.color.is_none() {
            self.color = Some(get_progress_value_color(&use_color));
        }
        use_size
    }

    /// Method to add class to progress bar.
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Method to add id to progress bar.
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Method to set percentage value
    pub fn percentage(mut self, percentage: f32) -> Self {
        self.percentage = Some(percentage);
        self
    }

    /// Method to set color.
    pub fn set_color(mut self, color: &str) -> Self {
        self.color = built_in_color_parser(color);
        self
    }

    /// Spawn progress bar into UI World.
    pub fn build(&mut self) -> Entity {
        let use_size = self._process_built_in_classes();

        FaProgressBar::new(
            self.id.clone(),
            self.class.clone(),
            &mut self.root_node,
            self.percentage.clone(),
            use_size,
            self.color.unwrap()
        )
    }
}

/// API to create `FaProgressBar`.
pub fn fa_progress_bar<'a>(builder: &'a mut FamiqBuilder) -> FaProgressBarBuilder<'a> {
    FaProgressBarBuilder::new(
        builder.ui_root_node.reborrow()
    )
}

pub fn can_run_fa_progress_bar_systems(
    bar_q: Query<&IsFamiqProgressBar>
) -> bool {
    bar_q.iter().count() > 0
}
