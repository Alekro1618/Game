use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, function_system::{F2, F3}, query::Query}}};

use crate::{enemy::{enemy::{EnemyList, SpawnRandomEnemy}, systems::{move_enemy, on_enemy_spawned, on_spawn_random}}, machines::machine::WaitingFor, players::{player::Player, player_type::Enemy, spawner::SpawnedEnemy}, utils::grid::{Grid, SmoothToGrid}};

pub mod enemy;
pub mod enemy_controller;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
             .add_systems::<F3<Commands, Query<(EntityId, &Enemy, &SmoothToGrid, &Player),  Without<WaitingFor>>, &Grid>>(
                ScheduleLabel::FixedUpdate, 
                move_enemy
            )
            .add_observer::<SpawnedEnemy, F3<On<SpawnedEnemy>, Commands, &Grid>>(
                on_enemy_spawned
            )
            .add_observer::<SpawnRandomEnemy, F3<On<SpawnRandomEnemy>, Commands, &mut EnemyList>>(
                on_spawn_random
            );;
    }
}