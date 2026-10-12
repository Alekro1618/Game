use std::{
    any::TypeId,
    collections::HashMap,
    fmt::Display,
    sync::{Arc, atomic::AtomicU32},
};

use async_channel::{Receiver, Sender, unbounded};

use crate::{
    asset::{event::AssetEvent, handle::AssetHandleProvider}, ecs::{
        change_detection::params::Mut, components::{Component, Mutable}, message::message_writer::MessageWriter, resource::Resource,
    }, error::error::{Error, ErrorKind},
};

#[derive(Debug)]
pub(crate) struct AssetIndexAllocator {
    next_index: Arc<AtomicU32>,
    recycled_sender: Sender<AssetIndex>,
    recycled_receiver: Receiver<AssetIndex>,
}

impl Default for AssetIndexAllocator {
    fn default() -> Self {
        let (recycled_sender, recycled_receiver) = unbounded();
        Self {
            next_index: Default::default(),
            recycled_receiver,
            recycled_sender,
        }
    }
}

impl AssetIndexAllocator {
    pub fn reserve(&self) -> AssetIndex {
        match self.recycled_receiver.try_recv() {
            Ok(next_index) => next_index,
            Err(_) => AssetIndex {
                index: self
                    .next_index
                    .fetch_add(1, core::sync::atomic::Ordering::Relaxed),
                generation: 0,
            },
        }
    }

    pub fn free(&self, mut index: AssetIndex) {
        index.generation += 1;
        self.recycled_sender.try_send(index).unwrap();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AssetIndex {
    pub(crate) generation: u32,
    pub(crate) index: u32,
}

impl Into<usize> for AssetIndex {
    fn into(self) -> usize {
        self.index as usize
    }
}

impl From<usize> for AssetIndex {
    fn from(value: usize) -> Self {
        AssetIndex { generation: 0, index: value as u32 }
    }
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

pub trait Asset: Send + Sync + 'static {}

impl Asset for () {}

impl<A1: Asset, A2: Asset> Asset for (A1, A2) {}

#[derive(Default)]
pub enum Entry<A: Asset> {
    #[default]
    None,
    Some {
        value: Option<A>,
        generation: u32,
    },
}

struct DenseAssetStorage<A: Asset> {
    storage: Vec<Entry<A>>,
    len: u32,
    allocator: Arc<AssetIndexAllocator>,
}

impl<A: Asset> Default for DenseAssetStorage<A> {
    fn default() -> Self {
        Self {
            storage: Default::default(),
            len: 0,
            allocator: Default::default(),
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
    pub(crate) fn insert(&mut self, index: AssetIndex, asset: A) -> Result<bool, Error> {
        self.flush();
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
                Err(Error::new(
                    ErrorKind::Asset,
                    format!("Occupied entry for index {}", index),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::Asset,
                format!("No entry for index {}", index),
            ))
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
        let value = match self.storage.get_mut(index.index as usize) {
            Some(entry) => match entry {
                Entry::None => return None,
                Entry::Some { value, generation } => {
                    if *generation == index.generation {
                        value.take().inspect(|_| self.len -= 1)
                    } else {
                        return None;
                    }
                }
            },
            None => None,
        };
        removed_action(self);
        value
    }

    pub(crate) fn get_index_allocator(&self) -> Arc<AssetIndexAllocator> {
        self.allocator.clone()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Entry<A>> {
        self.storage.iter()
    }

    pub(crate) fn drain(&mut self) -> impl Iterator<Item = A> {
        self.storage
            .drain(..)
            .filter_map(|entry| match entry {
                Entry::None => None,
                Entry::Some { value, generation: _ } => Some(value),
            })
            .filter_map(|entry| entry)
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

    pub(crate) fn contains(&self, index: AssetIndex) -> bool {
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

    pub(crate) fn flush(&mut self) {
        let new_len = self
            .allocator
            .next_index
            .load(core::sync::atomic::Ordering::Relaxed);

        self.storage.resize_with(new_len as usize, || Entry::Some {
            value: None,
            generation: 0,
        });

        while let Ok(recycled) = self.allocator.recycled_receiver.try_recv() {
            let entry = &mut self.storage[recycled.index as usize];
            *entry = Entry::Some {
                value: None,
                generation: recycled.generation,
            };
        }
    }
}

pub struct Assets<A: Asset> {
    dense_storage: DenseAssetStorage<A>,
    handle_provider: AssetHandleProvider,
    duplicate_handles: HashMap<AssetIndex, u16>,
    queued_events : Vec<AssetEvent<A>>
}

impl<A: Asset> Component for Assets<A> {
    type Mutability = Mutable;
}

impl<A: Asset> Resource for Assets<A> {}

impl<A: Asset> Default for Assets<A> {
    fn default() -> Self {
        let dense_storage = DenseAssetStorage::<A>::default();
        let handle_provider =
            AssetHandleProvider::new(TypeId::of::<A>(), dense_storage.get_index_allocator());

        Self {
            dense_storage,
            duplicate_handles: Default::default(),
            handle_provider,
            queued_events: Vec::new(),
        }
    }
}

impl<A: Asset> Assets<A> {
    pub fn insert(&mut self, index: AssetIndex, asset: A) -> Result<(), Error> {
        self.insert_with_index(index, asset).map(|_| ())
    }

    pub fn contains(&self, index: AssetIndex) -> bool {
        self.dense_storage.contains(index)
    }

    pub fn get_handle_provider(&self) -> AssetHandleProvider {
        self.handle_provider.clone()
    }

    pub(crate) fn insert_with_index(&mut self, index: AssetIndex, asset: A) -> Result<bool, Error> {
        
        let replaced = self.dense_storage.insert(index, asset)?;
        if replaced {
            self.queued_events.push(AssetEvent::Modified { id: index });
        } else {
            self.queued_events.push(AssetEvent::Added { id: index });
        }

        Ok(replaced)
    }

    #[inline]
    pub fn get(&self, index: AssetIndex) -> Option<&A> {
        self.dense_storage.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: AssetIndex) -> Option<&mut A> {
        self.dense_storage.get_mut(index)
    }

    #[inline]
    pub fn remove(&mut self, index: AssetIndex) -> Option<A> {
        self.duplicate_handles.remove(&index);
        self.dense_storage.remove_still_alive(index)
    }

    pub fn remove_dropped(&mut self, index: AssetIndex) {
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
        self.dense_storage.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.dense_storage.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Entry<A>> {
        self.dense_storage.iter()
    }

    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item = A> {
        self.dense_storage.drain()
    }

    #[inline]
    pub fn iter_indicies(&self) -> impl Iterator<Item = AssetIndex> {
        self.dense_storage
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| match entry {
                Entry::None => None,
                Entry::Some { value: _, generation } => Some(AssetIndex {
                    generation: *generation,
                    index: i as u32,
                }),
            })
    }

    pub(crate) fn asset_events(
        mut assets: Mut<Self>,
        mut messages: MessageWriter<AssetEvent<A>>,
    ) {
        //TOOD: Batch. not one by one
        for event in assets.queued_events.drain(..) {
            messages.write(event);
        }
    }

    pub(crate) fn asset_events_condition(assets: &Self) -> bool {
        !assets.queued_events.is_empty()
    }
}
