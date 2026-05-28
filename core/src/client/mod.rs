use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, function_system::{F2, F3, F4}, query::Query}}};
use macroquad::{file::load_file, texture::load_image};

use crate::{client::{client::ClientList, queue::{MoveQueue, SpawnRandomClient}, systems::{move_client_until_queue, on_client_spawned, on_move_queue, on_spawn_random}}, machines::machine::WaitingFor, players::{player::Player, player_type::Client, spawner::{SpawnClient, SpawnedClient}}, utils::grid::{Grid, SmoothToGrid}};

pub mod queue;
mod systems;
pub mod client;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_observer::<SpawnedClient, F3<On<SpawnedClient>, Commands, &Grid>>(
                on_client_spawned
            )
            .add_systems::<F3<Commands, Query<(EntityId, &Client, &SmoothToGrid, &Player),  Without<WaitingFor>>, &Grid>>(
                ScheduleLabel::FixedUpdate, move_client_until_queue
            )
            .add_observer::<MoveQueue, F3<On<MoveQueue>, Commands, Query<(EntityId, &Client, &Player)>>>(
                on_move_queue
            )
            .add_observer::<SpawnRandomClient, F3<On<SpawnRandomClient>, Commands, &mut ClientList>>(
                on_spawn_random
            );
    }
}