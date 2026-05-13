use engine::{ecs::{entities::{id::EntityId}, observer::system_param::On, systems::{commands::Commands, query::Query}}, math::transform::Transform};

use crate::{algos::path_finding::find_path, client::queue::MoveQueue, machines::{machine::WaitingFor, machine_register::CashRegister}, players::{player::{PendingAction, Player, PlayerAction}, player_type::Client, spawner::SpawnedClient}, utils::grid::{Grid, GridName, Point, SmoothToGrid}};

const ENTRENCE : Point = Point::new(0, 4);
const CASH_REGISTER : Point = Point::new(4, 4);

pub fn on_client_spawned(
    on : On<SpawnedClient>,
    mut commands: Commands,
    grid : &Grid,
    cash_register : &CashRegister
) {
    let spawned_client = on.event();
    let entity = spawned_client.0;
    let _client = spawned_client.1;

    let mut player = Player::new(ENTRENCE);
    player.add_path(find_path(grid, ENTRENCE, CASH_REGISTER));
    player.add_action(PlayerAction::Interact(cash_register.0));
    player.add_path(find_path(grid, CASH_REGISTER, ENTRENCE));

    commands.add(entity, (player, Transform::IDENTITY));
    commands.trigger(PendingAction(entity));
}

///System that gives [`PendingAction`] whenever it is needed. Specifically it will let client move if there are no clients
pub fn move_client_until_queue(
    mut commands : Commands,
    query_clients : Query<(EntityId, &Client, &SmoothToGrid,  &Player)>,//TODO: Add Without<WaitingFor>
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

        let PlayerAction::WalkTo(point) = &next_action else {
            continue;
        };

        //If there is another player - we don't move
        if grid.get_point(GridName::Players, *point).value() {
            continue;
        }

        commands.trigger(PendingAction(entity));
    }
}

/// When triggers [`MoveQueue`] it will move each [`Client`] by one in the queue
pub fn on_move_queue(
    _on: On<MoveQueue>,
    mut commands : Commands,
    query_clients : Query<(EntityId, &Client, &Player)>,
) {
    for (entity, _, _) in query_clients {
        //Send for every entity [`PendingAction`]
        commands.trigger(PendingAction(entity));
    }
}