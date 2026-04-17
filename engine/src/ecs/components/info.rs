use std::{alloc::Layout, any::{Any, TypeId}, borrow::Cow, collections::HashMap, mem::needs_drop, sync::PoisonError};

use crate::ecs::{components::{Component, StorageType, id::ComponentId, ComponentMutability}, hash::NoOpHash, lifecycle::ComponentHooks};

#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub id: ComponentId,
    pub descriptor: ComponentDescriptor,
    pub hooks: ComponentHooks,
}

impl ComponentInfo {
    #[inline]
    pub fn id(&self) -> ComponentId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.descriptor.name.clone()
    }

    #[inline]
    pub fn mutable(&self) -> bool {
        self.descriptor.mutable
    }

    #[inline]
    pub fn type_id(&self) -> Option<TypeId> {
        self.descriptor.type_id
    }

    #[inline]
    pub fn layout(&self) -> Layout {
        self.descriptor.layout
    }

    #[inline]
    pub fn drop(&self) -> Option<unsafe fn(*mut u8)> {
        self.descriptor.drop
    }

    #[inline]
    pub fn storage_type(&self) -> StorageType {
        self.descriptor.storage_type
    }

    #[inline]
    pub fn is_send_and_sync(&self) -> bool {
        self.descriptor.is_send_and_sync
    }

    pub(crate) fn new(id: ComponentId, descriptor: ComponentDescriptor) -> Self {
        ComponentInfo {
            id,
            descriptor,
            hooks: Default::default()
        }
    }

    pub fn hooks(&self) -> &ComponentHooks {
        &self.hooks
    }
}

#[derive(Clone, Debug)]
pub struct ComponentDescriptor {
    name: &'static str,
    storage_type: StorageType,
    is_send_and_sync: bool,
    type_id: Option<TypeId>,
    layout: Layout,
    drop: Option<for<'a> unsafe fn(*mut u8)>,//TODO: Change to different pointer 
    mutable: bool
}

impl ComponentDescriptor {
    unsafe fn drop_ptr<T>(x: *mut u8) {
        unsafe {
            x.drop_in_place();
        }
    }

    pub fn new<T: Component>() -> Self {
        Self {
            name: "", //TOOD: GET NAME FROM TYPE
            storage_type: T::STORAGE_TYPE,
            is_send_and_sync: true,
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            drop: needs_drop::<T>().then_some(Self::drop_ptr::<T> as _),
            mutable: T::Mutability::MUTABLE
        }
    }
    
    pub unsafe fn new_with_layout(
        name: impl Into<&'static str>,
        storage_type: StorageType,
        layout: Layout,
        drop: Option<for<'a> unsafe fn(*mut u8)>,
        mutable: bool
    ) -> Self {
        Self {
            name: name.into(),
            storage_type,
            is_send_and_sync: true,
            type_id: None,
            layout,
            drop,
            mutable
        }
    }

    pub(super) fn new_non_send<T: Any>(storage_type: StorageType) -> Self {
        Self {
            name: "", //TOOD: GET NAME FROM TYPE
            storage_type,
            is_send_and_sync: false,
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            drop: needs_drop::<T>().then_some(Self::drop_ptr::<T> as _),
            mutable: true
        }
    }

    #[inline]
    pub fn storage_type(&self) -> StorageType {
        self.storage_type
    }

    #[inline]
    pub fn type_id(&self) -> Option<TypeId> {
        self.type_id
    }

    /// Returns the name of the current component.
    #[inline]
    pub fn name(&self) -> &str {
        self.name.clone()
    }

    /// Returns whether this component is mutable.
    #[inline]
    pub fn mutable(&self) -> bool {
        self.mutable
    }

    fn initialize(&mut self, id: ComponentId, components: &mut Components) {
        //self.relationship_accessor.initialize(id, components);
    }
}

pub type TypeIdMap<V> = HashMap<TypeId, V, NoOpHash>;

#[derive(Debug, Default)]
pub struct Components {
    pub components: Vec<Option<ComponentInfo>>,
    pub indices: TypeIdMap<ComponentId>
}

impl Components {
    #[inline]
    pub(super) unsafe fn register_component_inner(
        &mut self,
        id: ComponentId,
        mut descriptor: ComponentDescriptor,
    ) {
        descriptor.initialize(id, self);
        let info = ComponentInfo::new(id, descriptor);
        let least_len = (id.id + 1) as usize;
        if self.components.len() < least_len {
            self.components.resize_with(least_len, || None);
        }
        let slot = self.components.get_mut(id.id as usize).unwrap();
        debug_assert!(slot.is_none());
        *slot = Some(info);
    }

    #[inline]
    pub fn get_info(&self, id: ComponentId) -> Option<&ComponentInfo> {
        self.components.get(id.id as usize).and_then(|info| info.as_ref())
    }

    #[inline]
    pub fn get_descriptor<'a>(&'a self, id: ComponentId) -> Option<Cow<'a, ComponentDescriptor>> {
        self.components
            .get(id.id as usize)
            .and_then(|info| 
                info
                    .as_ref()
                    .map(|info| 
                        Cow::Borrowed(&info.descriptor)
                    )
                )
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.num_registered()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn num_registered(&self) -> usize {
        self.components.len()
    }

    #[inline]
    pub fn any_registered(&self) -> bool {
        self.num_registered() > 0
    }

    #[inline]
    pub fn is_id_valid(&self, id: ComponentId) -> bool {
        self.components.get(id.id as usize).is_some_and(Option::is_some)
    }

    #[inline]
    pub fn get_valid_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.indices.get(&type_id).copied()
    }

    #[inline]
    pub fn get_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.indices.get(&type_id).copied().or_else(|| {
            None
        })
    }

    #[inline]
    pub fn component_id<T: Component>(&self) -> Option<ComponentId> {
        self.get_id(TypeId::of::<T>())
    }

    #[inline]
    pub(super) unsafe fn register_non_send_unchecked(
        &mut self,
        type_id: TypeId,
        component_id: ComponentId,
        descriptor: ComponentDescriptor,
    ) {
        unsafe {
            self.register_component_inner(component_id, descriptor);
        }
        let prev = self.indices.insert(type_id, component_id);
        debug_assert!(prev.is_none());
    }

    pub fn iter_registered(&self) -> impl Iterator<Item = &ComponentInfo> + '_ {
        self.components.iter().filter_map(Option::as_ref)
    }
}
