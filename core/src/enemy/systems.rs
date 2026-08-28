use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::Commands, query::Query}}, math::{transform::Transform, vec::Vec2}, render::{color::Color, sprite::Sprite}};

use crate::{actions::components::{ActionQueue, CurrentAction}, algos::path_finding::find_path, bundles::PlayerBundle, client::client::ClientList, enemy::enemy::{EnemyList, SpawnRandomEnemy}, items::Inventory, machines::machine::WaitingFor, players::{ events::{Spawn, Spawned}, player::{ Player, PlayerSpeed}, player_actions::SetPath, player_state::PlayerState, player_type::{Enemy, PlayerType, Service}}, queue, utils::grid::{Grid, GridName, Point, SmoothToGrid, SpecialKeys}};



pub fn on_enemy_spawned(
    on : On<Spawned<Enemy>>,
    mut commands: Commands,
    grid : &Grid
) {
    let spawned_enemy: &Spawned<Enemy> = on.event();
    let entity = spawned_enemy.0;
    let enemy = spawned_enemy.1;

    let position = grid.get_special(SpecialKeys::Entrence);

    let (current_action, action_queue) = queue![
        SetPath {
            from: (0, 0).into(),
            to: (0, 1).into()
        },
        SetPath {
            from : (0, 1).into(),
            to: (0, 2).into()
        }
    ];

    commands.add(entity, PlayerBundle {
        player: Player,
        smooth_to_grid: SmoothToGrid::new(GridName::Players, position, position),
        player_state: PlayerState::Idle,
        player_speed: PlayerSpeed(4.0),
        transform: Transform::IDENTITY,
        sprite: Sprite::from_color(Color::BLACK, Vec2 { x: 100.0, y: 100.0 }),
        inventory: Inventory::new(),
        current_action,
        action_queue,
    });
}



pub fn on_spawn_random(
    _on: On<SpawnRandomEnemy>,
    mut commands : Commands,
    mut enemy_list : Mut<EnemyList>
) {
    if let Some(next_enemy) = enemy_list.enemies.pop() {
        commands.trigger(Spawn::<Enemy>(next_enemy, (0, 0).into()));
    }
}