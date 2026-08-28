use std::process::Command;

use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, query::Query}}};

use crate::{machines::{machine::WaitingFor, machine_type::MachineType}, players::{events::{Inputed, Spawned}, player::Player, player_type::Service}, service::systems::{on_machine_input, on_service_spawned}, utils::grid::{Grid, SmoothToGrid}};

mod serivce;
mod systems;
mod service_controller;

pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_observer(
                on_service_spawned
            )
            .add_observer(
                on_machine_input
            );;
    }
}