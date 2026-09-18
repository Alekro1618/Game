use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use crate::{
    ecs::{
        change_detection::tick::Tick,
        entities::id::EntityId,
        hierarchy::Children,
        query::{
            fetch::QueryData, filter::QueryFilter, iter::QueryIter,
            iter_descedants::DescendantIter, state::QueryState,
        },
        worlds::World,
    },
    error::error::{Error, ErrorKind},
};

pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    world: &'world mut World,
    state: &'state QueryState<D, F>,
    last_run: Tick,
    this_run: Tick,
}

impl<'q, 'w, 's, F: QueryFilter> Query<'w, 's, &'w Children, F> {
    #[inline]
    pub fn iter_descendants(&'q self, entity: EntityId) -> DescendantIter<'q, 'w, 's, F> {
        DescendantIter::new(self, entity)
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> Query<'w, 's, D, F> {
    #[inline]
    pub(crate) fn new(
        world: &'w mut World,
        state: &'s QueryState<D, F>,
        last_run: Tick,
        this_run: Tick,
    ) -> Self {
        Self {
            world,
            state,
            last_run,
            this_run,
        }
    }

    #[inline]
    pub fn iter_inner(self) -> QueryIter<'w, 's, D, F> {
        QueryIter::new(self.world, self.state, self.last_run, self.this_run)
    }

    #[inline]
    pub fn iter<'d: 'w>(&'d self) -> QueryIter<'w, 's, D, F> {
        QueryIter::new(self.world, self.state, self.last_run, self.this_run)
    }

    pub fn get<'d: 'w>(&'d self, entity: EntityId) -> Result<D::Item<'w, 's>, Error> {
        let mut fetch = D::init_fetch(
            self.world,
            &self.state.fetch_state,
            self.last_run,
            self.this_run,
        );
        let mut filter = F::init_fetch(
            self.world,
            &self.state.filter_state,
            self.last_run,
            self.this_run,
        );

        let Ok(location) = self.world.entities.get_spawned(entity) else {
            return Err(Error::new(ErrorKind::ECS, "Entity does not exist"));
        };
        let row = location.table_row;
        let table_id = location.table_id;

        let table = self.world.storages.get_table(table_id as usize);

        if table.is_none() {
            return Err(Error::new(
                ErrorKind::ECS,
                "Table containing entity doesn't exist",
            ));
        }

        let table = table.unwrap();

        if table.is_empty() {
            return Err(Error::new(
                ErrorKind::ECS,
                "Table containing entity doesn't have components",
            ));
        }

        unsafe {
            D::set_table(&mut fetch, &self.state.fetch_state, table);
            F::set_table(&mut filter, &self.state.filter_state, table);
        }

        if !F::filter_fetch(&self.state.filter_state, &mut filter, entity, row) {
            return Err(Error::new(ErrorKind::ECS, "No matching entities"));
        }

        let item = unsafe { D::fetch(&self.state.fetch_state, &mut fetch, entity, row) };

        if let Some(item) = item {
            return Ok(item);
        }

        return Err(Error::new(
            ErrorKind::ECS,
            format!("Entity {} doesn't contain bundle", entity),
        ));
    }

    pub fn get_mut<'d: 'w>(&'d self, entity: EntityId) -> Result<D::Item<'w, 's>, Error> {
        let mut fetch = D::init_fetch(
            self.world,
            &self.state.fetch_state,
            self.last_run,
            self.this_run,
        );
        let mut filter = F::init_fetch(
            self.world,
            &self.state.filter_state,
            self.last_run,
            self.this_run,
        );

        let Ok(location) = self.world.entities.get_spawned(entity) else {
            return Err(Error::new(ErrorKind::ECS, "Entity does not exist"));
        };
        let row = location.table_row;
        let table_id = location.table_id;

        let table = self.world.storages.get_table(table_id as usize);

        if table.is_none() {
            return Err(Error::new(
                ErrorKind::ECS,
                "Table containing entity doesn't exist",
            ));
        }

        let table = table.unwrap();

        if table.is_empty() {
            return Err(Error::new(
                ErrorKind::ECS,
                "Table containing entity doesn't have components",
            ));
        }

        unsafe {
            D::set_table(&mut fetch, &self.state.fetch_state, table);
            F::set_table(&mut filter, &self.state.filter_state, table);
        }

        if !F::filter_fetch(&self.state.filter_state, &mut filter, entity, row) {
            return Err(Error::new(ErrorKind::ECS, "No matching entities"));
        }

        let item = unsafe { D::fetch(&self.state.fetch_state, &mut fetch, entity, row) };

        if let Some(item) = item {
            return Ok(item);
        }

        return Err(Error::new(
            ErrorKind::ECS,
            format!("Entity {} doesn't contain bundle", entity),
        ));
    }

    pub(crate) fn reborrow(&mut self) -> Query<'_, 's, D, F> {
        // SAFETY: this query is exclusively borrowed while the new one exists, so
        // no overlapping access can occur.
        unsafe { self.reborrow_unsafe() }
    }

    pub(crate) unsafe fn reborrow_unsafe(&self) -> Query<'_, 's, D, F> {
        // SAFETY:
        // - This is memory safe because the caller ensures that there are no conflicting references.
        // - The world matches because it was the same one used to construct self.
        unsafe { self.copy_unsafe() }
    }

    unsafe fn copy_unsafe(&self) -> Query<'w, 's, D, F> {
        // SAFETY:
        // - This is memory safe because the caller ensures that there are no conflicting references.
        // - The world matches because it was the same one used to construct self.
        let world: &'w mut World = unsafe { &mut *(self.world as *const World as *mut World) };
        Query::new(world, self.state, self.last_run, self.this_run)
    }

    #[inline]
    pub fn single_inner(self) -> Option<D::Item<'w, 's>> {
        let mut query = self.into_iter();
        query.next()
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> IntoIterator for Query<'w, 's, D, F> {
    type Item = D::Item<'w, 's>;

    type IntoIter = QueryIter<'w, 's, D, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_inner()
    }
}


pub struct Single<'w, 's, D: QueryData, F: QueryFilter = ()> {
    pub(crate) item: D::Item<'w, 's>,
    pub(crate) _filter: PhantomData<F>,
}

impl<'w, 's, D: QueryData, F: QueryFilter> Deref for Single<'w, 's, D, F> {
    type Target = D::Item<'w, 's>;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> DerefMut for Single<'w, 's, D, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> Single<'w, 's, D, F> {
    pub fn into_inner(self) -> D::Item<'w, 's> {
        self.item
    }
}