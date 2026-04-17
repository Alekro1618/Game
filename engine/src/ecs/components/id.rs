use std::{any::TypeId, collections::{HashMap, hash_map::Iter}, fmt::Display, hash::Hash};


use crate::ecs::{components::info::TypeIdMap, hash::NoOpHash};

/// Stores metadata for a type of component or resource stored in a specific [`World`](crate::world::World).

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ComponentId {
    pub id: u32
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Into<u32> for ComponentId {
    fn into(self) -> u32 {
        self.id
    }
}

impl From<u32> for ComponentId {
    fn from(num : u32) -> Self {
        Self { id: num }
    }
}

impl ComponentId {
    pub const PLACEHOLDER : Self = ComponentId { id : 0 };

    pub const fn new(id : u32) -> Self {
        Self {
            id
        }
    }

    #[inline(always)]
    pub const fn to_bits(self) -> u32 {
        self.id
    }
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id.to_string().as_str())
    }
}


#[derive(PartialEq, Eq, Clone, Default)]
pub struct ComponentIdsArray {
    component_ids : Vec<ComponentId>,
    type_map : TypeIdMap<ComponentId>
    
}

impl ComponentIdsArray {
    pub fn from(type_map : TypeIdMap<ComponentId>) -> Self {
        Self {
            component_ids: type_map
                .iter()
                .map(|c| c.1)
                .cloned()
                .collect(),
            type_map: type_map
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a TypeId, &'a ComponentId)>{ 
        self.into_iter()
    }

    pub fn contains(&self, component_id : ComponentId) -> bool {
        self.component_ids.contains(&component_id)
    }

    pub fn get(&self, type_id : TypeId) -> Option<ComponentId> {
        self.type_map.get(&type_id).map(|c| *c)
    }

    pub fn len(&self) -> usize {
        self.component_ids.len()
    }
}

impl Hash for ComponentIdsArray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.component_ids.hash(state);
    }
}

impl FromIterator<(TypeId, ComponentId)> for ComponentIdsArray {
    fn from_iter<T: IntoIterator<Item = (TypeId, ComponentId)>>(iter: T) -> Self {
        let mut type_map = HashMap::with_capacity_and_hasher(
            0, 
            NoOpHash
        );
        let mut component_ids = Vec::new();

        for (type_id, comp_id) in iter {
            type_map.insert(type_id, comp_id);
            component_ids.push(comp_id);
        }

        Self {
            type_map,
            component_ids,
        }
    }
}

impl<'a> IntoIterator for &'a ComponentIdsArray {
    type Item = (&'a TypeId, &'a ComponentId);

    type IntoIter = Iter<'a, TypeId, ComponentId>;

    fn into_iter(self) -> Self::IntoIter {
        self.type_map.iter()
    }
}