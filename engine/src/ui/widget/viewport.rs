use crate::ecs::{components::{Component, Mutable, StorageType}, entities::id::EntityId};



#[derive( Debug, Clone, Copy)]
pub struct ViewportNode {
    pub camera: Option<EntityId>,
}

impl Component for ViewportNode {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl ViewportNode {
    #[inline]
    pub const fn new(camera: EntityId) -> Self {
        Self {
            camera: Some(camera),
        }
    }
}
