use std::{any::{Any, TypeId}, ops::Deref, sync::atomic::AtomicU32};

use crate::ecs::components::{Component, StorageType, id::ComponentId, info::{ComponentDescriptor, Components}};



#[derive(Debug, Default)]
pub struct ComponentIds {
    pub next : AtomicU32
}

impl ComponentIds {
    pub fn peek(&self) -> ComponentId {
        ComponentId {
            id : self.next
                .load(std::sync::atomic::Ordering::Relaxed),
        }
    }

    pub fn next(&self) -> ComponentId {
        ComponentId {
            id : self.next
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }

    pub fn peek_mut(&mut self) -> ComponentId {
        ComponentId {
            id : *self.next.get_mut()
        }
    }

    pub fn next_mut(&mut self) -> ComponentId {
        let id = self.next.get_mut();
        let result = ComponentId {
            id : *id
        };
        *id += 1;
        result
    }

    pub fn len(&self) -> usize {
        self.peek().id as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }


}

pub struct ComponentsRegistrator<'w> {
    pub components: &'w mut Components,
    pub ids: &'w mut ComponentIds
}

impl Deref for ComponentsRegistrator<'_> {
    type Target = Components;

    fn deref(&self) -> &Self::Target {
        self.components
    }
}

impl<'w> ComponentsRegistrator<'w> {
    pub unsafe fn new(components: &'w mut Components, ids: &'w mut ComponentIds) -> Self {
        Self {
            components,
            ids
        }
    }

    #[inline]
    pub fn register_component<T: Component>(&mut self) -> ComponentId {
         self.register_component_checked::<T>()
    }

    #[inline]
    pub fn register_component_checked<T: Component>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        if let Some(&id) = self.indices.get(&type_id) {
            return id;
        }

        let id = ComponentId { id: self.components.components.len() as u32 };

        unsafe {
            self
            .components
            .register_non_send_unchecked(TypeId::of::<T>(), id, ComponentDescriptor::new::<T>());
        }

        id
    }

    #[inline]
    pub fn register_component_with_descriptor(
        &mut self,
        descriptor: ComponentDescriptor,
    ) -> ComponentId {
        let id = self.ids.next_mut();

        unsafe {
            self.components.register_component_inner(id, descriptor);
        }
        id
    }

    #[inline]
    pub fn register_non_send<T: Any>(&mut self) -> ComponentId {
        unsafe {
            self.register_non_send_with(TypeId::of::<T>(), || {
                ComponentDescriptor::new_non_send::<T>(StorageType::default())
            })
        }
    }

    #[inline]
    unsafe fn register_non_send_with(
        &mut self,
        type_id: TypeId,
        descriptor: fn() -> ComponentDescriptor,
    ) -> ComponentId {
        if let Some(id) = self.indices.get(&type_id) {
            return *id;
        }

        let id = self.ids.next_mut();
        unsafe {
            self.components
                .register_non_send_unchecked(type_id, id, descriptor());
        }
        id
    }

}