use std::any::TypeId;

use crate::ecs::{change_detection::tick::Tick, components::{id::{ComponentId, ComponentIdsArray}, info::Components, register::{ComponentIds, ComponentsRegistrator}}, entities::id::EntityId, storage::Table};
pub mod info;

pub trait Bundle: Send + Sync + 'static {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<Self>;

    fn insert_into_table(self, components : &ComponentIdsArray, table: &mut Table, tick : Tick);
}

