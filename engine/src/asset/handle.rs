use std::{any::TypeId, marker::PhantomData, sync::{Arc, atomic::{AtomicU32, Ordering}}};

use crate::asset::asset::{Asset, AssetIndex};

pub struct StrongHandle {
    index : AssetIndex,
    type_id : TypeId,
}



impl StrongHandle {
    pub(crate) fn new(index: AssetIndex, type_id : TypeId) -> Self {
        Self { index, type_id }
    }

    pub fn index(&self) -> AssetIndex { self.index }
}

impl<A : Asset> Into<Handle<A>> for Arc<StrongHandle> {
    fn into(self) -> Handle<A> {
        Handle {
            strong_handle: self,
            _phantom: PhantomData 
        }
    }
}


pub struct Handle<A : Asset> {
    strong_handle : Arc<StrongHandle>,
    _phantom : PhantomData<A>
}

impl<A: Asset> Handle<A> {
    pub(crate) fn new(index: AssetIndex) -> Self {
        Self { 
            strong_handle: Arc::new(StrongHandle { index, type_id: TypeId::of::<A>() }), 
            _phantom : PhantomData }
    }

    pub fn index(&self) -> AssetIndex { self.strong_handle.index() }
}

impl<A : Asset> Clone for Handle<A> {
    fn clone(&self) -> Self {
        Self::new(self.index().clone())
    }
}

impl<A : Asset> Into<StrongHandle> for Handle<A> {
    fn into(self) -> StrongHandle {
        StrongHandle::new(self.index(), TypeId::of::<A>())
    }
}
