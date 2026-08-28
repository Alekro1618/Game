use std::collections::HashMap;

use engine::ecs::entities::id::EntityId;
use engine_macros::Resource;

use crate::machines::machine_type::MachineType;

#[derive(Resource, Default)]
pub struct MachineRegister {
    map : HashMap<MachineType, EntityId>,
    position : HashMap<MachineType, EntityId>
}

impl MachineRegister {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            position: HashMap::new()
        }
    }

    pub fn get(&self, machine_type : &MachineType) -> Option<EntityId> {
        self.map.get(machine_type).map(|e| *e)
    }

    pub fn insert(&mut self, machine_type : MachineType, entity: EntityId) {
        self.map.insert(machine_type, entity);
    }
}