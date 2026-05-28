use std::collections::HashMap;

use engine::ecs::entities::id::EntityId;
use engine_macros::{Event, Resource};

use crate::players::player_type::PlayerType;

#[derive(Resource, Default)]
pub struct PlayerRegister {
    map : HashMap<PlayerType, EntityId>
}

impl PlayerRegister {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn get(&self, player_type : &PlayerType) -> Option<EntityId> {
        self.map.get(player_type).map(|e| *e)
    }

    pub fn insert(&mut self, player_type : PlayerType, entity: EntityId) {
        self.map.insert(player_type, entity);
    }
}

#[derive(Event)]
pub struct RegisterPlayer(pub PlayerType, pub EntityId);