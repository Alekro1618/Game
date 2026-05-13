use engine::{ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::{resources::GameSpeed, timer::Timer}};
use engine_macros::{Component, Event};

use crate::{machines::machine::InteractWithMachine, players::{player::{PendingAction, Player, PlayerAction}, player_type::Client, spawner::{SpawnClient, SpawnEnemy, SpawnService, SpawnedClient, SpawnedEnemy, SpawnedService}}, utils::grid::{self, Point, SmoothToGrid}};



pub fn advance_smooth(
    query : Query<&mut SmoothToGrid>,
    speed : &GameSpeed
) {
    for mut smooth in query {
        smooth.update(4.0 * speed.relative());
    }
}


/// Handles players with `PendingAction` component.
/// If `PlayerAction` exists then it will remove `PendingAction` and proceed with either
/// - `PlayerAction::WalkTo` adds `SmoothToGrid` from current position to target position
/// - `PlayerAction::Interact` adds `Interacting` at curret position
pub fn handle_pending_players(
    on : On<PendingAction>,
    mut command : Commands,
    query : Query<&mut Player>
) {
    let entity = on.event().0;

    let Ok(mut player) = query.get_mut(entity) else {
        panic!("");
    };

    let current_pos = player.point;

    //Procceed with next action
    player.next_action();
    
    if let Some(action) = player.current_action {
        match action {
            PlayerAction::WalkTo(point) => {
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
            PlayerAction::Interact(with) => {
                command.trigger(InteractWithMachine(entity, with));
            },
            PlayerAction::Direction(_) => {

            },
        }
    }

    if player.is_next_action_interact() {
        command.trigger(PendingAction(entity));
    }
}


pub fn on_client_spawn(
    on : On<SpawnClient>,
    mut commands : Commands
) {
    let spawn= on.event();
    let client = spawn.0;
    commands.queue(move |world| {
        let entity_id = world.spawn((client,));
        world.trigger_ref(&mut SpawnedClient(entity_id, client));
    });
}

pub fn on_service_spawn(
    on : On<SpawnService>,
    mut commands : Commands
) {
    let spawn= on.event();
    let service = spawn.0;
    commands.queue(move |world| {
        let entity_id = world.spawn((service,));
        world.trigger_ref(&mut SpawnedService(entity_id, service));
    });
}

pub fn on_enemy_spawn(
    on : On<SpawnEnemy>,
    mut commands : Commands
) {
    let spawn= on.event();
    let enemy = spawn.0;
    commands.queue(move |world| {
        let entity_id = world.spawn((enemy,));
        world.trigger_ref(&mut SpawnedEnemy(entity_id, enemy));
    });
}