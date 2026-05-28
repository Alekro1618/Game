use std::{any::TypeId, cell::UnsafeCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::ecs::{entities::id::EntityId, event::{Event}, observer::{system_param::{On}}, systems::observer_system::ObserverSystem, worlds::World};

#[derive(Default)]
pub struct Observers {
    cache: HashMap<TypeId, CachedObserver>,
}

impl Observers {

    pub fn register<E: Event>(
        &mut self,
        system: impl ObserverSystem<E>,
    ) {
        self.cache.insert(TypeId::of::<E>(), CachedObserver::new(system, None));
    }

    pub fn register_with<E: Event>(
        &mut self,
        entity_id: EntityId,
        system: impl ObserverSystem<E>,
    ) {
        self.cache.insert(TypeId::of::<E>(), CachedObserver::new(system, Some(entity_id)));
    }

    pub fn get<E: Event>(
        &self
    ) -> Option<&CachedObserver> {
        self.cache.get(&TypeId::of::<E>())
    }

    pub fn get_mut<E: Event>(
        &mut self
    ) -> Option<&mut CachedObserver> {
        self.cache.get_mut(&TypeId::of::<E>())
    }

    pub fn remove<E: Event>(
        &mut self
    ) -> Option<CachedObserver> {
        self.cache.remove(&TypeId::of::<E>())
    }

    pub fn insert<E : Event>(
        &mut self,
        cached_observer : CachedObserver
    ) {
        self.cache.insert(TypeId::of::<E>(), cached_observer);
    }
}

pub struct CachedObserver {
    event_type: TypeId,
    observer_function: ObserverFunction,
    observer: Option<EntityId>,
}

impl CachedObserver {
    pub fn new<E : Event>(system: impl ObserverSystem<E>, observer: Option<EntityId>) -> Self {
        let  system = UnsafeCell::new(system);
        Self {
            event_type: TypeId::of::<E>(),
            observer_function: Box::new(move |world,   event| {
                unsafe  {
                    let system = &mut *system.get();
                    system.init(world);
                    system.set_on_state(event);
                    system.run(world);
                    system.apply(world);
                }
            }),
            observer: observer
        }
    }
    
    pub fn observer_function(&self) -> &ObserverFunction {
        &self.observer_function
    }
}

pub type ObserverFunction = Box<dyn Fn(&mut World, *mut u8)>;

pub type ObserverMap 
    = HashMap<EntityId, ObserverFunction>;
