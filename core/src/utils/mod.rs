use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::systems::{query::Query}, math::transform::Transform};

use crate::utils::{grid::{Grid, LockToGrid, SmoothToGrid, draw_grid, point_to_transform, smooth_point_to_transform, update_grid}, path::{Path, traverse_path}};

pub mod grid;
pub mod path;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app.add_systems(
            ScheduleLabel::PreUpdate, 
            smooth_point_to_transform
        );
        app.add_systems(
            ScheduleLabel::PreUpdate, 
            point_to_transform
        );
        app.add_systems(
            ScheduleLabel::FixedUpdate, 
            update_grid
        );
        app.add_systems(
            ScheduleLabel::FixedUpdate, 
            traverse_path
        );
        app.add_systems(
            ScheduleLabel::Update, 
            draw_grid
        );
    }
}