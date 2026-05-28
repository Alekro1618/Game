use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, function_system::{F1, F2, F3, F4}, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::resources::GameSpeed};

use crate::{machines::machine_register::MachineRegister, players::{player::{Command, Disrupt, Interact, PendingAction, Player, PlayerSpeed, SetPath, StepEnd}, player_register::{PlayerRegister, RegisterPlayer}, player_state::PlayerState, player_type::{Client, Enemy, Service}, spawner::{SpawnClient, SpawnEnemy, SpawnService}, systems::{advance_smooth, handle_pending_players, on_client_spawn, on_disrupt, on_enemy_spawn, on_interact, on_register_player, on_service_spawn, on_set_path, on_step_end, register_players}}, utils::grid::{Grid, SmoothToGrid}};

pub mod player;
mod systems;
pub mod player_register;
pub mod spawner;
pub mod player_type;
pub mod player_state;

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
        .add_message::<Command>()
        .add_systems::<F4<Commands, Query<(EntityId, &Client)>, Query<(EntityId, &Service)>, Query<(EntityId, &Enemy)>>>(
            ScheduleLabel::Startup, 
            register_players
        )
        .add_observer::<StepEnd, F2<On<StepEnd>, Query<&mut PlayerState>>>(
            on_step_end
        )
        .add_systems::<F3<Commands, Query<(EntityId, &PlayerSpeed, &mut SmoothToGrid)>, &GameSpeed>>(
            ScheduleLabel::FixedUpdate, 
            advance_smooth
        )
        .add_observer::<PendingAction, F3<On<PendingAction>, Commands, Query<(&mut Player, &mut PlayerState)>>>(
            handle_pending_players
        )
        .add_observer::<SpawnClient, F2<On<SpawnClient>, Commands>>(
            on_client_spawn
        )
        .add_observer::<SpawnService, F2<On<SpawnService>, Commands>>(
            on_service_spawn
        )
        .add_observer::<SpawnEnemy, F2<On<SpawnEnemy>, Commands>>(
            on_enemy_spawn
        )
        .add_observer::<Interact, F4<On<Interact>, Query<&mut Player>, &MachineRegister, &PlayerRegister>>(
            on_interact
        )
        .add_observer::<Disrupt, F4<On<Disrupt>, Query<&mut Player>, &MachineRegister, &PlayerRegister>>(
            on_disrupt
        )
        .add_observer::<SetPath, F4<On<SetPath>, Query<&mut Player>, &Grid, &PlayerRegister>>(
            on_set_path
        )
        .add_observer::<RegisterPlayer, F2<On<RegisterPlayer>, &mut PlayerRegister>>(
            on_register_player
        );
    }
}