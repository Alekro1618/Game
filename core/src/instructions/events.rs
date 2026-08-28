use engine::ecs::entities::id::EntityId;
use engine_macros::Event;

use crate::{instructions::components::{InstructionLabel, Step}, players::player_type::PlayerType};


#[derive(Event)]
pub struct ExecuteInstruction(pub PlayerType, pub InstructionLabel);

#[derive(Event)]
pub struct ExecuteStep(pub PlayerType, pub Step);