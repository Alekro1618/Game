use std::collections::HashMap;

use engine::ecs::entities::id::EntityId;
use engine_macros::{Event, Resource};

use crate::{players::player_type::PlayerType, utils::grid::Point};

#[derive(Resource, Default)]
pub struct PlayerRegister {
    map : HashMap<PlayerType, EntityId>,
    future: HashMap<PlayerType, Point>,
    id_to_type : HashMap<EntityId, PlayerType>
}

impl PlayerRegister {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            future: HashMap::new(),
            id_to_type : HashMap::new()
        }
    }

    pub fn get(&self, player_type : &PlayerType) -> Option<EntityId> {
        self.map.get(player_type).map(|e| *e)
    }

    pub fn get_type(&self, entity_id : &EntityId) -> Option<PlayerType> {
        self.id_to_type.get(entity_id).map(|t| *t)
    }

    pub fn get_future(&self,  player_type : &PlayerType) -> Option<Point> {
        self.future.get(player_type).map(|e| *e)
    }

    pub fn update_future(&mut self, player_type : PlayerType, future : Point) -> Option<Point> {
        self.future.insert(player_type, future)
    }


    pub fn take(&mut self, player_type : &PlayerType) -> Option<EntityId> {
        let entity_id = self.map.remove(player_type);
        if let Some(entity_id) = entity_id {
             self.id_to_type.remove(&entity_id);
        }
        entity_id
    }

    pub fn insert(&mut self, player_type : PlayerType, entity: EntityId) {
        self.map.insert(player_type, entity);
        self.id_to_type.insert(entity, player_type);
    }

}

#[derive(Event)]
pub struct RegisterPlayer(pub PlayerType, pub EntityId, pub Point);