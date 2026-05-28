use std::process::Command;

use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, function_system::{F2, F3}, query::Query}}};

use crate::{machines::machine::WaitingFor, players::{player::Player, player_type::Service, spawner::SpawnedService}, service::systems::{move_service, on_service_spawned}, utils::grid::{Grid, SmoothToGrid}};

mod serivce;
mod systems;
mod service_controller;

pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_systems::<F3<Commands, Query<(EntityId, &Service, &SmoothToGrid, &Player),  Without<WaitingFor>>, &Grid>>(
                ScheduleLabel::FixedUpdate, 
                move_service
            )
            .add_observer::<SpawnedService, F3<On<SpawnedService>, Commands, &Grid>>(
                on_service_spawned
            );
    }
}