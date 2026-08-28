use std::slice::Iter;

use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, message::message_reader::MessageReader, observer::system_param::On, systems::{commands::{self, Commands}, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::{resources::GameSpeed, *}};
use engine_macros::{Component, Event};

use crate::{actions::{action::{Action, IntoBoxedAction}, events::QueueActions}, algos::path_finding::find_path, instructions::{components::{InstructionLabel::WalkTo, Step::{self, WaitUntil}}, events::{ExecuteInstruction, ExecuteStep}, manager::InstructionManager}, items::{AddToInventory, Inventory, RemoveFromInventory}, machines::{events::{Inputed, Outputed}, register::MachineRegister}, players::{events::{self, Despawn, Despawned, Spawn, Spawned}, player::{ Player, PlayerSpeed}, player_actions::{self, add_path}, player_register::{self, PlayerRegister, RegisterPlayer}, player_state::{self, PlayerState}, player_type::{Client, Enemy, PlayerType, Service}}, utils::grid::{self, Grid, GridName::{self, Players}, Point, SmoothToGrid}};



pub fn advance_smooth(
    query : Query<(&PlayerSpeed, &mut SmoothToGrid)>,
    speed : &GameSpeed,
    grid : &Grid,
) {
    for (player_speed, mut smooth) in query {
        smooth.update(player_speed.0 * speed.relative());
    }
}

pub fn register_players(
    mut commands : Commands,
    query_clients : Query<(EntityId, &Client, &SmoothToGrid)>,
    query_service : Query<(EntityId, &Service, &SmoothToGrid)>,
    query_enemies : Query<(EntityId, &Enemy, &SmoothToGrid)>,
) {
    let mut player_register = PlayerRegister::new();
    for (entity, &client, smooth) in query_clients {
        player_register.insert(PlayerType::Client(client), entity);
        player_register.update_future(PlayerType::Client(client), smooth.start);
    }
    for (entity, &service, smooth) in query_service {
        player_register.insert(PlayerType::Service(service), entity);
        player_register.update_future(PlayerType::Service(service), smooth.start);
    }
    for (entity, &enemy, smooth ) in query_enemies {
        player_register.insert(PlayerType::Enemy(enemy), entity);
        player_register.update_future(PlayerType::Enemy(enemy), smooth.start);
    }
    commands.insert(player_register);
}

pub fn on_client_spawn(
    on : On<Spawn<Client>>,
    mut commands : Commands,
) {
    let spawn= on.event();
    let client = spawn.0;
    let point = spawn.1;
    commands.queue(move |world| {
        let entity_id = world.spawn((client,));
        world.trigger_ref(&mut RegisterPlayer(PlayerType::Client(client), entity_id, point));
        world.trigger_ref(&mut Spawned(entity_id, client));
    });
}

pub fn on_service_spawn(
    on : On<Spawn<Service>>,
    mut commands : Commands,
) {
    let spawn= on.event();
    let service = spawn.0;
    let point = spawn.1;
    commands.queue(move |world| {
        let entity_id = world.spawn((service,));
        world.trigger_ref(&mut RegisterPlayer(PlayerType::Service(service), entity_id, point));
        world.trigger_ref(&mut Spawned(entity_id, service));
    });
}

pub fn on_enemy_spawn(
    on : On<Spawn<Enemy>>,
    mut commands : Commands,
) {
    let spawn= on.event();
    let enemy = spawn.0;
    let point = spawn.1;
    commands.queue(move |world| {
        let entity_id = world.spawn((enemy,));
        world.trigger_ref(&mut RegisterPlayer(PlayerType::Enemy(enemy), entity_id, point));
        world.trigger_ref(&mut Spawned(entity_id, enemy));
    });
}

pub fn on_client_despawn(
    on : On<Despawn<Client>>, 
    mut commands : Commands,
    mut player_register : Mut<'_, PlayerRegister>,
) {
    let despawn= on.event();
    let client = despawn.0;
    let entity_id = player_register.take(&PlayerType::Client(client));
    commands.queue(move |world| {
        if let Some(entity) = entity_id {
            world.trigger_ref(&mut Despawned(entity, client));
            world.despawn(entity);
        }
    });
}

pub fn on_service_despawn(
    on : On<Despawn<Service>>, 
    mut commands : Commands,
    mut player_register : Mut<'_, PlayerRegister>,
) {
    let despawn= on.event();
    let service = despawn.0;
    let entity_id = player_register.take(&PlayerType::Service(service));
    commands.queue(move |world| {
        if let Some(entity) = entity_id {
            world.trigger_ref(&mut Despawned(entity, service));
            world.despawn(entity);
        }
    });
}

pub fn on_enemy_despawn(
    on : On<Despawn<Enemy>>, 
    mut commands : Commands,
    mut player_register : Mut<'_, PlayerRegister>,
) {
    let despawn= on.event();
    let enemy = despawn.0;
    let entity_id = player_register.take(&PlayerType::Enemy(enemy));
    commands.queue(move |world| {
        if let Some(entity) = entity_id {
            world.trigger_ref(&mut Despawned(entity, enemy));
            world.despawn(entity);
        }
    });
}

pub fn on_register_player(
    on : On<RegisterPlayer>,
    mut player_register : Mut<'_, PlayerRegister>
) {
    let register = on.event();
    player_register.insert(register.0, register.1);
    player_register.update_future(register.0, register.2);
}

pub fn on_inputed(
    on : On<Inputed>,
    mut commands : Commands,
    player_register :  &PlayerRegister,
) {
    let inputed = on.event();
    let Inputed(entity, machine_id, input) = inputed;

    if let Some(item) = input {
        commands.trigger(RemoveFromInventory(*entity, *item));
    }
    
    match player_register.get_type(entity).unwrap() {
        PlayerType::Client(client) => {
            commands.trigger(events::Inputed(*entity, client, *machine_id, *input));
        },  
        PlayerType::Service(service) => {
            commands.trigger(events::Inputed(*entity, service, *machine_id, *input));
        },
        PlayerType::Enemy(enemy) => {
            commands.trigger(events::Inputed(*entity, enemy, *machine_id, *input));
        },  
    }
}

pub fn on_outputed(
    on : On<Outputed>,
    mut commands : Commands,
    player_register :  &PlayerRegister,
) {
    let outputed = on.event();
    let Outputed(entity, machine_id, input) = outputed;

    if let Some(item) = input {
        commands.trigger(AddToInventory(*entity, *item));
    }

    match player_register.get_type(entity).unwrap() {
        PlayerType::Client(client) => {
            commands.trigger(events::Outputed(*entity, client, *machine_id, *input));
        },  
        PlayerType::Service(service) => {
            commands.trigger(events::Outputed(*entity, service, *machine_id, *input));
        },
        PlayerType::Enemy(enemy) => {
            commands.trigger(events::Outputed(*entity, enemy, *machine_id, *input));
        },  
    }
}

pub fn on_execute_instruction(
    on : On<ExecuteInstruction>,
    mut commands : Commands,
    instruction_manager : &InstructionManager,
    mut player_register :  Mut<PlayerRegister>,
    machine_register : & MachineRegister,
    grid : &Grid,
) {
    let ExecuteInstruction(player, label) = on.event();
    let instruction = instruction_manager.get(label).unwrap();
    let mut iter = instruction.iter();

    let mut queue = Vec::new();

    let mut future_position = player_register.get_future(player).unwrap();
    let player_id = player_register.get(player).unwrap();

    while let Some(step) = iter.next() {
        match step {
            Step::MoveTo(point) => {
                add_path(grid, &mut queue, future_position, *point);
                future_position = *point;
            },
            Step::Interact(machine_type, item) => {
                let machine = machine_register.get(machine_type).unwrap();
                queue.push(Box::new(player_actions::Interact{ machine, item: *item }));
            },
            Step::WaitUntil(machine_type) => {
                let machine = machine_register.get(machine_type).unwrap();
                queue.push(Box::new(player_actions::WaitUntil{ machine }));
            },
        }
    }
    commands.trigger(QueueActions(player_id, queue));

    player_register.update_future(*player, future_position);
}


pub fn on_execute_step(
    on : On<ExecuteStep>,
    mut commands : Commands,
    mut player_register :  Mut<PlayerRegister>,
    machine_register : & MachineRegister,
    grid : &Grid,
) {
    let ExecuteStep(player, step) = on.event();

    let mut queue = Vec::new();

    let mut future_position = player_register.get_future(player).unwrap();
    let player_id = player_register.get(player).unwrap();

    match step {
        Step::MoveTo(point) => {
            add_path(grid, &mut queue, future_position, *point);
            future_position = *point;
        },
        Step::Interact(machine_type, item) => {
            let machine = machine_register.get(machine_type).unwrap();
            queue.push(Box::new(player_actions::Interact{ machine, item: *item }));
        },
        Step::WaitUntil(machine_type) => {
            let machine = machine_register.get(machine_type).unwrap();
            queue.push(Box::new(player_actions::WaitUntil{ machine }));
        },
    }

    commands.trigger(QueueActions(player_id, queue));

    player_register.update_future(*player, future_position);
}
