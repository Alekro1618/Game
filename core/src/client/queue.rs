use std::collections::VecDeque;

use engine::ecs::entities::id::EntityId;
use engine_macros::{Component, Event, Resource};

use crate::utils::grid::Point;


#[derive(Event)]
pub struct MoveQueue;