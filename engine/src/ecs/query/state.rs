use crate::ecs::{
    query::{fetch::QueryData, filter::QueryFilter, iter::QueryIter},
    systems::query::Query,
    worlds::World,
};

#[allow(dead_code)]
pub struct QueryState<D: QueryData, F: QueryFilter = ()> {
    pub(crate) fetch_state: D::State,
    pub(crate) filter_state: F::State,
}

impl<D: QueryData, F: QueryFilter> QueryState<D, F> {
    pub unsafe fn new_unchecked(world: &mut World) -> Self {
        let fetch_state = D::init_state(world);
        let filter_state = F::init_state(world);

        Self {
            fetch_state,
            filter_state,
        }
    }

    pub fn new(world: &mut World) -> Self {
        let state = unsafe { Self::new_unchecked(world) };
        state
    }

    #[inline]
    pub fn iter<'w, 's>(&'s mut self, world: &'w mut World) -> QueryIter<'w, 's, D, F> {
        self.query(world).iter_inner()
    }

    #[inline]
    pub fn iter_mut<'w, 's>(&'s mut self, world: &'w mut World) -> QueryIter<'w, 's, D, F> {
        self.query_mut(world).iter_inner()
    }

    pub fn query<'w, 's>(&'s mut self, world: &'w mut World) -> Query<'w, 's, D, F> {
        let last_run = world.last_change_tick();
        let this_run = world.change_tick();

        Query::new(world, self, last_run, this_run)
    }

    pub fn query_mut<'w, 's>(&'s mut self, world: &'w mut World) -> Query<'w, 's, D, F> {
        let last_run = world.last_change_tick();
        let this_run = world.change_tick();

        Query::new(world, self, last_run, this_run)
    }
}
