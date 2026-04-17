use crate::ecs::{change_detection::tick::Tick, query::{fetch::QueryData, filter::QueryFilter, iter::QueryIter, state::QueryState}, worlds::World};

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
}

impl<'w, 's, D: QueryData, F: QueryFilter> IntoIterator for Query<'w, 's, D, F> {
    type Item = D::Item<'w, 's>;

    type IntoIter = QueryIter<'w, 's, D, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_inner()
    }
}