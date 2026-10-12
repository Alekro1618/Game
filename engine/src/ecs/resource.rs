use std::{
    alloc::{Layout, alloc}, cell::UnsafeCell, collections::HashMap,
};

use crate::{ecs::{
    change_detection::tick::Tick, components::{Component, Mutable, id::ComponentId}, storage::{blob_ref::BlobRef, element_ref::ElementRef},
}, utils::SparseArray};

pub trait Resource: Component<Mutability = Mutable> {}

struct ResourceData {
    data: *mut u8,
    drop_fn: unsafe fn(*mut u8),
    layout: std::alloc::Layout
}

impl ResourceData {
    pub fn as_blob_ref<'a>(&'a self) -> BlobRef<'a> {
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
pub struct ResourceEntities {
    entities : HashMap<ComponentId, ResourceData>,
    changed_ticks : SparseArray<UnsafeCell<Tick>, ComponentId>,
    added_ticks : SparseArray<UnsafeCell<Tick>, ComponentId>
}

impl ResourceEntities {

    #[inline]
    pub fn get<R: Resource>(&self, component_id: ComponentId) -> Option<&R> {
        self.entities
            .get(&component_id)
            .map(|e| unsafe { &*e.data.cast::<R>() })
    }

    pub fn insert<R: Resource>(&mut self, component_id: ComponentId, value: R, tick : Tick) {
        unsafe fn drop_resource<R>(ptr: *mut u8) {
            unsafe {
                ptr.cast::<R>().drop_in_place();
            }
        }

        let layout = Layout::new::<R>();
        let ptr = unsafe { alloc(layout) };

        unsafe {
            ptr.cast::<R>().write(value);
        }

        self.added_ticks.insert(component_id, UnsafeCell::new(tick));
        self.changed_ticks.insert(component_id, UnsafeCell::new(tick));

        self.entities.insert(
            component_id,
            ResourceData {
                data: ptr,
                drop_fn: drop_resource::<R>,
                layout,
            },
        );
    }

    #[inline]
    pub fn get_mut<R : Resource>(&mut self, component_id: ComponentId) -> Option<&mut R> {
        //Safety: We expect the user to not retrive the same resource twice at the time
        unsafe { 
            self.get_mut_inner(component_id)
        }
    }

    #[inline]
    unsafe fn get_mut_inner<R: Resource>(&mut self, component_id: ComponentId) -> Option<&mut R> {
        self.entities
            .get(&component_id)
            .map(|e| unsafe { &mut *e.data.cast::<R>() })
    }

    #[inline]
    pub fn get_mut_with_ticks<R : Resource>(&mut self, component_id: ComponentId) -> Option<(&mut R, &mut Tick, &mut Tick)> {
        //Safety: We expect the user to not retrive the same resource twice at the time
        unsafe {
            self.get_mut_with_ticks_inner(component_id)
        }
    }

    #[inline]
    unsafe fn get_mut_with_ticks_inner<R : Resource>(&mut self, component_id: ComponentId) -> Option<(&mut R, &mut Tick, &mut Tick)> {
        if let (Some(res), Some(added), Some(changed)) = (
            self.entities
                .get(&component_id)
                .map(|e| unsafe { &mut *e.data.cast::<R>() }),
            self.added_ticks
                .get(component_id)
                .map(|t|unsafe { &mut *t.get() }),
            self.changed_ticks
                .get(component_id)
                .map(|t| unsafe { &mut *t.get() })
        ) {
            Some((res, added, changed))
        } else {
            None
        }
    }

    #[inline]
    pub fn remove<R: Resource>(&mut self, component_id: ComponentId) -> Option<R> {
        use std::alloc::dealloc;
        let data = self.entities.remove(&component_id)?;
        self.added_ticks.remove(component_id);
        self.changed_ticks.remove(component_id);

        let value = unsafe { data.data.cast::<R>().read() };
        unsafe {
            dealloc(data.data, data.layout);
        }
        std::mem::forget(data);
        Some(value)
    }

    #[inline]
    pub fn contains<R: Resource>(&self, component_id: ComponentId) -> bool {
        self.entities.contains_key(&component_id)
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = ElementRef<'a>> {
        self.entities.iter().map(|(component_id, resource_data)| {
            ElementRef::from(component_id.clone(), resource_data.as_blob_ref())
        })
    }
}
