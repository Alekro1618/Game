use std::{alloc::Layout, ops::Deref};

use crate::ecs::{components::id::ComponentId, storage::blob_ref::BlobRef};

#[derive(Clone, Copy)]
pub struct ElementRef<'a>(ComponentId, BlobRef<'a>);


impl<'a> ElementRef<'a> {
    pub fn new(component_id : ComponentId, ptr : *mut u8, layout : Layout, drop: Option<unsafe fn(*mut u8)>) -> Self {
        Self(component_id, BlobRef::new(ptr, layout, drop))
    }

    pub unsafe fn set<T>(&mut self, data : T) {
        unsafe {
            self.1.set_unchecked(data);
        }
    }

    pub fn from(component_id : ComponentId, blob : BlobRef<'a>) -> Self {
        Self(component_id, blob)
    }

    pub fn component_id(&self) -> ComponentId {
        self.0
    }

    pub unsafe fn get<T>(self) -> &'a T {
        unsafe {
            self.1.get_unchecked::<T>()
        }
    }

    pub unsafe fn get_mut<T>(&mut self) -> &mut T {
        unsafe {
            self.1.get_mut_unchecked::<T>()
        }
    }
}