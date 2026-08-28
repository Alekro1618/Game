use std::any::Any;

use crate::ecs::{
    components::{Component, Mutable, StorageType},
    event::Event,
    systems::observer_system::IntoObserverSystem,
};

pub struct Observer {
    pub(crate) system: Box<dyn Any + Send + Sync + 'static>,
    pub(crate) last_trigger_id: u32,
    pub(crate) despawned_watched_entities: u32,
}

impl Observer {
    pub fn new<E: Event, M, I: IntoObserverSystem<E, M>>(system: I) -> Self {
        let system = Box::new(IntoObserverSystem::into_system(system));
        Self {
            system,
            despawned_watched_entities: 0,
            last_trigger_id: 0,
        }
    }
}

impl Component for Observer {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    type Mutability = Mutable;
}

/// Trait for types that can be converted into an [`Observer`].
pub trait IntoObserver<E: Event, Marker>: Send + 'static {
    /// Converts this type into an [`Observer`].
    fn into_observer(self) -> Observer;
}

impl<E: Event, M, T: IntoObserverSystem<E, M>> IntoObserver<E, M> for T {
    fn into_observer(self) -> Observer {
        Observer::new(self)
    }
}
