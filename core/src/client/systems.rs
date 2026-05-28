use core::num;

use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, query::Query}}, math::{transform::Transform, vec::Vec2}, render::{color::Color, sprite::Sprite}};
use rand::random_bool;

use crate::{algos::path_finding::find_path, client::{client::{ClientList, client_color}, queue::{MoveQueue, SpawnRandomClient}}, items::Inventory, machines::{machine::WaitingFor, machine_type::MachineType}, players::{player::{Interact, PendingAction, Player, PlayerAction, SetPath}, player_type::{Client, PlayerType}, spawner::{SpawnClient, SpawnedClient}}, utils::grid::{Grid, GridName, Point, SmoothToGrid, SpecialKeys}};

pub fn on_client_spawned(
    on : On<SpawnedClient>,
    mut commands: Commands,
    grid : &Grid
) {
    let spawned_client = on.event();
    let entity = spawned_client.0;
    let client = spawned_client.1;

    let player = Player::new(grid.get_special(SpecialKeys::Entrence));
    commands.add(entity, (
        player, 
        Transform::IDENTITY, 
        Sprite::from_color(client_color(&client), Vec2 { x: 100.0, y: 100.0 }),
        Inventory::new()
    ));


    commands.trigger(
        SetPath {
            player: PlayerType::Client(client), 
            from: grid.get_special(SpecialKeys::Entrence), 
            to: grid.get_special(SpecialKeys::CashRegister) 
        }
    );
    commands.trigger(
        Interact {
            player: PlayerType::Client(client), 
            machine: MachineType::CashRegister, 
            item : None 
        }
    );
    commands.trigger(
        SetPath {
            player: PlayerType::Client(client), 
            from: grid.get_special(SpecialKeys::CashRegister), 
            to: grid.get_special(SpecialKeys::Exit) 
        }
);
    commands.trigger(PendingAction(entity));
}

///System that gives [`PendingAction`] whenever it is needed. Specifically it will let client move if there are no clients
pub fn move_client_until_queue(
    mut commands : Commands,
    query_clients : Query<(EntityId, &Client, &SmoothToGrid,  &Player), Without<WaitingFor>>,
    grid : &Grid
) {
    for (entity, _, smooth_to_grid, player) in query_clients {
        //We will wait until it reaches the end of [`SmoothToGrid`] 
        if !smooth_to_grid.is_end() {
            continue;
        }

        //Look at next action
        let Some(next_action) = player.view_next_action() else {
            continue;
        };

        if let PlayerAction::WalkTo(point) = &next_action {
            //If there is another player - we don't move
            if grid.get_point(GridName::Players, *point).value() {
                continue;
            }
            
            commands.trigger(PendingAction(entity));
        } else {
            commands.trigger(PendingAction(entity));
        };
    }
}

/// When triggers [`MoveQueue`] it will move each [`Client`] by one in the queue
pub fn on_move_queue(
    _on: On<MoveQueue>,
    mut commands : Commands,
    query_clients : Query<(EntityId, &Client, &Player)>,
) {
    let mut number_of_clients = 0;
    for (entity, client, _) in query_clients {
        //Send for every entity [`PendingAction`]
        number_of_clients += 1;
        commands.trigger(PendingAction(entity));
    }

    if random_bool(0.5) {
        commands.trigger(SpawnRandomClient);
        number_of_clients += 1;
    }

    if number_of_clients <= 1 {
        commands.trigger(SpawnRandomClient);
    }
}

pub fn on_spawn_random(
    _on: On<SpawnRandomClient>,
    mut commands : Commands,
    mut client_list : Mut<ClientList>,
) {
    if let Some(next_client) = client_list.clients.pop() {
        commands.trigger(SpawnClient(next_client));
    }
}