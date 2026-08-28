use std::{collections::HashMap, slice::SliceIndex};

use crate::ecs::{
    change_detection::tick::Tick,
    components::{
        Component, StorageType,
        id::{ComponentId, ComponentIdMap},
        info::ComponentInfo,
    },
    entities::id::EntityId,
    hash::NoOpHash,
    storage::{
        column::Column,
        element_ref::{ElementRef, ElementRefMut},
        row::{RowRef, RowRefMut},
    },
};

mod blob_array;
pub mod blob_ref;
pub mod column;
pub mod element_ref;
pub mod row;

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
    columns: ComponentIdMap<Column>,
    entities: Vec<EntityId>,
}

impl TableBuilder {
    pub fn with_capacity(capacity: usize, column_capacity: usize) -> Self {
        Self {
            columns: HashMap::with_capacity_and_hasher(column_capacity, NoOpHash::default()),
            entities: Vec::with_capacity(capacity),
        }
    }

    pub fn add_column(&mut self, component_info: &ComponentInfo) {
        self.columns.insert(
            component_info.id(),
            Column::with_capacity(
                self.entities.capacity(),
                component_info.layout(),
                component_info.drop(),
            ),
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
    pub columns: ComponentIdMap<Column>,
    entities: Vec<EntityId>,
}

impl Table {
    pub fn add_entity(&mut self, id: EntityId) -> usize {
        let row = self.entities.len();
        self.entities.push(id);
        row
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn remove_entity(&mut self, row: usize) -> Option<EntityId> {
        if self.entities.len() <= row {
            return None;
        }

        Some(self.entities.remove(row))
    }

    pub fn insert<T: Component>(&mut self, component_id: ComponentId, value: T, tick: Tick) {
        if let Some(col) = self.columns.get_mut(&component_id) {
            // Safety: Caller is responsible for component id matching with the actual component type.
            // Panics if out of memory
            unsafe {
                col.push_unchecked(value, tick);
            }
        }
    }

    pub fn replace<T: Component>(
        &mut self,
        component_id: ComponentId,
        value: T,
        row: usize,
        tick: Tick,
    ) {
        if let Some(col) = self.columns.get_mut(&component_id) {
            // Safety: Caller is responsible for component id matching with the actual component type.
            // Row should be in bound.
            // Caller should use it only if the existing row is not null.
            // Panics if out of memory.
            unsafe {
                col.replace(value, row, tick);
            }
        }
    }

    pub fn remove(&mut self, component_id: ComponentId, row: usize, _tick: Tick) {
        if let Some(col) = self.columns.get_mut(&component_id) {
            // Safety: Caller is responsible for component id matching with the actual component type.
            // Row should be in bound.
            // Caller should use it only if the existing row is not null.
            // Panics if out of memory.
            let last_index = col.len() - 1;
            unsafe {
                col.swap_remove(last_index, row);
            }
        }
    }

    pub fn remove_whole_row(&mut self, row: usize, _tick: Tick) {
        for (_, col) in &mut self.columns {
            let last_index = col.len() - 1;
            unsafe {
                col.swap_remove(last_index, row);
            }
        }
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

    /// SAFETY: Should be used knowing beforehand the type of elements in the column and that row is in bounds.
    pub unsafe fn get<T: Component>(&self, component_id: ComponentId, row: usize) -> Option<&T> {
        unsafe {
            self.columns
                .get(&component_id)
                .map(|column| &*column.get(row).cast::<T>())
        }
    }

    /// SAFETY: Should be used knowing beforehand the type of elements in the column and that row is in bounds.
    pub unsafe fn get_mut<T: Component>(
        &mut self,
        component_id: ComponentId,
        row: usize,
    ) -> Option<&mut T> {
        unsafe {
            self.columns
                .get_mut(&component_id)
                .map(|column| &mut *column.get_mut(row).cast::<T>())
        }
    }

    pub fn get_column(&self, component_id: ComponentId) -> Option<&Column> {
        self.columns.get(&component_id)
    }

    pub unsafe fn get_row<'a>(&'a self, row: usize) -> Option<RowRef<'a>> {
        if self.len() <= row {
            return None;
        }

        unsafe {
            let elements: ComponentIdMap<ElementRef<'a>> = self
                .columns
                .iter()
                .map(|(component_id, column)| {
                    (
                        *component_id,
                        ElementRef::from(*component_id, column.get_as_blob_ref(row)),
                    )
                })
                .collect();

            Some(RowRef::from(elements))
        }
    }

    pub unsafe fn get_row_mut<'a>(&'a mut self, row: usize) -> Option<RowRefMut<'a>> {
        if self.len() <= row {
            return None;
        }

        unsafe {
            let elements: ComponentIdMap<ElementRefMut<'a>> = self
                .columns
                .iter()
                .map(|(component_id, column)| {
                    (
                        *component_id,
                        ElementRefMut::from(*component_id, column.get_as_blob_ref(row)),
                    )
                })
                .collect();

            Some(RowRefMut::from(elements))
        }
    }

    //Safety:
    pub unsafe fn move_to(&mut self, src_row: usize, dst_table: &mut Self, tick: Tick) {
        let ids: Vec<ComponentId> = dst_table.columns.keys().copied().collect();
        let last_index = self.entity_count();
        unsafe {
            for compononent_id in ids {
                if let (Some(src_col), Some(dst_col)) = (
                    self.columns.get_mut(&compononent_id),
                    dst_table.columns.get_mut(&compononent_id),
                ) {
                    src_col.move_to(dst_col, src_row, last_index, tick);
                }
            }
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.entities.clear();
        for col in self.columns.values() {
            drop(col.as_mut_ptr());
        }
    }
}

#[derive(Default)]
pub struct Storages {
    pub tables: Vec<Table>,
}

impl Storages {
    pub fn len(&self) -> usize {
        self.tables.len()
    }

    pub fn insert_table(&mut self, table: Table) {
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

    pub fn get_table(&self, table_id: usize) -> Option<&Table> {
        self.tables.get(table_id)
    }

    pub fn get_table_mut(&mut self, table_id: usize) -> Option<&mut Table> {
        self.tables.get_mut(table_id)
    }

    pub fn get_disjoint_table_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Option<[&mut Table; N]> {
        self.tables.get_disjoint_mut(indices).ok()
    }
}

#[cfg(test)]
mod tests {
    use std::{any::TypeId, iter};

    use crate::ecs::{
        change_detection::tick::Tick,
        components::{
            Component, Mutable, StorageType,
            info::Components,
            register::{ComponentIds, ComponentsRegistrator},
        },
        entities::{Entities, EntityAllocator},
        storage::{Table, TableBuilder},
    };

    #[derive(PartialEq, Eq, Debug)]
    pub struct A;
    impl Component for A {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct B;
    impl Component for B {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct C;
    impl Component for C {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct T(u32);
    impl Component for T {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[test]
    fn test_create_table() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let a = component_reg.register_component::<A>();
        let b = component_reg.register_component::<B>();
        let c = component_reg.register_component::<C>();

        let mut table = TableBuilder::with_capacity(0, 3);
        table.add_column(components.get_info(a).unwrap());
        table.add_column(components.get_info(b).unwrap());
        table.add_column(components.get_info(c).unwrap());
        let table = table.build();
        assert_eq!(table.columns.len(), 3);
        assert_eq!(table.entities.len(), 0);
    }

    #[test]
    fn test_insert_table() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let a = component_reg.register_component::<A>();
        let b = component_reg.register_component::<B>();
        let c = component_reg.register_component::<C>();

        let mut table = TableBuilder::with_capacity(0, 2);
        table.add_column(components.get_info(a).unwrap());
        table.add_column(components.get_info(b).unwrap());
        table.add_column(components.get_info(c).unwrap());
        let mut table = table.build();

        let mut entity_allocator = EntityAllocator::default();
        let mut entities = Entities::new();
        let entity = entity_allocator.allocate();
        entities.alloc(entity.id(), entity.gnr());
        let tick = Tick::new(0);
        table.add_entity(entity);
        table.insert(a, A, tick);
        table.insert(b, B, tick);
        table.insert(c, C, tick);
        unsafe {
            let a_: &A = table.get(a, 0).unwrap();
            let b_: &B = table.get(b, 0).unwrap();
            let c_: &C = table.get(c, 0).unwrap();

            assert_eq!(a_, &A);
            assert_eq!(b_, &B);
            assert_eq!(c_, &C);
        }
    }

    #[test]
    fn test_replace_table() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let t = component_reg.register_component::<T>();

        let mut table = TableBuilder::with_capacity(0, 1);
        table.add_column(components.get_info(t).unwrap());
        let mut table = table.build();

        let mut entity_allocator = EntityAllocator::default();
        let mut entities = Entities::new();
        let entity = entity_allocator.allocate();
        entities.alloc(entity.id(), entity.gnr());
        let tick = Tick::new(0);
        table.add_entity(entity);
        table.insert(t, T(10), tick);
        unsafe {
            let t_: &T = table.get(t, 0).unwrap();
            assert_eq!(t_, &T(10));
            table.replace(t, T(20), 0, tick);
            let t_: &T = table.get(t, 0).unwrap();
            assert_eq!(t_, &T(20));
        }
    }

    #[test]
    fn test_move_to_table() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let a = component_reg.register_component::<A>();
        let b = component_reg.register_component::<B>();

        let mut table1 = TableBuilder::with_capacity(0, 2);
        table1.add_column(components.get_info(a).unwrap());
        table1.add_column(components.get_info(b).unwrap());

        let mut table1 = table1.build();
        let mut table2 = TableBuilder::with_capacity(0, 2);

        table2.add_column(components.get_info(a).unwrap());
        table2.add_column(components.get_info(b).unwrap());

        let mut table2 = table2.build();

        let mut entity_allocator = EntityAllocator::default();
        let mut entities = Entities::new();
        let entity = entity_allocator.allocate();
        entities.alloc(entity.id(), entity.gnr());
        let tick = Tick::new(0);

        table1.add_entity(entity);
        table1.insert(a, A, tick);
        table1.insert(b, B, tick);
        unsafe {
            let a_: &A = table1.get(a, 0).unwrap();
            let b_: &B = table1.get(b, 0).unwrap();

            assert_eq!(a_, &A);
            assert_eq!(b_, &B);

            table2.add_entity(entity);
            table1.move_to(0, &mut table2, tick);
            let _ = table1.remove_entity(0);

            let a_: &A = table2.get(a, 0).unwrap();
            let b_: &B = table2.get(b, 0).unwrap();

            assert_eq!(a_, &A);
            assert_eq!(b_, &B);
            assert_eq!(table1.entity_count(), 0);
        }
    }

    #[test]
    fn test_select_row() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let a = component_reg.register_component::<A>();
        let b = component_reg.register_component::<B>();
        let t = component_reg.register_component::<T>();

        let mut table = TableBuilder::with_capacity(0, 3);
        table.add_column(components.get_info(a).unwrap());
        table.add_column(components.get_info(b).unwrap());
        table.add_column(components.get_info(t).unwrap());

        let mut table = table.build();

        let mut entity_allocator = EntityAllocator::default();
        let mut entities = Entities::new();
        let entity = entity_allocator.allocate();
        entities.alloc(entity.id(), entity.gnr());
        let tick = Tick::new(0);

        table.add_entity(entity);
        table.insert(a, A, tick);
        table.insert(b, B, tick);
        table.insert(t, T(20), tick);

        let row = unsafe { table.get_row(0).unwrap() };
        unsafe {
            for element_ref in row.into_iter() {
                match element_ref.component_id() {
                    x if x == a => {
                        assert_eq!(&A, element_ref.get::<A>())
                    }
                    x if x == b => {
                        assert_eq!(&B, element_ref.get::<B>())
                    }
                    x if x == t => {
                        assert_eq!(&T(20), element_ref.get::<T>())
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_select_row_get_multiple_mut_ref() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let a = component_reg.register_component::<A>();
        let b = component_reg.register_component::<B>();
        let t = component_reg.register_component::<T>();

        let mut table = TableBuilder::with_capacity(0, 3);
        table.add_column(components.get_info(a).unwrap());
        table.add_column(components.get_info(b).unwrap());
        table.add_column(components.get_info(t).unwrap());

        let mut table = table.build();

        let mut entity_allocator = EntityAllocator::default();
        let mut entities = Entities::new();
        let entity = entity_allocator.allocate();
        entities.alloc(entity.id(), entity.gnr());
        let tick = Tick::new(0);

        table.add_entity(entity);
        table.insert(a, A, tick);
        table.insert(b, B, tick);
        table.insert(t, T(20), tick);

        let row = unsafe { table.get_row_mut(0).unwrap() };
        unsafe {
            let a_mut = row.element_ref(a).get_mut::<A>();
            let t_mut = row.element_ref(t).get_mut::<T>();

            *a_mut = A;
            *t_mut = T(10);

            assert_eq!(a_mut, &A);
            assert_eq!(t_mut.0, 10);
        }
    }

    #[test]
    fn test_remove_push_replace_old_value() {
        let mut components = Components::default();
        let mut component_ids = ComponentIds::default();

        let mut component_reg =
            unsafe { ComponentsRegistrator::new(&mut components, &mut component_ids) };
        let t = component_reg.register_component::<T>();

        let mut table = TableBuilder::with_capacity(0, 1);
        table.add_column(components.get_info(t).unwrap());
        let mut table = table.build();

        let mut entity_allocator = EntityAllocator::default();
        let mut entities = Entities::new();
        let entity = entity_allocator.allocate();
        entities.alloc(entity.id(), entity.gnr());
        let tick = Tick::new(0);

        table.insert(t, T(1), tick);
        table.remove(t, 0, tick);
        table.insert(t, T(2), tick);
        let column = table.get_column(t).unwrap();
        unsafe {
            let blob_ref = column.get_as_blob_ref(0);
            let t_new = blob_ref.get_unchecked::<T>();
            assert_eq!(t_new.0, 2)
        }
    }
}
