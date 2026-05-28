use std::{collections::{HashMap, HashSet}, fmt::{Debug, Display}, ops::{Index, IndexMut}};

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
    entities_vec: Vec<EntityId>,
    components : Vec<ComponentId>
}

impl Archetype {
    pub fn new(id : ArchetypeId, component_ids: Vec<ComponentId>, table_id: u32) -> Self {
        Self {
            id,
            table_id,
            entities: HashMap::new(),
            entities_vec: Vec::new(),
            components: component_ids
        }
    }

    pub fn id(&self) -> ArchetypeId {
        self.id
    }

    pub fn entites(&self) -> Vec<EntityId> {
        self.entities.keys().map(|e| *e).collect()
    }

    pub fn archetype_components(&self) -> &Vec<ComponentId> {
        &self.components
    }

    pub fn contains(&self, id: &ComponentId) -> bool {
        self.components.contains(id)
    }

    pub fn contains_all(&self, ids: &Vec<ComponentId>) -> bool {
        ids
            .iter()
            .all(|component_id| self.components.contains(component_id))
    }

    pub fn contains_any(&self, ids: &Vec<ComponentId>) -> bool {
        ids
            .iter()
            .all(|component_id| self.components.contains(component_id))
    }

    pub fn table_id(&self) -> u32 {
        self.table_id
    }

    pub fn row(&self, entity: EntityId) -> Option<&u32> {
        self.entities.get(&entity)
    }

    pub fn insert(&mut self, entity: EntityId, row : u32) -> EntityLocation {
        self.entities.insert(entity, row);
        self.entities_vec.insert(row as usize, entity);
        EntityLocation { 
            archetype_id: self.id, 
            table_id: self.table_id, 
            table_row: row }
    }

    #[inline]
    pub fn swap_remove(&mut self, entity: EntityId, row: u32) -> (Option<EntityId>, u32) {
        let is_last = row as usize == self.entities.len() - 1;
        if is_last {
            self.entities.remove(&entity).unwrap();
            (None, row)
        } else {
            self.entities.remove(&entity).unwrap();
            let last_entity_id = self
                .entities
                .iter()
                .max_by_key(|(_, value)| **value );

            if let Some((&entity_id, _)) = last_entity_id {
                self.entities.insert(entity_id, row);
                (Some(entity_id), row)
            } else {
                (None, row)
            }
            
        }
    }
}

impl Display for Archetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.components.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, ",{}", item)?;
            }
        }
        write!(f, "]:{{")?;
        let mut iter = self.entities.iter();
        if let Some(first) = iter.next() {
            write!(f, "(e: {}, r: {})", first.0, first.1)?;
            for item in iter {
                write!(f, ",(e: {}, r: {})", item.0, item.1)?;
            }
        }
        write!(f, "}}")
    }
}

#[derive(Hash, PartialEq, Eq)]
struct ArchetypeComponents {
    components: Box<[ComponentId]>,
}

impl ArchetypeComponents {
    fn contains(&self, component_id : &ComponentId) -> bool {
        self.components.contains(component_id)
    }
}

impl Display for ArchetypeComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.components.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        
        for component_id in iter {
            write!(f, ",{}", component_id)?;
        }
        write!(f, "]")
    }
}


#[allow(dead_code)]
pub struct Archetypes {
    pub archetypes: Vec<Archetype>,
    by_components: HashMap<ArchetypeComponents, ArchetypeId>,
}

impl Archetypes {
    pub fn new() -> Self {
        Self {
            archetypes : Vec::new(),
            by_components : HashMap::new(),
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
        table_components: Vec<ComponentId>
    ) -> ArchetypeId {
        let archetypes = &mut self.archetypes;

        let id = ArchetypeId::new(archetypes.len());
        archetypes.push(Archetype::new(
            id,
            table_components.clone(),
            table_id,
        ));

        self.by_components.insert(
            ArchetypeComponents {
                components: table_components.into_boxed_slice() 
            },
            id
        );

        id
    }

    pub(crate) fn get_id(&self, components: Vec<ComponentId>) -> Option<ArchetypeId> {
        let archetype_components = ArchetypeComponents {
            components: components.into_boxed_slice()
        };
        
        match self.by_components.get(&archetype_components) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub(crate) fn contains_components(&self, components: Vec<ComponentId>) -> bool {
        let archetype_components = ArchetypeComponents {
            components: components.into_boxed_slice()
        };
        self.by_components.contains_key(&archetype_components)
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

impl Display for Archetypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.archetypes.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, ",\n{}", item)?;
            }
        }
        write!(f, "")
    }
}
