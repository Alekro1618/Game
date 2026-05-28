use crate::{ecs::{change_detection::tick::Tick, entities::id::EntityId, query::{fetch::QueryData, filter::QueryFilter, iter::QueryIter, state::QueryState}, worlds::World}, error::error::{Error, ErrorKind}};

pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    world: &'world mut World,
    state: &'state QueryState<D, F>,
    last_run: Tick,
    this_run: Tick,
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

    pub fn get_mut<'t>(self, entity : EntityId) -> Result<D::Item<'w, 's>, Error> {
        let mut fetch =  D::init_fetch(self.world, &self.state.fetch_state, self.last_run, self.this_run);
        let mut filter = F::init_fetch(self.world, &self.state.filter_state, self.last_run, self.this_run);

        let Ok(location) = self.world.entities.get_spawned(entity) else {
            return Err(Error::new(ErrorKind::ECS, "Entity does not exist"));
        };
        let row = location.table_row;
        let table_id = location.table_id;

        let table = self.world.storages.get_table(table_id as usize);

        if table.is_none() {
            return Err(Error::new(ErrorKind::ECS, "Table containing entity doesn't exist"));
        }

        let table = table.unwrap();

        if table.is_empty() {
            return Err(Error::new(ErrorKind::ECS, "Table containing entity doesn't have components"));
        }

        unsafe {
            D::set_table(&mut fetch, &self.state.fetch_state, table);
            F::set_table(&mut filter, &self.state.filter_state, table);
        }

        if !F::filter_fetch(&self.state.filter_state, &mut filter, entity, row) {
            return Err(Error::new(ErrorKind::ECS, "No matching entities"));
        }

        let item =
            unsafe { D::fetch(&self.state.fetch_state, &mut fetch, entity, row) };

        if let Some(item) = item {
            return Ok(item);
        }

        return Err(Error::new(ErrorKind::ECS, format!("Entity {} doesn't contain bundle", entity)));
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> IntoIterator for Query<'w, 's, D, F> {
    type Item = D::Item<'w, 's>;

    type IntoIter = QueryIter<'w, 's, D, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_inner()
    }
}