use std::marker::PhantomData;

use crate::ecs::{components::{Component, Mutable, StorageType, id::ComponentId}, entities::id::EntityId, observer::{self, centralized_storage::CachedObserver}, worlds::World};

pub struct EventWrapperComponent<E: Event>(PhantomData<E>);

impl<E: Event> Component for EventWrapperComponent<E> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

pub trait Event: Send + Sync + Sized + 'static {
}
