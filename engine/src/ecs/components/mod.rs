use core::fmt::Debug;

pub mod id;
pub mod info;
pub mod register;

use crate::ecs::lifecycle::ComponentHook;

pub trait Component: Send + Sync + 'static {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability: ComponentMutability;

    fn on_add() -> Option<ComponentHook> {
        None
    }

    fn on_insert() -> Option<ComponentHook> {
        None
    }

    fn on_discard() -> Option<ComponentHook> {
        None
    }

    fn on_remove() -> Option<ComponentHook> {
        None
    }

    fn on_despawn() -> Option<ComponentHook> {
        None
    }

    // #[inline]
    // fn map_entities<E: EntityMapper>(_this: &mut Self, _mapper: &mut E) {}
}

mod private {
    pub trait Seal {}
}

pub trait ComponentMutability: private::Seal + 'static {
    const MUTABLE: bool;
}

pub struct Immutable;

impl private::Seal for Immutable {}

impl ComponentMutability for Immutable {
    const MUTABLE: bool = false;
}

pub struct Mutable;

impl private::Seal for Mutable {}

impl ComponentMutability for Mutable {
    const MUTABLE: bool = true;
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum StorageType {
    #[default]
    Table,
    SparseSet,
}
