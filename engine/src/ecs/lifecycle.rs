use std::{any::Any, collections::HashMap};

use crate::ecs::{components::{Component, id::ComponentId}, entities::{id::EntityId}, event::EventKey, worlds::World};



#[derive(Clone, Copy, Debug)]
pub struct HookContext {
    pub entity: EntityId,
    pub component_id: ComponentId
}

pub struct HookWorld<'w>(&'w mut World);

impl<'w> HookWorld<'w> {
    pub fn get_component<T: Component>(&self, entity: EntityId) -> Option<&T> {
        self.0.get_component::<T>(entity)
    }

    // pub fn queue(&mut self, cmd: impl FnOnce(&mut World) + Send + 'static) {
    //     self.0.command_queue.push(cmd);
    // } TODO
}

pub type ComponentHook = for<'w> fn(HookWorld<'w>, HookContext);

#[derive(Debug, Clone, Default)]
pub struct ComponentHooks {
    pub(crate) on_add: Option<ComponentHook>,
    pub(crate) on_insert: Option<ComponentHook>,
    pub(crate) on_discard: Option<ComponentHook>,
    pub(crate) on_remove: Option<ComponentHook>,
    pub(crate) on_despawn: Option<ComponentHook>,
}

impl ComponentHooks {
    pub(crate) fn update_from_component<C: Component + ?Sized>(&mut self) -> &mut Self {
        if let Some(hook) = C::on_add() {
            self.on_add(hook);
        }
        if let Some(hook) = C::on_insert() {
            self.on_insert(hook);
        }
        if let Some(hook) = C::on_discard() {
            self.on_discard(hook);
        }
        if let Some(hook) = C::on_remove() {
            self.on_remove(hook);
        }
        if let Some(hook) = C::on_despawn() {
            self.on_despawn(hook);
        }

        self
    }
    pub fn on_add(&mut self, hook: ComponentHook) -> &mut Self {
        self.try_on_add(hook)
            .expect("Component already has an on_add hook")
    }

    pub fn on_insert(&mut self, hook: ComponentHook) -> &mut Self {
        self.try_on_insert(hook)
            .expect("Component already has an on_insert hook")
    }


    pub fn on_discard(&mut self, hook: ComponentHook) -> &mut Self {
        self.try_on_discard(hook)
            .expect("Component already has an on_discard hook")
    }

    pub fn on_remove(&mut self, hook: ComponentHook) -> &mut Self {
        self.try_on_remove(hook)
            .expect("Component already has an on_remove hook")
    }

    pub fn on_despawn(&mut self, hook: ComponentHook) -> &mut Self {
        self.try_on_despawn(hook)
            .expect("Component already has an on_despawn hook")
    }

    pub fn try_on_add(&mut self, hook: ComponentHook) -> Option<&mut Self> {
        if self.on_add.is_some() {
            return None;
        }
        self.on_add = Some(hook);
        Some(self)
    }

    pub fn try_on_insert(&mut self, hook: ComponentHook) -> Option<&mut Self> {
        if self.on_insert.is_some() {
            return None;
        }
        self.on_insert = Some(hook);
        Some(self)
    }

    pub fn try_on_discard(&mut self, hook: ComponentHook) -> Option<&mut Self> {
        if self.on_discard.is_some() {
            return None;
        }
        self.on_discard = Some(hook);
        Some(self)
    }

    pub fn try_on_remove(&mut self, hook: ComponentHook) -> Option<&mut Self> {
        if self.on_remove.is_some() {
            return None;
        }
        self.on_remove = Some(hook);
        Some(self)
    }

    pub fn try_on_despawn(&mut self, hook: ComponentHook) -> Option<&mut Self> {
        if self.on_despawn.is_some() {
            return None;
        }
        self.on_despawn = Some(hook);
        Some(self)
    }
}

pub const ADD: EventKey = EventKey(ComponentId::new(0));
pub const INSERT: EventKey = EventKey(ComponentId::new(1));
pub const REPLACE: EventKey = EventKey(ComponentId::new(2));
pub const REMOVE: EventKey = EventKey(ComponentId::new(3));
pub const DESPAWN: EventKey = EventKey(ComponentId::new(4));

#[derive(Debug, Clone)]
pub struct Add {
    pub entity: EntityId,
}

#[derive(Debug, Clone)]
pub struct Insert {
    pub entity: EntityId,
}

#[derive(Debug, Clone)]
pub struct Replace {
    pub entity: EntityId,
}

#[derive(Debug, Clone)]
pub struct Remove {
    pub entity: EntityId,
}

#[derive(Debug, Clone)]
pub struct Despawn {
    pub entity: EntityId,
}

#[derive(Debug, Clone)]
pub struct RemovedComponentEntity(EntityId);


pub struct RemovedComponentMessages {
    event_sets: HashMap<ComponentId, Vec<RemovedComponentEntity>>,
}

impl Default for RemovedComponentMessages {
    fn default() -> Self {
        Self { event_sets: HashMap::new() }
    }
}

impl RemovedComponentMessages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        for (_component_id, messages) in self.event_sets.iter_mut() {
            todo!("update each message")
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&ComponentId, &Vec<RemovedComponentEntity>)> {
        self.event_sets.iter()
    }

    pub fn get(
        &self,
        component_id: impl Into<ComponentId>,
    ) -> Option<&Vec<RemovedComponentEntity>> {
        self.event_sets.get(&component_id.into())
    }

    pub fn write(&mut self, component_id: impl Into<ComponentId>, entity: EntityId) {
        todo!("write")
    }
}