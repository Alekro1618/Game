use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, query::Query}}, math::{transform::Transform, vec::Vec2}, render::{color::Color, sprite::Sprite}};

use crate::{client::client::ClientList, enemy::enemy::{EnemyList, SpawnRandomEnemy}, items::Inventory, machines::machine::WaitingFor, players::{player::{PendingAction, Player, PlayerAction, SetPath}, player_type::{Enemy, PlayerType, Service}, spawner::{SpawnEnemy, SpawnService, SpawnedEnemy, SpawnedService}}, utils::grid::{Grid, GridName, Point, SmoothToGrid, SpecialKeys}};



pub fn move_enemy(
    mut commands : Commands,
    query_enemies : Query<(EntityId, &Enemy, &SmoothToGrid,  &Player),  Without<WaitingFor>>,
    grid : &Grid
) {
    for (entity, _, smooth_to_grid, player) in query_enemies {

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
        } else if let PlayerAction::Interact(_, _) = &next_action {
            commands.trigger(PendingAction(entity));
        }
    }
}


pub fn on_enemy_spawned(
    on : On<SpawnedEnemy>,
    mut commands: Commands,
    grid : &Grid
) {
    let spawned_enemy: &SpawnedEnemy = on.event();
    let entity = spawned_enemy.0;
    let enemy = spawned_enemy.1;

    let player = Player::new(grid.get_special(SpecialKeys::Entrence), );
    commands.add(entity, (
        player,
        Transform::IDENTITY, 
        Sprite::from_color(Color::PURPLE, Vec2 { x: 100.0, y: 100.0 }),
        Inventory::new()
    ));

    commands.trigger(SetPath { player: PlayerType::Enemy(enemy), from: grid.get_special(SpecialKeys::Entrence), to: grid.get_special(SpecialKeys::ServicePlace) });
    commands.trigger(PendingAction(entity));
}



pub fn on_spawn_random(
    _on: On<SpawnRandomEnemy>,
    mut commands : Commands,
    mut enemy_list : Mut<EnemyList>
) {
    if let Some(next_enemy) = enemy_list.enemies.pop() {
        commands.trigger(SpawnEnemy(next_enemy));
    }
}