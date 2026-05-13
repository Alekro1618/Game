use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::systems::{function_system::F2, query::Query}, math::transform::Transform};

use crate::utils::{grid::{Grid, LockToGrid, SmoothToGrid, smooth_point_to_transform, update_grid}, path::{Path, traverse_path}};

pub mod grid;
pub mod path;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app:  &mut engine::app::App) {
        app.add_systems::<F2<Query<(&mut Transform, &SmoothToGrid)>, &Grid>>(ScheduleLabel::PreUpdate, smooth_point_to_transform);
        //app.add_systems::<F2<Query<(&mut Transform, &LockToGrid)>, &Grid>>(schedule, systems)
        app.add_systems::<F2<&mut Grid, Query<&LockToGrid>>>(ScheduleLabel::PostUpdate, update_grid);
        app.add_systems::<F2<Query<(&mut Transform, &Path)>, &Grid>>(ScheduleLabel::FixedUpdate, traverse_path);
    }
}