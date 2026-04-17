use std::f32::consts::PI;

use crate::ecs::{archetype::{self, Archetype, Archetypes}, change_detection::tick::Tick, components::StorageType, entities::{Entity, id::EntityId}, query::{fetch::{self, QueryData}, filter::QueryFilter, state::QueryState}, storage::{Storages, Table}, worlds::World};

pub struct QueryIter<'w, 's, D: QueryData, F: QueryFilter> {
    world: &'w World,
    tables: &'w Vec<Table>,
    archetypes: &'w Archetypes,
    query_state: &'s QueryState<D, F>,
    cursor : QueryCursor<'w, D, F>
}

impl<'w, 's, D: QueryData, F: QueryFilter> QueryIter<'w, 's, D, F> {
    pub(crate) fn new(
        world: &'w World,
        query_state: &'s QueryState<D, F>,
        last_run: Tick,
        this_run: Tick,
    ) -> Self {
        let fetch =  D::init_fetch(world, &query_state.fetch_state, last_run, this_run);
        let filter = F::init_fetch(world, &query_state.filter_state, last_run, this_run);
        let tables = &world.storages().tables;
        let archetypes = world.archetypes();
        QueryIter {
            world,
            query_state,
            tables,
            archetypes,
            cursor: QueryCursor::init(fetch, filter)
        }
    }
}

struct QueryCursor<'w, D: QueryData, F: QueryFilter> {
    fetch: D::Fetch<'w>,
    filter: F::Fetch<'w>,
    entities: &'w [EntityId],
    current_table_id: u32,
    current_len: u32,
    current_row: u32,
}

impl<'w, 's, D: QueryData, F: QueryFilter> QueryCursor<'w, D, F> {
    fn init(
        fetch: D::Fetch<'w>,
        filter: F::Fetch<'w>
    ) -> Self {
        Self {
            fetch,
            filter,
            entities: &[],
            current_len: 0,
            current_row: 0,
            current_table_id: 0
        }
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> Iterator for QueryIter<'w, 's, D, F> {
    type Item  = D::Item<'w, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cursor = &mut self.cursor;
            if cursor.current_row == cursor.current_len {
                let table = self.tables.get(cursor.current_table_id as usize);
                cursor.current_table_id += 1;

                if table.is_none() {
                    return None;
                }

                let table = table.unwrap();

                if table.is_empty() {
                    continue;
                }
                unsafe {
                    D::set_table(&mut cursor.fetch, &self.query_state.fetch_state, table);
                    F::set_table(&mut cursor.filter, &self.query_state.filter_state, table);
                }
                cursor.entities = table.entities();
                cursor.current_len = table.len() as u32;
                cursor.current_row = 0;
            } 

            let entity = 
                unsafe { cursor.entities.get_unchecked(cursor.current_row as usize) };
            let row = cursor.current_row.clone();
            cursor.current_row += 1;
            if !F::filter_fetch(&self.query_state.filter_state, &mut cursor.filter, *entity, row) {
                continue;
            }

            let item =
                unsafe { D::fetch(&self.query_state.fetch_state, &mut cursor.fetch, *entity, row) };
            if let Some(item) = item {
                return Some(item);
            }
        }
    }
}