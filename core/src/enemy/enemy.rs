use engine::{ecs::entities::id::EntityId, render::color::Color};
use engine_macros::{Event, Resource};

use crate::players::player_type::Enemy;


#[derive(Resource)]
pub struct EnemyList {
    pub enemies : Vec<Enemy>
}

impl Default for EnemyList {
    fn default() -> Self {
        Self { enemies: vec![
            Enemy::First,
            Enemy::Second,
            Enemy::Third,
            Enemy::Fourth
        ]}
    }
}

pub fn client_color(client : &Enemy) -> Color {
    match client {
        Enemy::First => Color::BLACK,
        Enemy::Second => Color::BLUE,
        Enemy::Third => Color::GREEN,
        Enemy::Fourth => Color::YELLOW,
    }
}

#[derive(Event)]
pub struct SpawnRandomEnemy;