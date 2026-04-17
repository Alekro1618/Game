use std::{collections::HashMap, ops::{Index, IndexMut}};

use crate::ecs::{components::id::{ComponentId, ComponentIdsArray}, entities::{EntityLocation, id::EntityId}};



#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ArchetypeId(u32);

impl ArchetypeId {
    pub const EMPTY: ArchetypeId = ArchetypeId(0);

    #[inline]
    pub const fn new(index: usize) -> Self {
        ArchetypeId(index as u32)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}


pub struct Archetype {
    id: ArchetypeId,
    table_id: u32,
    entities: HashMap<EntityId, u32>,
    component_ids : ComponentIdsArray
}

impl Archetype {
    pub fn new(id : ArchetypeId, component_ids: ComponentIdsArray, table_id: u32) -> Self {
        Self {
            id,
            table_id,
            entities: HashMap::new(),
            component_ids
        }
    }

    pub fn id(&self) -> ArchetypeId {
        self.id
    }

    pub fn component_ids(&self) -> &ComponentIdsArray {
        &self.component_ids
    }

    pub fn contains(&self, id: ComponentId) -> bool {
        self.component_ids.contains(id)
    }

    pub fn table_id(&self) -> u32 {
        self.table_id
    }

    pub fn row(&self, entity: EntityId) -> Option<&u32> {
        self.entities.get(&entity)
    }

    pub fn insert(&mut self, entity: EntityId, row : u32) -> EntityLocation {
        self.entities.insert(entity, row);
        EntityLocation { 
            archetype_id: self.id, 
            table_id: self.table_id, 
            table_row: row }
    }
}

#[allow(dead_code)]
pub struct Archetypes {
    pub archetypes: Vec<Archetype>,
    by_components: HashMap<ComponentIdsArray, ArchetypeId>,
    by_component: HashMap<ComponentId, Vec<ArchetypeId>>
}

impl Archetypes {
    pub fn new() -> Self {
        Self {
            archetypes : Vec::new(),
            by_components : HashMap::new(),
            by_component: HashMap::new()
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.archetypes.len()
    }

    #[inline]
    pub fn get(&self, id: ArchetypeId) -> Option<&Archetype> {
        self.archetypes.get(id.index())
    }

    #[inline]
    pub fn get_mut(&mut self, id: ArchetypeId) -> Option<&mut Archetype> {
        self.archetypes.get_mut(id.index())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Archetype> {
        self.archetypes.iter()
    }

    pub(crate) fn insert_id(
        &mut self,
        table_id: u32,
        table_components: &ComponentIdsArray
    ) -> ArchetypeId {
        let archetypes = &mut self.archetypes;

        let id = ArchetypeId::new(archetypes.len());
        archetypes.push(Archetype::new(
            id,
            table_components.clone(),
            table_id,
        ));
        self.by_components.insert(table_components.clone(), id);

        id
    }

    pub(crate) fn get_id(&self, table_components: &ComponentIdsArray) -> Option<ArchetypeId> {
        match self.by_components.get(table_components) {
            Some(id) => Some(*id),
            None => None,
        }
    }
}

impl Index<ArchetypeId> for Archetypes {
    type Output = Archetype;

    #[inline]
    fn index(&self, index: ArchetypeId) -> &Self::Output {
        &self.archetypes[index.index()]
    }
}

impl IndexMut<ArchetypeId> for Archetypes {
    #[inline]
    fn index_mut(&mut self, index: ArchetypeId) -> &mut Self::Output {
        &mut self.archetypes[index.index()]
    }
}
