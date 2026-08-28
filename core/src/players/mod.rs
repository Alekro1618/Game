use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::resources::GameSpeed};

use crate::{instructions::{events::{ExecuteInstruction, ExecuteStep}, manager::InstructionManager}, machines::{events::{Inputed, Outputed}, register::MachineRegister}, players::{events::{Despawn, Spawn}, player::{ Player, PlayerSpeed}, player_register::{PlayerRegister, RegisterPlayer}, player_state::PlayerState, player_type::{Client, Enemy, Service}, systems::{advance_smooth, on_client_despawn, on_client_spawn, on_enemy_despawn, on_enemy_spawn, on_execute_instruction, on_execute_step, on_inputed, on_outputed, on_register_player, on_service_despawn, on_service_spawn, register_players}}, utils::grid::{Grid, SmoothToGrid}};

pub mod player;
mod systems;
pub mod events;
pub mod player_register;
pub mod player_type;
pub mod player_state;
pub mod player_actions;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
        .add_systems(
            ScheduleLabel::Startup, 
            register_players
        )
        .add_systems(
            ScheduleLabel::FixedUpdate, 
            advance_smooth
        )
        .add_observer(
            on_execute_instruction
        )
        .add_observer(
            on_execute_step
        )
        .add_observer(
            on_client_spawn
        )
        .add_observer(
            on_service_spawn
        )
        .add_observer(
            on_enemy_spawn
        )
        .add_observer(
            on_client_despawn
        )
        .add_observer(
            on_service_despawn
        )
        .add_observer(
            on_enemy_despawn
        )
        .add_observer(
            on_register_player
        )
        .add_observer(
            on_inputed
        )
        .add_observer(
            on_outputed
        );
    }
}

