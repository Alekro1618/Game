use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, query::Query}}};

use crate::{enemy::{enemy::{EnemyList, SpawnRandomEnemy}, systems::{on_enemy_spawned, on_spawn_random}}, machines::machine::WaitingFor, players::{events::Spawned, player::Player, player_type::Enemy}, utils::grid::{Grid, SmoothToGrid}};

pub mod enemy;
pub mod enemy_controller;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_observer(
                on_enemy_spawned
            )
            .add_observer(
                on_spawn_random
            );
    }
}