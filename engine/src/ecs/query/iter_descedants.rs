use std::collections::VecDeque;

use crate::ecs::{
    entities::id::EntityId,
    hierarchy::Children,
    query::filter::QueryFilter,
    systems::query::Query,
};

pub struct DescendantIter<'q, 'w, 's, F: QueryFilter> {
    children_query: &'q Query<'w, 's, &'w Children, F>,
    vecdeque: VecDeque<EntityId>,
}

impl<'q, 'w, 's, F: QueryFilter> DescendantIter<'q, 'w, 's, F> {
    /// Returns a new [`DescendantIter`].
    pub fn new(children_query: &'q Query<'w, 's, &'w Children, F>, entity: EntityId) -> Self {
        DescendantIter {
            children_query,
            vecdeque: children_query
                .get(entity)
                .into_iter()
                .flat_map(|c| c.iter().map(|e| *e))
                .collect(),
        }
    }
}

impl<'q, 'w, 's, F: QueryFilter> Iterator for DescendantIter<'q, 'w, 's, F> {
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = self.vecdeque.pop_front()?;

        if let Ok(children) = self.children_query.get(entity) {
            self.vecdeque.extend(children.iter());
        }

        Some(entity)
    }
}
