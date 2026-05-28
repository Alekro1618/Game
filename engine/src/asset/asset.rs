use std::{collections::HashMap, fmt::Display, sync::{Arc, atomic::AtomicU32}};

use crate::{asset::handle::Handle, ecs::{components::{Component, Mutable}, resource::Resource}, error::error::{Error, ErrorKind}};

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct AssetIndex {
    pub(crate) generation: u32,
    pub(crate) index: u32,
}

impl Display for AssetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl AssetIndex {
    pub fn to_bits(self) -> u64 {
        let Self { generation, index } = self;
        ((generation as u64) << 32) | index as u64
    }

    pub fn from_bits(bits: u64) -> Self {
        let index = ((bits << 32) >> 32) as u32;
        let generation = (bits >> 32) as u32;
        Self { generation, index }
    }
}


pub trait Asset : Send + Sync + 'static {}



#[derive(Default)]
enum Entry<A: Asset> {
    #[default]
    None,
    Some { value: Option<A>, generation: u32 },
}

struct DenseAssetStorage<A: Asset> {
    storage: Vec<Entry<A>>,
    len: u32,
    free : Vec<AssetIndex>,
    next : Arc<AtomicU32>
}

impl<A: Asset> Default for DenseAssetStorage<A> {
    fn default() -> Self {
        Self { 
            storage: Default::default(),
            free: Vec::new(),
            len: Default::default(),
            next: Arc::new(AtomicU32::new(0))
        }
    }
}

impl<A: Asset> DenseAssetStorage<A> {
    // Returns the number of assets stored.
    pub(crate) fn len(&self) -> usize {
        self.len as usize
    }

    // Returns `true` if there are no assets stored.
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert the value at the given index. Returns true if a value already exists (and was replaced)
    pub(crate) fn insert(
        &mut self,
        index: AssetIndex,
        asset: A,
    ) -> Result<bool, Error> {
        let entry = &mut self.storage[index.index as usize];
        if let Entry::Some { value, generation } = entry {
            if *generation == index.generation {
                let exists = value.is_some();
                if !exists {
                    self.len += 1;
                }
                *value = Some(asset);
                Ok(exists)
            } else {
                Err(Error::new(ErrorKind::Asset, format!("Occupied entry for index {}", index)))
            }
        } else {
            Err(Error::new(ErrorKind::Asset, format!("No entry for index {}", index)))
        }
    }

    pub fn reserve(&mut self) -> AssetIndex {
        if let Some(mut index) = self.free.pop() {
            index.generation += 1;
            index
        } else {
            AssetIndex {
                index: self
                    .next
                    .fetch_add(1, core::sync::atomic::Ordering::Relaxed),
                generation: 0,
            }
        }
    }

    pub(crate) fn remove_dropped(&mut self, index: AssetIndex) -> Option<A> {
        self.remove_internal(index, |dense_storage| {
            dense_storage.storage[index.index as usize] = Entry::None;
        })
    }

    pub(crate) fn remove_still_alive(&mut self, index: AssetIndex) -> Option<A> {
        self.remove_internal(index, |_| {})
    }

    fn remove_internal(
        &mut self,
        index: AssetIndex,
        removed_action: impl FnOnce(&mut Self),
    ) -> Option<A> {
        let value = match &mut self.storage[index.index as usize] {
            Entry::None => return None,
            Entry::Some { value, generation } => {
                if *generation == index.generation {
                    self.free.push(index);
                    value.take().inspect(|_| self.len -= 1)
                } else {
                    return None;
                }
            }
        };
        removed_action(self);
        value
    }

    pub(crate) fn get(&self, index: AssetIndex) -> Option<&A> {
        let entry = self.storage.get(index.index as usize)?;
        match entry {
            Entry::None => None,
            Entry::Some { value, generation } => {
                if *generation == index.generation {
                    value.as_ref()
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn contains(&self, index : AssetIndex) -> bool {
        self.storage.get(index.index as usize).is_some()
    }

    pub(crate) fn get_mut(&mut self, index: AssetIndex) -> Option<&mut A> {
        let entry = self.storage.get_mut(index.index as usize)?;
        match entry {
            Entry::None => None,
            Entry::Some { value, generation } => {
                if *generation == index.generation {
                    value.as_mut()
                } else {
                    None
                }
            }
        }
    }
}

pub struct Assets<A: Asset> {
    dense_storage: DenseAssetStorage<A>,
    hash_map: HashMap<AssetIndex, A>,
    duplicate_handles: HashMap<AssetIndex, u16>,
}

impl<A: Asset> Component for Assets<A> {
    type Mutability = Mutable;
}

impl<A: Asset> Resource for Assets<A> {}

impl<A: Asset> Default for Assets<A> {
    fn default() -> Self {
        let dense_storage = DenseAssetStorage::<A>::default();
        Self {
            dense_storage,
            hash_map: Default::default(),
            duplicate_handles: Default::default(),
        }
    }
}

impl<A: Asset> Assets<A> {
    pub fn insert(
        &mut self,
        index: AssetIndex,
        asset: A,
    ) -> Result<(), Error> {
        self.insert_with_index(index, asset).map(|_| ())
    }

    pub fn contains(&self, index : AssetIndex) -> bool {
        self.dense_storage.contains(index)
    }


    pub(crate) fn insert_with_index(
        &mut self,
        index: AssetIndex,
        asset: A,
    ) -> Result<bool, Error> {
        let replaced = self.dense_storage.insert(index, asset)?;
        Ok(replaced)
    }

    #[inline]
    pub fn add(&mut self, asset: impl Into<A>) -> Handle<A> {
        let index = self.dense_storage.reserve();
        self.insert_with_index(index, asset.into()).unwrap();
        Handle::new(index)
    }

   #[inline]
    pub fn get(&self, index : AssetIndex) -> Option<&A> {
       self.dense_storage.get(index)
    }

   #[inline]
    pub fn get_mut(&mut self, index : AssetIndex) -> Option<&mut A> {
        self.dense_storage.get_mut(index)
    }

    #[inline]
    pub fn remove(&mut self, index: AssetIndex) -> Option<A> {
        self.duplicate_handles.remove(&index);
        self.dense_storage.remove_still_alive(index)
    }

    pub(crate) fn remove_dropped(&mut self, index: AssetIndex) {
        match self.duplicate_handles.get_mut(&index) {
            None => {}
            Some(0) => {
                self.duplicate_handles.remove(&index);
            }
            Some(value) => {
                *value -= 1;
            }
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense_storage.is_empty() && self.hash_map.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.dense_storage.len() + self.hash_map.len()
    }
}
