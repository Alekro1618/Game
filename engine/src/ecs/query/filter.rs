use std::marker::PhantomData;

use crate::ecs::{components::{Component, id::ComponentId}, entities::id::EntityId, query::query_world::WorldQuery};

pub trait QueryFilter: WorldQuery {
    const IS_ARCHETYPAL: bool;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool;
}

impl QueryFilter for () {
    const IS_ARCHETYPAL: bool = false;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool {
        true
    }
}

pub struct With<T>(PhantomData<T>);

impl<T: Component> WorldQuery for With<T> {
    type Fetch<'w> = ();

    type State = ComponentId;

    fn init_fetch<'w, 's>(
        world: &'w crate::ecs::worlds::World,
        state: &'s Self::State,
        last_run: crate::ecs::change_detection::tick::Tick,
        this_run: crate::ecs::change_detection::tick::Tick,
    ) -> Self::Fetch<'w> {
        
    }

    fn init_state(world: &mut crate::ecs::worlds::World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w crate::ecs::storage::Table,
    ) { }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(crate::ecs::components::id::ComponentId) -> bool,
    ) -> bool {
        set_contains_id(*state)
    }
}

impl<T: Component> QueryFilter for With<T> {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool {
        true
    }
}