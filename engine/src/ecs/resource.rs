use std::{alloc::{Layout, alloc}, any::TypeId, collections::HashMap, ops::{Deref, DerefMut}};

use crate::ecs::{components::{Component, Mutable, StorageType, id::ComponentId}, entities::{Entity, id::EntityId}};


pub trait Resource: Component<Mutability = Mutable> {}

struct ResourceData {
    data: *mut u8,
    drop_fn: unsafe fn(*mut u8),
    layout: std::alloc::Layout,
}

unsafe impl Send for ResourceData {}
unsafe impl Sync for ResourceData {}

impl Drop for ResourceData {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.data);
            std::alloc::dealloc(self.data, self.layout);
        }
    }
}

#[derive(Default)]
pub struct ResourceEntities(HashMap<TypeId, ResourceData>);

impl ResourceEntities {
    pub fn get<R: Resource>(&self) -> Option<&R> {
        self.0
            .get(&TypeId::of::<R>())
            .map(|e| unsafe  { &*e.data.cast::<R>() })
    }

    pub fn insert<R: Resource>(&mut self, value : R) {
        unsafe fn drop_resource<R>(ptr: *mut u8) {
            ptr.cast::<R>().drop_in_place();
        }

        let layout = Layout::new::<R>();
        let ptr = unsafe { alloc(layout) };

        unsafe { ptr.cast::<R>().write(value); }

        let type_id = TypeId::of::<R>();

        self.0.remove(&type_id);

        self.0.insert(type_id, ResourceData {
            data: ptr,
            drop_fn: drop_resource::<R>,
            layout,
        });
    }

    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.0
            .get(&TypeId::of::<R>())
            .map(|e| unsafe  { &mut *e.data.cast::<R>() })
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        use std::alloc::dealloc;
        let data = self.0.remove(&TypeId::of::<R>())?;
        let value = unsafe { data.data.cast::<R>().read() };
        unsafe { dealloc(data.data, data.layout); }
        std::mem::forget(data);
        Some(value)
    }

    pub fn contains<R: Resource>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<R>())
    }
}