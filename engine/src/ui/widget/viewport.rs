use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::ecs::entities::id::EntityId;

#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ViewportNode {
    pub camera: Option<EntityId>
}

impl ViewportNode {
    #[inline]
    pub const fn new(camera: EntityId) -> Self {
        Self {
            camera: Some(camera),
        }
    }
}
