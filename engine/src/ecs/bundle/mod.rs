use std::any::TypeId;

use crate::ecs::{
    change_detection::tick::Tick, components::{
        id::{ComponentId, ComponentIdsArray},
        register::ComponentsRegistrator,
    }, query::iter, storage::Table,
};
pub mod info;

pub trait Bundle: Send + Sync + 'static {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<Self>;

    fn insert_into_table(self, components: &ComponentIdsArray, table: &mut Table, tick: Tick);

    fn replace_in_table(
        self,
        components: &ComponentIdsArray,
        table: &mut Table,
        row: usize,
        tick: Tick,
    );

    fn remove_from_table(components: &ComponentIdsArray, table: &mut Table, row: usize, tick: Tick);
}

impl Bundle for () {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<> {
        vec![].into_iter()
    }

    fn insert_into_table(self, components: &ComponentIdsArray, table: &mut Table, tick: Tick) {

    }

    fn replace_in_table(
        self,
        components: &ComponentIdsArray,
        table: &mut Table,
        row: usize,
        tick: Tick,
    ) {

    }

    fn remove_from_table(components: &ComponentIdsArray, table: &mut Table, row: usize, tick: Tick) {

    }
}