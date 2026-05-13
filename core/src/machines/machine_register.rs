use std::collections::HashMap;

use engine::ecs::entities::id::EntityId;
use engine_macros::Resource;

use crate::machines::machine_type::MachineType;

#[derive(Resource)]
pub struct CashRegister(pub EntityId);