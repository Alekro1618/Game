use crate::ecs::components::{Component, Mutable, StorageType};

/// Marker struct for buttons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Button;

impl Component for Button {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}
