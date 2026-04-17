use std::{cell::UnsafeCell, collections::HashMap};

use crate::ecs::{change_detection::tick::Tick, components::{Component, StorageType, id::ComponentId, info::ComponentInfo}, entities::id::EntityId, storage::column::Column};

pub mod column;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableId(u32);

impl TableId {
    #[inline]
    pub const fn from_u32(index: u32) -> Self {
        Self(index)
    }

    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}

pub(crate) struct TableBuilder {
    columns: HashMap<ComponentId, Column>,
    entities: Vec<EntityId>,
}

impl TableBuilder {
    pub fn with_capacity(capacity: usize, column_capacity: usize) -> Self {
        Self {
            columns: HashMap::with_capacity(column_capacity),
            entities: Vec::with_capacity(capacity),
        }
    }

    pub fn add_column(&mut self, component_info: &ComponentInfo) {
        self.columns.insert(
            component_info.id(),
            Column::with_capacity(self.entities.capacity(), component_info.layout(), component_info.drop()),
        );
    }

    #[must_use]
    pub fn build(self) -> Table {
        Table {
            columns: self.columns,
            entities: self.entities,
        }
    }
}


pub struct Table {
    pub columns: HashMap<ComponentId, Column>,
    entities: Vec<EntityId>,
}

impl Table {
    pub fn add_entity(&mut self, id : EntityId) -> usize {
        let row = self.entities.len();
        self.entities.push(id);
        row
    }

    pub fn insert<T : Component>(&mut self, component_id: ComponentId, value : T, tick : Tick) {
        if let Some(col) = self.columns.get_mut(&component_id) {
            col.push( value, tick);
            //self.entities.push(entity);
        }
    }

    pub fn swap_remove(&mut self, row : usize) -> Option<EntityId> {
        self.entities.swap_remove(row);
        for col in self.columns.values_mut() {
            col.swap_remove(row);
        }
        self.entities.get(row).copied()
    }

    pub fn entities(&self) -> &[EntityId] {
        &self.entities
    } 

    pub fn len(&self) -> usize {
        self.entities.len()
    } 

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn has_component(&self, component_id: ComponentId) -> bool {
        self.columns.contains_key(&component_id)
    }

    pub fn get<T: Component>(&self, component_id: ComponentId, row: usize) -> Option<&T> {
        self.columns.get(&component_id)?.get(row)
    }

    pub fn get_column(&self, component_id: ComponentId) -> Option<&Column> {
        self.columns.get(&component_id)
    }

    pub fn get_column_mut(&mut self, component_id: ComponentId) -> Option<&mut Column> {
        self.columns.get_mut(&component_id).map(|d| d)
    }

    pub unsafe fn get_slice<T : Component>(&self, component_id: ComponentId) -> &[UnsafeCell<T>] {
        match self.get_column(component_id) {
            Some(column) => unsafe { column.get_slice::<T>() },
            None => &[],
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.entities.clear();
        for _col in self.columns.values() {
            //TODO:implement drop
        }
    }
}

#[derive(Default)]
pub struct Storages {
    pub tables: Vec<Table>
}

impl Storages {
    pub fn len(&self) -> usize {
        self.tables.len()
    }

    pub fn insert_table(&mut self, table : Table) {
        self.tables.push(table)
    }

    pub fn prepare_component(&mut self, component: &ComponentInfo) {
        match component.storage_type() {
            StorageType::Table => {
                //Nothing
            }
            StorageType::SparseSet => {
                todo!("Implement")
            }
        }
    }
}
