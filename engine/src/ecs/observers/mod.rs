use std::{ any::Any, collections::HashMap};

use crate::ecs::{components::id::ComponentId, entities::id::EntityId, event::EventKey, lifecycle::ComponentHook};

pub type ObserverRunner = fn(observer: EntityId);

pub trait AnyNamedSystem: Any + Send + Sync + 'static {
    fn system_name(&self) -> String;
}

impl<T: Any + Send + Sync> AnyNamedSystem for T {
    fn system_name(&self) -> String {
        self.system_name()
    }
}


pub struct Observer {
    hook_on_add: ComponentHook,
    pub(crate) system: Box<dyn AnyNamedSystem>,
    pub(crate) descriptor: ObserverDescriptor,
    pub(crate) last_trigger_id: u32,
    pub(crate) despawned_watched_entities: u32,
    pub(crate) runner: ObserverRunner,
}



#[derive(Default, Clone)]
pub struct ObserverDescriptor {
    pub event_keys: Vec<EventKey>,
    pub components: Vec<ComponentId>,
    pub(super) entities: Vec<EntityId>,
}

impl ObserverDescriptor {
    pub unsafe fn with_event_keys(mut self, event_keys: Vec<EventKey>) -> Self {
        self.event_keys = event_keys;
        self
    }

    pub fn with_components(mut self, components: Vec<ComponentId>) -> Self {
        self.components = components;
        self
    }

    pub fn with_entities(mut self, entities: Vec<EntityId>) -> Self {
        self.entities = entities;
        self
    }

    pub fn event_keys(&self) -> &[EventKey] {
        &self.event_keys
    }

    pub fn components(&self) -> &[ComponentId] {
        &self.components
    }

    /// Returns the `entities` that the observer is watching.
    pub fn entities(&self) -> &[EntityId] {
        &self.entities
    }
}

#[derive(Default, Debug)]
pub struct Observers {
    add: Vec<ObserverRunner>,
    insert: Vec<ObserverRunner>,
    replace: Vec<ObserverRunner>,
    remove: Vec<ObserverRunner>,
    despawn: Vec<ObserverRunner>,
    cache: HashMap<EventKey, Vec<ObserverRunner>>,
}

impl Observers {
    pub(crate) fn get_observers_mut(&mut self, event_key: EventKey) -> &mut Vec<ObserverRunner> {
        use crate::ecs::lifecycle::*;

        match event_key {
            ADD => &mut self.add,
            INSERT => &mut self.insert,
            REPLACE => &mut self.replace,
            REMOVE => &mut self.remove,
            DESPAWN => &mut self.despawn,
            _ => self.cache.entry(event_key).or_default(),
        }
    }

    pub fn try_get_observers(&self, event_key: EventKey) -> Option<&Vec<ObserverRunner>> {
        use crate::ecs::lifecycle::*;

        match event_key {
            ADD => Some(&self.add),
            INSERT => Some(&self.insert),
            REPLACE => Some(&self.replace),
            REMOVE => Some(&self.remove),
            DESPAWN => Some(&self.despawn),
            _ => self.cache.get(&event_key),
        }
    }

    pub fn register(&mut self, event_key: EventKey, f: ObserverRunner) {
        use crate::ecs::lifecycle::*;

        match event_key {
            ADD => self.add.push(f),
            INSERT => self.add.push(f),
            REPLACE => self.add.push(f),
            REMOVE => self.add.push(f),
            DESPAWN => self.add.push(f),
            _ => {
                if let Some(list) = self.cache.get_mut(&event_key) {
                    list.push(f);
                } else {
                    self.cache.insert(event_key, vec![f]);
                }
            },
        }
    }

    pub fn notify(&self, entity: EntityId) {
        for add in &self.add {
            add(entity);
        }

        for insert in &self.insert {
            insert(entity);
        }

        for replace in &self.replace {
            replace(entity);
        }

        for remove in &self.remove {
            remove(entity);
        }

        for despawn in &self.despawn {
            despawn(entity);
        }

        for (_, list) in &self.cache {
            for runner in list {
                runner(entity);
            }
        }
    }
}
