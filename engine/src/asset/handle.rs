use std::{
    any::TypeId, collections::{HashMap, HashSet, VecDeque}, marker::PhantomData, sync::Arc,
};

use engine_macros::{Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::{asset::{AssetPath, asset::{Asset, AssetIndex, AssetIndexAllocator}, loader::LoadContext}, ecs::{reflect::{FromReflect, Reflect}, type_register::{FromType, GetTypeRegistration, ReflectFromReflect, ReflectHandle, TypeRegistration}}, utils::{SparseArray, SparseArrayIndex}};

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
        Handle::Strong {
            strong_handle: self,
            _phantom: PhantomData,
            lazy: false
        }
    }
}


pub struct ErasedHandle {
    strong_handle: Arc<StrongHandle>,
    lazy : bool,
}

impl Into<StrongHandle> for ErasedHandle {
    fn into(self) -> StrongHandle {
        StrongHandle::new(self.index(), self.type_id(), self.path())
    }
}

impl Into<ErasedHandle> for Arc<StrongHandle> {
    fn into(self) -> ErasedHandle {
        ErasedHandle {
            strong_handle: self,
            lazy: false
        }
    }
}


impl ErasedHandle {
    pub fn new(index : AssetIndex, path : AssetPath, type_id : TypeId) -> Self {
        Self {
            strong_handle: Arc::new(StrongHandle {
                 index, 
                 type_id, 
                 path }
            ),
            lazy : false
        }
    }

    pub fn lazy(mut self) -> Self {
        self.lazy = true;
        self
    }

    pub fn type_id(&self) -> TypeId {
        self.strong_handle.type_id
    }

    pub fn index(&self) -> AssetIndex {
        self.strong_handle.index
    }

    pub fn path(&self) -> AssetPath {
        self.strong_handle.path
    }

    pub fn typed<A : Asset>(self) -> Handle<A> {
        Handle::Strong { strong_handle: self.strong_handle, lazy: self.lazy, _phantom: PhantomData }
    }
}

impl Clone for ErasedHandle {
    fn clone(&self) -> Self {
        Self::new(self.index().clone(),  self.path().clone() ,self.type_id().clone())
    }
}

pub enum Handle<A: Asset> {
    Strong {
        strong_handle: Arc<StrongHandle>,
        lazy : bool,
        _phantom: PhantomData<A>,
    },
    Weak {
        path : AssetPath,
        lazy: bool,
    }
}

pub trait HasHandle {
    //We expect that after deserialization all handles are `Handle::Weak` and so needs to be replaced with `Handle::Strong`
    fn replace_handles(&mut self, context : &mut LoadContext);
}

impl<A: Asset> HasHandle for Handle<A> {
    fn replace_handles(&mut self, context : &mut LoadContext) {
        if matches!(&self, Weak) {
            let _ = core::mem::replace(self, context.asset_server.load(self.path()));
        }
    }
}

impl<T: HasHandle> HasHandle for Vec<T> {
    fn replace_handles(&mut self, context : &mut LoadContext) {
        self
            .iter_mut()
            .for_each(|handle| handle.replace_handles(context));
    }
}

impl<T: HasHandle> HasHandle for VecDeque<T> {
    fn replace_handles(&mut self, context : &mut LoadContext) {
        self
            .iter_mut()
            .for_each(|handle| handle.replace_handles(context));
    }
}

impl<K, T: HasHandle> HasHandle for HashMap<K, T> {
    fn replace_handles(&mut self, context : &mut LoadContext) {
        self
            .iter_mut()
            .for_each(|(_, handle)| handle.replace_handles(context));
    }
}

impl<I: SparseArrayIndex + From<usize>, T: HasHandle> HasHandle for SparseArray<T, I> {
    fn replace_handles(&mut self, context : &mut LoadContext) {
        self
            .iter_mut()
            .for_each(|(_, handle)| handle.replace_handles(context));
    }
}

impl<A: Asset> Reflect for Handle<A> {
    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    fn into_reflect(self) -> Box<dyn Reflect> {
        Box::new(self)
    }

    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, crate::error::error::Error> {
        Ok(Box::new(Clone::clone(self)))
    }

    fn name(&self) -> String {
        String::from(::std::any::type_name::<Handle<A>>())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type_id(&self) -> Option<TypeId> {
        Some(::std::any::TypeId::of::<Self>())
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<A : Asset> FromReflect for Handle<A> {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        reflect.downcast_ref().cloned()
    }
}

impl<A: Asset> Serialize for Handle<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                strong_handle.path.serialize(serializer)
            },
            Handle::Weak { path, lazy } => {
                path.serialize(serializer)
            },
        }
    }
}


impl<'de, A: Asset> Deserialize<'de> for Handle<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let path = AssetPath::deserialize(deserializer)?;
        Ok(Handle::Weak { path, lazy: false })
    }
}

impl<A: Asset> Default for Handle<A> {
    fn default() -> Self {
        Self::Strong { 
            strong_handle: Arc::new(StrongHandle {
                index: AssetIndex { generation: 0, index: u32::MAX },
                type_id: TypeId::of::<A>(),
                path: AssetPath::default(),
            }),
            lazy: false,
            _phantom: PhantomData
        }
    }
}

impl<A: Asset> Handle<A> {
    pub(crate) fn new(index: AssetIndex, path : AssetPath) -> Self {
        Self::Strong {
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
        match self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                *lazy
            },
            Handle::Weak { path, lazy } => {
                *lazy
            },
        }
    }

    pub fn unlazify(&mut self) {
        match self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                *lazy = false;
            },
            Handle::Weak { path, lazy } => {
                *lazy = false;
            },
        }
    }

    pub fn lazy(mut self) -> Self {
        match &mut self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                *lazy = true;
            },
            Handle::Weak { path, lazy } => {
                *lazy = true;
            },
        }
        self
    }

    pub fn index(&self) -> AssetIndex {
        match self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                strong_handle.index()
            },
            Handle::Weak { path, lazy } => {
                panic!("Couldn't get id for weak handle");
            },
        }
        
    }

    pub fn path(&self) -> AssetPath {
        match self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                strong_handle.path()
            },
            Handle::Weak { path, lazy } => {
                path.clone()
            },
        }
    }
}

impl<A: Asset> Clone for Handle<A> {
    fn clone(&self) -> Self {
        match self {
            Handle::Strong { strong_handle, lazy, _phantom } => {
                Self::new(strong_handle.index().clone(), strong_handle.path().clone())
            },
            Handle::Weak { path, lazy } => {
                Self::Weak { path: path.clone(), lazy: lazy.clone() }
            }
        }
        
    }
}

impl<A: Asset> Into<StrongHandle> for Handle<A> {
    fn into(self) -> StrongHandle {
        StrongHandle::new(self.index(), TypeId::of::<A>(), self.path())
    }
}
