use core::num;

use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, math::{transform::Transform, vec::Vec2}, render::sprite::Sprite};
use rand::seq::IteratorRandom;

use crate::{bundles::PlayerBundle, client::{client::{ClientList, client_color}, queue::{InQueue, MoveQueue, Queue, SpawnRandomClient}}, instructions::{components::{InstructionLabel, SpecialLabel, Step}, events::{ExecuteInstruction, ExecuteStep}}, items::Inventory, machines::{machine::{Machine, WaitingFor}, machine_type::MachineType}, order::order::{CompletedOrder, NewRandomOrder}, players::{ events::{Inputed, Spawn, Spawned}, player::{ Player, PlayerSpeed}, player_actions::{SetPath, add_path}, player_state::PlayerState, player_type::{Client, PlayerType}}, queue, utils::grid::{Grid, GridName, Point, SmoothToGrid, SpecialKeys}};

pub fn on_client_spawned(
    on : On<Spawned<Client>>,
    mut commands: Commands,
    grid : &Grid,
    query : Query<(EntityId, &mut Queue)>,
) {
    let spawned_client = on.event();
    let entity = spawned_client.0;
    let client = spawned_client.1;

    let position = grid.get_special(SpecialKeys::Entrence);

    let mut rng = rand::rng();
    let (queue_id, mut queue) = query.iter().choose(&mut rng).unwrap();
    let target = queue.get_free().unwrap();
    queue.add(client);
    
    commands.add(entity, 
    PlayerBundle { 
            smooth_to_grid: SmoothToGrid::new(GridName::Players, position, position),
            player_speed: PlayerSpeed(2.0), 
            transform: Transform::IDENTITY, 
            sprite: Sprite::from_color(client_color(&client), Vec2 { x: 100.0, y: 100.0 }),
            ..Default::default()
        }
    );
    commands.add(entity,
        InQueue(queue_id)
    );

    commands.trigger(ExecuteStep(PlayerType::Client(client), Step::MoveTo(target)));

    if queue.len() == 1 {
        commands.trigger(ExecuteStep(PlayerType::Client(client), Step::Interact(MachineType::CashRegister, None)));
    }
}

pub fn client_to_queue(
    entity : EntityId,
    client : Client,

) {
    
}


/// When triggers [`MoveQueue`] it will move each [`Client`] by one in the queue
pub fn on_move_queue(
    _on: On<MoveQueue>,
    mut commands : Commands,
    query_queues : Query<&mut Queue>,
) {
    for mut queue in query_queues {
        for client in queue.clients() {
            let (next, is_first) = queue.move_entity(&client);
            
            if let Some(next) = next {
                commands.trigger(ExecuteStep(PlayerType::Client(client), Step::MoveTo(next)));
                if is_first {
                    commands.trigger(ExecuteStep(PlayerType::Client(client), Step::Interact(MachineType::CashRegister, None)));
                }
            } else {
                commands.trigger(ExecuteInstruction(PlayerType::Client(client), InstructionLabel::Special(SpecialLabel::Exit)));
            }
        }
        
        if queue.len() <= 1 {
            commands.trigger(SpawnRandomClient);
        }
    }    
}

pub fn on_spawn_random(
    _on: On<SpawnRandomClient>,
    mut commands : Commands,
    mut client_list : Mut<ClientList>,
) {
    if let Some(next_client) = client_list.clients.pop() {
        commands.trigger(Spawn::<Client>(next_client, (0, 0).into()));
    }
}

pub fn on_cash_register_input(
    on : On<Inputed<Client>>,
    mut commands: Commands,
    query : Query<&MachineType>
) {
    let input = on.event();
    let Inputed::<Client>(entity, _,  machine_id,  _) = input;

    let Ok(machine_type) = query.get_mut(*machine_id) else {
        panic!("");
    };
    
    if matches!(machine_type, MachineType::CashRegister) {
        commands.trigger(NewRandomOrder(*entity));
    }  
}

pub fn on_completed_order(
    on : On<CompletedOrder>,
    query : Query<(&Client, &InQueue)>,
    mut commands: Commands
) {
    let CompletedOrder(client_id, order) = on.event();
    let (client, in_queue) = query.get(*client_id).unwrap();

    commands.trigger(MoveQueue(in_queue.0));
}