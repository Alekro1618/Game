use std::{alloc::{Layout, alloc}, any::TypeId, collections::HashMap};

use crate::ecs::{components::{Component, Mutable, id::ComponentId}, storage::{blob_ref::BlobRef, element_ref::ElementRef}};


pub trait Resource: Component<Mutability = Mutable> {}

struct ResourceData {
    data: *mut u8,
    drop_fn: unsafe fn(*mut u8),
    layout: std::alloc::Layout,
}

impl ResourceData {
    pub fn as_blob_ref<'a>(&'a self) -> BlobRef<'a>  {
        BlobRef::new(self.data, self.layout, Some(self.drop_fn))
    }
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
pub struct ResourceEntities(HashMap<ComponentId, ResourceData>);

impl ResourceEntities {
    pub fn get<R: Resource>(&self, component_id : ComponentId) -> Option<&R> {
        self.0
            .get(&component_id)
            .map(|e| unsafe  { &*e.data.cast::<R>() })
    }

    pub fn insert<R: Resource>(&mut self, component_id : ComponentId, value : R) {
        unsafe fn drop_resource<R>(ptr: *mut u8) { unsafe {
            ptr.cast::<R>().drop_in_place();
        }}

        let layout = Layout::new::<R>();
        let ptr = unsafe { alloc(layout) };

        unsafe { ptr.cast::<R>().write(value); }

        self.0.remove(&component_id);

        self.0.insert(component_id, ResourceData {
            data: ptr,
            drop_fn: drop_resource::<R>,
            layout,
        });
    }

    pub fn get_mut<R: Resource>(&mut self, component_id : ComponentId) -> Option<&mut R> {
        self.0
            .get(&component_id)
            .map(|e| unsafe  { &mut *e.data.cast::<R>() })
    }

    pub fn remove<R: Resource>(&mut self, component_id : ComponentId) -> Option<R> {
        use std::alloc::dealloc;
        let data = self.0.remove(&component_id)?;
        let value = unsafe { data.data.cast::<R>().read() };
        unsafe { dealloc(data.data, data.layout); }
        std::mem::forget(data);
        Some(value)
    }

    pub fn contains<R: Resource>(&self, component_id : ComponentId) -> bool {
        self.0.contains_key(&component_id)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = ElementRef<'a>> {
        self.0
            .iter()
            .map(|(component_id, resource_data)| {
                ElementRef::from(*component_id, resource_data.as_blob_ref())
            })
    }

}