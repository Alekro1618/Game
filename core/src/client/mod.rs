use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, query::Query}}};

use crate::{client::{client::ClientList, queue::{InQueue, MoveQueue, Queue, SpawnRandomClient}, systems::{ on_cash_register_input, on_client_spawned, on_completed_order, on_move_queue, on_spawn_random}}, machines::{machine::WaitingFor, machine_type::MachineType}, order::order::CompletedOrder, players::{events::{Inputed, Spawned}, player::Player, player_type::Client}, utils::grid::{Grid, SmoothToGrid}};

pub mod queue;
mod systems;
pub mod client;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .register_type::<Queue>()
            .add_observer(
                on_client_spawned
            )
            .add_observer(
                on_spawn_random
            )
            .add_observer(
                on_cash_register_input
            )
            .add_observer(
                on_completed_order
            )
            .add_observer(
                on_move_queue
            );
    }
}