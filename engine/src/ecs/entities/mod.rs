use crate::{ecs::{archetype::ArchetypeId, entities::id::EntityId}, error::error::{Error, ErrorKind}};
use std::{hash::Hash};

pub mod id;
mod hash;
pub use hash::*;

pub struct Entity {
    id: EntityId
}

impl PartialEq for Entity {
    #[inline]
    fn eq(&self, other: &Entity) -> bool {
        self.to_bits() == other.to_bits()
    }
}

impl Eq for Entity {}

impl PartialOrd for Entity {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entity {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_bits().cmp(&other.to_bits())
    }
}

impl Hash for Entity {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.to_bits().hash(state);
    }
}

impl Entity {
    #[inline(always)]
    pub const fn from_id(
        id: EntityId
    ) -> Entity {
        Self { id }
    }

    pub const PLACEHOLDER: Self = Self::from_id(EntityId::PLACEHOLDER);

    #[inline(always)]
    pub fn to_bits(&self) -> u64 {
        self.id.to_bits() as u64
    }

    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        let id = EntityId::from(bits);
        Entity { id }
    }

    #[inline]
    pub const fn id(self) -> EntityId {
        self.id
    }

}

#[derive(Debug, Default)]
pub struct EntityAllocator {
    next_index: u32,
    free_list: Vec<(u32, u32)>,
}

impl EntityAllocator {
    pub fn allocate(&mut self) -> EntityId {
        if let Some((id, gnr)) = self.free_list.pop() {
            EntityId::new(id, gnr)
        } else {
            let id = self.next_index;
            self.next_index += 1;
            EntityId::new(id, 0)
        }
    }

    pub fn free(&mut self, entity: EntityId) {
        self.free_list.push((entity.id(), entity.gnr() + 1));
    }
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityLocation {
    pub archetype_id: ArchetypeId,
    pub table_id: u32,
    pub table_row: u32,
}

#[derive(Copy, Clone, Debug)]
struct EntityMeta {
    generation: u32,
    location: Option<EntityLocation>,
    spawned: bool,
}

impl EntityMeta {
    const FRESH: EntityMeta = EntityMeta {
            generation: 0,
            location: None,
            spawned: false
    };

    pub fn new(generation : u32, location : EntityLocation, spawned : bool) -> Self {
        Self {
            generation,
            location: Some(location),
            spawned
        }
    }

    pub fn new_no_location(generation : u32, spawned : bool) -> Self {
        Self {
            generation,
            location: None,
            spawned
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entities {
    meta: Vec<EntityMeta>,
    count: usize,
}
impl Entities {
    pub fn new() -> Self {
        Self { meta: Vec::new(), count: 0 }
    }

    pub fn alloc(&mut self, index: u32, generation: u32) {
        let idx = index as usize;
        if idx >= self.meta.len() {
            self.meta.resize(idx + 1, EntityMeta::new_no_location(0, false));
        }
        self.meta[idx] = EntityMeta::new_no_location(generation, true);
        self.count += 1;
    }

    pub fn set_location(&mut self, entity: EntityId, location : EntityLocation) {
        self.meta[entity.id() as usize].location = Some(location);
    }

    pub fn free(&mut self, entity: EntityId) -> bool {
        let idx = entity.id() as usize;
        if let Some(slot) = self.meta.get_mut(idx) {
            if slot.generation == entity.gnr() && slot.spawned {
                slot.spawned = false;
                self.count -= 1;
                return true;
            }
        }
        false
    }

    pub fn get_spawned(&self, entity: EntityId) -> Result<EntityLocation, Error> {
        let meta = self.meta.get(entity.id() as usize);
        let meta = meta.unwrap_or(&EntityMeta::FRESH);
        if entity.gnr() != meta.generation {
            return Err(Error::new(ErrorKind::ECS, "Invalid entity error"));
        };

        meta.location
            .ok_or(
                Error::new(ErrorKind::ECS, "Valid entity but not spawned")
            )
    }

    pub fn is_alive(&self, entity: EntityId) -> bool {
        self.meta
            .get(entity.id() as usize)
            .map(|&meta| meta.generation == entity.gnr() && meta.spawned)
            .unwrap_or(false)
    }

    pub fn count_spawned(&self) -> u32 {
        self.len() as u32
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

