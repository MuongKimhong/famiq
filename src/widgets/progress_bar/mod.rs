pub mod helper;
pub mod tests;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::hashbrown::HashSet;
use crate::utils::{process_spacing_built_in_class, insert_id_and_class};
use helper::*;

use super::{
    DefaultWidgetEntity, ExternalStyleHasChanged, FamiqWidgetClasses, FamiqWidgetId, WidgetStyle
};

/// Animation speed, defined by `speed = INDETERMINATE_SPEED_FACTOR * bar_width`.
const INDETERMINATE_SPEED_FACTOR: f32 =  0.5; // Move 50% of the bar width per second

/// Progress value width when set as indeterminate.
const INDETERMINATE_WIDTH: f32 = 40.0; // 40% of bar width

/// Marker component for identifying an entity as a Famiq Progress bar.
#[derive(Component)]
pub struct IsFamiqProgressBar;

/// Marker component for identifying an entity as a Famiq Progress bar's value.
#[derive(Component)]
pub struct IsFamiqProgressValue;

/// Component storing the progress bar entity associated with its value.
#[derive(Component)]
pub struct FamiqProgressBarEntity(pub Entity);

/// Component storing the progress value entity associated with its bar.
#[derive(Component)]
pub struct FamiqProgressValueEntity(pub Entity);

/// Component storing direction of indeterminate animation, 1.0 for right, -1.0 for left.
#[derive(Component)]
pub struct IndeterminateDirection(pub f32); // 1.0 right, -1.0 left

/// Component storing percentage of a progress bar.
#[derive(Component)]
pub struct FaProgressValuePercentage(pub f32);

#[derive(Component)]
pub struct PercentageFlag(pub Option<f32>);

/// Indeterminate animation timer, moving at 120 fps.
#[derive(Resource)]
pub struct IndeterminateAnimationTimer {
    pub timer: Timer,
}

impl Default for IndeterminateAnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / 120.0, TimerMode::Repeating), // 120 fps
        }
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
#[derive(Component, PartialEq, Debug)]
pub enum ProgressBarSize {
    Normal,
    Small,
    Large
}

/// Color options for progress bar.
#[derive(Component, PartialEq, Debug)]
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

impl FaProgressBar {
    fn _build_progress_value(
        id: Option<String>,
        commands: &mut Commands,
        percentage: Option<f32>,
        color: &ProgressBarColor,
        bar_entity: Entity
    ) -> Entity {

        let entity = commands
            .spawn((
                default_progress_value_node(percentage),
                get_progress_value_border_color(color),
                get_progress_value_background_color(color),
                BorderRadius::all(Val::Px(5.0)),
                ZIndex::default(),
                Visibility::Inherited,
                IsFamiqProgressValue,
                FamiqProgressBarEntity(bar_entity),
                WidgetStyle::default(),
                ExternalStyleHasChanged(false),
                DefaultWidgetEntity::new(
                    default_progress_value_node(percentage),
                    get_progress_value_border_color(color),
                    BorderRadius::all(Val::Px(5.0)),
                    get_progress_value_background_color(color),
                    ZIndex::default(),
                    Visibility::Inherited
                )
            ))
            .id();

        if let Some(id) = id {
            commands
                .entity(entity)
                .insert(FamiqWidgetId(format!("{id}_progress_value")));
        }
        if percentage.is_none() {
            commands
                .entity(entity)
                .insert(IndeterminateDirection(1.0));
        }
        else {
            commands
                .entity(entity)
                .insert(FaProgressValuePercentage(percentage.unwrap()));
        }
        entity
    }

    /// Internal system to move progress value when set to indeterminate mode.
    pub fn move_progress_value_as_indeterminate_system(
        time: Res<Time>,
        bar_q: Query<(&Node, &ComputedNode), With<IsFamiqProgressBar>>,
        mut value_q: Query<
            (&mut Node, &ComputedNode, &mut IndeterminateDirection, &FamiqProgressBarEntity),
            Without<IsFamiqProgressBar>
        >,
        mut animation: ResMut<IndeterminateAnimationTimer>
    ) {
        animation.timer.tick(time.delta());

        if !animation.timer.just_finished() {
            return;
        }
        for (mut value_node, value_computed_node, mut direction, bar_entity) in value_q.iter_mut() {

            if let Ok((_, bar_computed_node)) = bar_q.get(bar_entity.0) {

                let current_left = match value_node.left {
                    Val::Px(val) => val,
                    _ => 0.0,
                };
                let bar_width = bar_computed_node.size().x;
                let speed = bar_width * INDETERMINATE_SPEED_FACTOR;
                let pixels_per_frame = speed * time.delta_secs();
                let new_left = current_left + direction.0 * pixels_per_frame;

                let max_left = bar_computed_node.size().x - value_computed_node.size().x;
                let min_left = 0.0;

                if new_left >= max_left {
                    direction.0 = -1.0;
                } else if new_left <= min_left {
                    direction.0 = 1.0;
                }
                value_node.left = Val::Px(new_left);
            }
        }
    }

    fn _set_to_percentage(
        commands: &mut Commands,
        node: &mut Node,
        direction: Option<&IndeterminateDirection>,
        percentage: Option<Mut<'_, FaProgressValuePercentage>>,
        new_percentage: f32,
        value_entity: Entity
    ) {
        if direction.is_some() {
            commands.entity(value_entity).remove::<IndeterminateDirection>();
        }
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
        direction: Option<&IndeterminateDirection>,
        percentage: Option<Mut<'_, FaProgressValuePercentage>>,
        value_entity: Entity
    ) {
        if percentage.is_some() {
            commands.entity(value_entity).remove::<FaProgressValuePercentage>();
        }
        if direction.is_none() {
            commands.entity(value_entity).insert(IndeterminateDirection(1.0));
        }
        node.width = Val::Percent(INDETERMINATE_WIDTH);
        node.left = Val::Px(0.0);
    }

    /// Internal system to reflect on percentage changes via `FaProgressBarResource` by id.
    pub fn handle_progress_value_change_by_id(
        mut commands: Commands,
        bar_q: Query<(&FamiqWidgetId, &FamiqProgressValueEntity)>,
        mut value_q: Query<
            (
                &mut Node,
                Option<&IndeterminateDirection>,
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

            if let Ok((mut node, direction, percentage)) = value_q.get_mut(value_entity.0) {
                if progress_bar_res.get_changed_ids().contains(&id.0) {
                    match progress_bar_res.get_percentage_by_id(id.0.as_str()) {
                        Some(new_percentage) => {
                            Self::_set_to_percentage(
                                &mut commands,
                                &mut node,
                                direction,
                                percentage,
                                new_percentage,
                                value_entity.0
                            );
                        },
                        None => {
                            Self::_set_to_indeterminate(&mut commands, &mut node, direction, percentage, value_entity.0);
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
                Option<&IndeterminateDirection>,
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

            if let Ok((mut node, direction, percentage)) = value_q.get_mut(value_entity.0) {

                if progress_bar_res.get_changed_entities().contains(&bar_entity) {
                    match progress_bar_res.get_percentage_by_entity(bar_entity) {
                        Some(new_percentage) => {
                            Self::_set_to_percentage(
                                &mut commands,
                                &mut node,
                                direction,
                                percentage,
                                new_percentage,
                                value_entity.0
                            );
                        },
                        None => {
                            Self::_set_to_indeterminate(&mut commands, &mut node, direction, percentage, value_entity.0);
                        }
                    }
                    progress_bar_res.clear_changes_entity();
                }
            }
        }
    }

    /// Internal system to detect new progress bars bing created.
    pub fn detect_new_progress_bar_widget_system(
        bar_q: Query<(Entity, Option<&FamiqWidgetId>, &FamiqProgressValueEntity), Added<IsFamiqProgressBar>>,
        value_q: Query<Option<&FaProgressValuePercentage>>,
        mut bar_res: ResMut<FaProgressBarResource>
    ) {
        for (entity, id, value_entity) in bar_q.iter() {
            if let Ok(percentage) = value_q.get(value_entity.0) {

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

    pub fn _detect_fa_progress_bar_creation_system(
        mut commands: Commands,
        bar_q: Query<
            (Entity, &PercentageFlag, &ProgressBarColor, &ProgressBarSize, Option<&FamiqWidgetId>, Option<&FamiqWidgetClasses>),
            Added<IsFamiqProgressBar>
        >,
        mut bar_res: ResMut<FaProgressBarResource>
    ) {
        for (entity, percentage_flag, color, size, id, class) in bar_q.iter() {
            let id_ref = id.map(|s| s.0.clone());
            let class_ref = class.map(|s| s.0.clone());
            let value_entity = FaProgressBar::_build_progress_value(id_ref, &mut commands, percentage_flag.0, color, entity);

            if let Some(id) = id {
                if !bar_res.exists_by_id(id.0.as_str()) {
                    if let Some(percent) = percentage_flag.0 {
                        bar_res.set_percentage_by_id(id.0.as_str(), Some(percent));
                    } else {
                        bar_res.set_percentage_by_id(id.0.as_str(), None);
                    }
                }
            }
            if !bar_res.exists_by_entity(entity) {
                if let Some(percent) = percentage_flag.0 {
                    bar_res.set_percentage_by_entity(entity, Some(percent));
                } else {
                    bar_res.set_percentage_by_entity(entity, None);
                }
            }

            let bar_color = Color::srgba(0.6, 0.6, 0.6, 0.95);
            let mut node = default_progress_bar_node(&size);
            process_spacing_built_in_class(&mut node, &class_ref);

            commands
                .entity(entity)
                .add_child(value_entity)
                .insert((
                    node.clone(),
                    BorderColor(bar_color),
                    BackgroundColor(bar_color),
                    BorderRadius::all(Val::Px(5.0)),
                    ZIndex::default(),
                    Visibility::Visible,
                    WidgetStyle::default(),
                    ExternalStyleHasChanged(false),
                    DefaultWidgetEntity::new(
                        node,
                        BorderColor(bar_color),
                        BorderRadius::all(Val::Px(5.0)),
                        BackgroundColor(bar_color),
                        ZIndex::default(),
                        Visibility::Visible
                    ),
                    FamiqProgressValueEntity(value_entity)
                ));
        }
    }
}

/// Builder for creating `FaProgressBar` widget.
pub struct FaProgressBarBuilder<'w, 's> {
    pub id: Option<String>,
    pub class: Option<String>,
    pub commands: Commands<'w, 's>,
    pub percentage: Option<f32>,
    pub color: Option<Color>

}

impl<'w, 's> FaProgressBarBuilder<'w, 's> {
    pub fn new(commands: Commands<'w, 's>) -> Self {
        Self {
            id: None,
            class: None,
            commands,
            percentage: None,
            color: None
        }
    }

    fn _process_built_in_classes(&self) -> (ProgressBarColor, ProgressBarSize) {
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
        (use_color, use_size)
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

    /// Spawn progress bar into UI World.
    pub fn build(&mut self) -> Entity {
        let (color, size) = self._process_built_in_classes();
        let entity = self.commands.spawn((
            IsFamiqProgressBar,
            PercentageFlag(self.percentage),
            color,
            size
        )).id();
        insert_id_and_class(&mut self.commands, entity, &self.id, &self.class);
        entity
    }
}

/// API to create `FaProgressBar`.
pub fn fa_progress_bar<'w, 's>(commands: &'w mut Commands) -> FaProgressBarBuilder<'w, 's>
where
    'w: 's
{
    FaProgressBarBuilder::new(commands.reborrow())
}

/// Internal function to check if `FaProgressBar`'s indeterminate system can run.
pub fn can_move_progress_value_as_indeterminate_system(
    value_q: Query<&IndeterminateDirection>
) -> bool {
    value_q.iter().count() > 0
}

/// Internal function to check if `handle_progress_value_change` system can run.
pub fn can_run_handle_progress_value_change(
    bar_q: Query<&IsFamiqProgressBar>
) -> bool {
    bar_q.iter().count() > 0
}
