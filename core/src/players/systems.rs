use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, message::message_reader::MessageReader, observer::system_param::On, systems::{commands::{self, Commands}, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::{resources::GameSpeed, *}};
use engine_macros::{Component, Event};

use crate::{algos::path_finding::find_path, machines::{machine::{DisruptMachine, InteractWithMachine}, machine_register::MachineRegister}, players::{player::{Command, Disrupt, Interact, PendingAction, Player, PlayerAction, PlayerSpeed, SetPath, StepEnd}, player_register::{self, PlayerRegister, RegisterPlayer}, player_state::PlayerState, player_type::{Client, Enemy, PlayerType, Service}, spawner::{DespawnEnemy, SpawnClient, SpawnEnemy, SpawnService, SpawnedClient, SpawnedEnemy, SpawnedService}}, utils::grid::{self, Grid, Point, SmoothToGrid}};



pub fn advance_smooth(
    mut commands : Commands,
    query : Query<(EntityId, &PlayerSpeed, &mut SmoothToGrid)>,
    speed : &GameSpeed
) {
    for (entity, player_speed, mut smooth) in query {
        smooth.update(player_speed.0 * speed.relative());
        if smooth.is_end() {
            commands.trigger(StepEnd(entity));
        }
    }
}


pub fn on_step_end(
    on : On<StepEnd>,
    query : Query<&mut PlayerState>,
) {
    let step_end = on.event();
    let Ok(mut player_state) = query.get_mut(step_end.0) else {
        panic!("Player who end stepping doesn't exist");
    };

    player_state.set_idle();
}



/// Handles players with `PendingAction` component.
/// If `PlayerAction` exists then it will remove `PendingAction` and proceed with either
/// - `PlayerAction::WalkTo` adds `SmoothToGrid` from current position to target position
/// - `PlayerAction::Interact` adds `Interacting` at curret position
pub fn handle_pending_players(
    on : On<PendingAction>,
    mut command : Commands,
    query : Query<(&mut Player, &mut PlayerState)>
) {
    let entity = on.event().0;

    let Ok((mut player, mut player_state)) = query.get_mut(entity) else {
        panic!("Player who we pend action doesn't exist");
    };

    //We don't handle it if player is currently in walking state or repearing machine
    if player_state.is_walking() || player_state.is_repearing() {
        return;
    }

    let current_pos = player.point;
    //Procceed with next action
    player.next_action();
    
    if let Some(action) = player.current_action {
        match action {
            PlayerAction::WalkTo(point) => {
                player_state.set_walking();
                let point = point.clone();
                command.add(entity, 
                    (
                        SmoothToGrid::new(
                        grid::GridName::Players, 
                        current_pos,
                        point),
                    )
                );
                player.point = point;
            },
            PlayerAction::Interact(with, item) => {
                player_state.set_active();
                command.trigger(InteractWithMachine(entity, with, item));
            },
            PlayerAction::Direction(_) => {

            },
            PlayerAction::Disrupt(who) => {
                command.trigger(DisruptMachine(entity, who));
            },
        }
    }
}

pub fn on_interact(
    on : On<Interact>,
    query : Query<&mut Player>,
    machine_register : &MachineRegister,
    player_register : &PlayerRegister
) {
    let interact = on.event();
    let entity_id = player_register.get(&interact.player).unwrap();
    let mut player = query.get_mut(entity_id).unwrap();

    player.interact(machine_register.get(&interact.machine).unwrap(), interact.item);
}


pub fn on_disrupt(
    on : On<Disrupt>,
    query : Query<&mut Player>,
    machine_register : &MachineRegister,
    player_register : &PlayerRegister
) {
    let disrupt = on.event();
    let entity_id = player_register.get(&disrupt.player).unwrap();
    let mut player = query.get_mut(entity_id).unwrap();

    player.disrupt(machine_register.get(&disrupt.machine).unwrap());
}

pub fn on_set_path(
    on : On<SetPath>,
    query : Query<&mut Player>,
    grid : &Grid,
    player_register : &PlayerRegister
) {
    let set_path = on.event();
    let entity_id = player_register.get(&set_path.player).unwrap();
    let mut player = query.get_mut(entity_id).unwrap();
    let path = find_path(grid, set_path.from, set_path.to);
    player.add_path(path);
}

pub fn register_players(
    mut commands : Commands,
    query_clients : Query<(EntityId, &Client)>,
    query_service : Query<(EntityId, &Service)>,
    query_enemies : Query<(EntityId, &Enemy)>,
) {
    let mut player_register = PlayerRegister::new();
    for (entity, &client) in query_clients {
        player_register.insert(PlayerType::Client(client), entity);
    }
    for (entity, &service) in query_service {
        player_register.insert(PlayerType::Service(service), entity);
    }
    for (entity, &enemy) in query_enemies {
        player_register.insert(PlayerType::Enemy(enemy), entity);
    }
    commands.insert(player_register);
}


pub fn on_client_spawn(
    on : On<SpawnClient>,
    mut commands : Commands,
) {
    let spawn= on.event();
    let client = spawn.0;
    commands.queue(move |world| {
        let entity_id = world.spawn((client,));
        world.trigger_ref(&mut RegisterPlayer(PlayerType::Client(client), entity_id));
        world.trigger_ref(&mut SpawnedClient(entity_id, client));
        
    });
}

pub fn on_service_spawn(
    on : On<SpawnService>,
    mut commands : Commands,
) {
    let spawn= on.event();
    let service = spawn.0;
    commands.queue(move |world| {
        let entity_id = world.spawn((service,));
        world.trigger_ref(&mut RegisterPlayer(PlayerType::Service(service), entity_id));
        world.trigger_ref(&mut SpawnedService(entity_id, service));
    });
}

pub fn on_enemy_spawn(
    on : On<SpawnEnemy>,
    mut commands : Commands,
) {
    let spawn= on.event();
    let enemy = spawn.0;
    commands.queue(move |world| {
        let entity_id = world.spawn((enemy,));
        world.trigger_ref(&mut RegisterPlayer(PlayerType::Enemy(enemy), entity_id));
        world.trigger_ref(&mut SpawnedEnemy(entity_id, enemy));
    });
}

pub fn on_enemy_despawn(
    on : On<DespawnEnemy>,
    mut commands : Commands
) {
    // let despawn= on.event();
    // let enemy = despawn.0;
    // commands.queue(move |world| {
    //     let entity_id = world.despawn();
    //     world.trigger_ref(&mut RegisterPlayer(PlayerType::Enemy(enemy), entity_id));
    //     world.trigger_ref(&mut SpawnedEnemy(entity_id, enemy));
    // });
}

pub fn on_register_player(
    on : On<RegisterPlayer>,
    mut player_register : Mut<'_, PlayerRegister>
) {
    let register = on.event();
    player_register.insert(register.0, register.1);
}