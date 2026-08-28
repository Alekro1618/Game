use std::{
    any::TypeId,
    marker::PhantomData,
    sync::Arc,
};

use crate::asset::{AssetPath, asset::{Asset, AssetIndex, AssetIndexAllocator}};

#[derive(Clone, Debug)]
pub struct AssetHandleProvider {
    pub(crate) allocator: Arc<AssetIndexAllocator>,
    pub(crate) type_id: TypeId,
}

impl AssetHandleProvider {
    pub(crate) fn new(type_id: TypeId, allocator: Arc<AssetIndexAllocator>) -> Self {
        Self { type_id, allocator }
    }

    pub fn reserve_handle(&self, path : AssetPath) -> Arc<StrongHandle> {
        let index = self.allocator.reserve();
        self.get_handle(index, path)
    }

    pub(crate) fn get_handle(&self, index: AssetIndex, path : AssetPath) -> Arc<StrongHandle> {
        Arc::new(StrongHandle {
            index,
            type_id: self.type_id,
            path
        })
    }
}

#[derive(Clone, Copy)]
pub struct StrongHandle {
    index: AssetIndex,
    type_id: TypeId,
    path : AssetPath
}

impl StrongHandle {
    pub(crate) fn new(index: AssetIndex, type_id: TypeId, path : AssetPath) -> Self {
        Self { index, type_id, path }
    }

    pub fn index(&self) -> AssetIndex {
        self.index
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn path(&self) -> AssetPath {
        self.path
    }
}

impl<A: Asset> Into<Handle<A>> for Arc<StrongHandle> {
    fn into(self) -> Handle<A> {
        Handle {
            strong_handle: self,
            _phantom: PhantomData,
            lazy: false
        }
    }
}

pub struct Handle<A: Asset> {
    strong_handle: Arc<StrongHandle>,
    lazy : bool,
    _phantom: PhantomData<A>,
}

impl<A: Asset> Handle<A> {
    pub(crate) fn new(index: AssetIndex, path : AssetPath) -> Self {
        Self {
            strong_handle: Arc::new(StrongHandle {
                index,
                type_id: TypeId::of::<A>(),
                path
            }),
            lazy : false,
            _phantom: PhantomData,
        }
    }

    pub fn is_lazy(&self) -> bool {
        self.lazy
    }

    pub fn lazy(mut self) -> Self {
        self.lazy = true;
        self
    }

    pub fn index(&self) -> AssetIndex {
        self.strong_handle.index()
    }

    pub fn path(&self) -> AssetPath {
        self.strong_handle.path()
    }
}

impl<A: Asset> Clone for Handle<A> {
    fn clone(&self) -> Self {
        Self::new(self.index().clone(), self.path().clone())
    }
}

impl<A: Asset> Into<StrongHandle> for Handle<A> {
    fn into(self) -> StrongHandle {
        StrongHandle::new(self.index(), TypeId::of::<A>(), self.path())
    }
}
