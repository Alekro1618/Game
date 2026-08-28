use std::marker::PhantomData;

use crate::ecs::components::{Component, Mutable, StorageType};

pub struct EventWrapperComponent<E: Event>(PhantomData<E>);

impl<E: Event> Component for EventWrapperComponent<E> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

pub trait Event: Send + Sync + Sized + 'static {}
