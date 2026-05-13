use crate::ecs::components::{Component, Mutable, StorageType};

/// Marker struct for labels
#[derive(Debug, Default, Clone, Copy)]
pub struct Label;

impl Component for Label {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}
