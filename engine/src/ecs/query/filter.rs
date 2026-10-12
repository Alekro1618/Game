use std::{cell::UnsafeCell, marker::PhantomData};

use crate::ecs::{
    change_detection::tick::Tick, components::{Component, id::ComponentId}, entities::id::EntityId, query::query_world::WorldQuery,
};

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
        _state: &Self::State,
        _fetch: &mut Self::Fetch<'_>,
        _entity: EntityId,
        _table_row: u32,
    ) -> bool {
        true
    }
}

pub struct With<T>(PhantomData<T>);

impl<T: Component> WorldQuery for With<T> {
    type Fetch<'w> = ();

    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w crate::ecs::worlds::World,
        _state: &'s Self::State,
        _last_run: crate::ecs::change_detection::tick::Tick,
        _this_run: crate::ecs::change_detection::tick::Tick,
    ) -> Self::Fetch<'w> {
    }

    fn init_state(world: &mut crate::ecs::worlds::World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        _fetch: &mut Self::Fetch<'w>,
        _state: &'s Self::State,
        _table: &'w crate::ecs::storage::Table,
    ) {
    }

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
        _state: &Self::State,
        _fetch: &mut Self::Fetch<'_>,
        _entity: EntityId,
        _table_row: u32,
    ) -> bool {
        true
    }
}

pub struct Without<T>(PhantomData<T>);

impl<T: Component> WorldQuery for Without<T> {
    type Fetch<'w> = ();

    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w crate::ecs::worlds::World,
        _state: &'s Self::State,
        _last_run: crate::ecs::change_detection::tick::Tick,
        _this_run: crate::ecs::change_detection::tick::Tick,
    ) -> Self::Fetch<'w> {
    }

    fn init_state(world: &mut crate::ecs::worlds::World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        _fetch: &mut Self::Fetch<'w>,
        _state: &'s Self::State,
        _table: &'w crate::ecs::storage::Table,
    ) {
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(crate::ecs::components::id::ComponentId) -> bool,
    ) -> bool {
        !set_contains_id(*state)
    }
}

impl<T: Component> QueryFilter for Without<T> {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        _state: &Self::State,
        _fetch: &mut Self::Fetch<'_>,
        _entity: EntityId,
        _table_row: u32,
    ) -> bool {
        true
    }
}

pub struct Or<A: QueryFilter, B: QueryFilter>(pub A, pub B);

impl<A: QueryFilter, B: QueryFilter> WorldQuery for Or<A, B> {
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>);

    type State = (A::State, B::State);

    fn init_fetch<'w, 's>(
        world: &'w crate::ecs::worlds::World,
        state: &'s Self::State,
        last_run: crate::ecs::change_detection::tick::Tick,
        this_run: crate::ecs::change_detection::tick::Tick,
    ) -> Self::Fetch<'w> {
        (
            A::init_fetch(world, &state.0, last_run, this_run),
            B::init_fetch(world, &state.1, last_run, this_run),
        )
    }

    fn init_state(world: &mut crate::ecs::worlds::World) -> Self::State {
        (A::init_state(world), B::init_state(world))
    }

    unsafe fn set_table<'w, 's>(
        _fetch: &mut Self::Fetch<'w>,
        _state: &'s Self::State,
        _table: &'w crate::ecs::storage::Table,
    ) {
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(crate::ecs::components::id::ComponentId) -> bool,
    ) -> bool {
        A::matches_component_set(&state.0, set_contains_id)
            || B::matches_component_set(&state.1, set_contains_id)
    }
}

impl<A: QueryFilter, B: QueryFilter> QueryFilter for Or<A, B> {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool {
        A::filter_fetch(&state.0, &mut fetch.0, entity, table_row)
            || B::filter_fetch(&state.1, &mut fetch.1, entity, table_row)
    }
}

pub struct And<A: QueryFilter, B: QueryFilter>(pub A, pub B);

impl<A: QueryFilter, B: QueryFilter> WorldQuery for And<A, B> {
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>);

    type State = (A::State, B::State);

    fn init_fetch<'w, 's>(
        world: &'w crate::ecs::worlds::World,
        state: &'s Self::State,
        last_run: crate::ecs::change_detection::tick::Tick,
        this_run: crate::ecs::change_detection::tick::Tick,
    ) -> Self::Fetch<'w> {
        (
            A::init_fetch(world, &state.0, last_run, this_run),
            B::init_fetch(world, &state.1, last_run, this_run),
        )
    }

    fn init_state(world: &mut crate::ecs::worlds::World) -> Self::State {
        (A::init_state(world), B::init_state(world))
    }

    unsafe fn set_table<'w, 's>(
        _fetch: &mut Self::Fetch<'w>,
        _state: &'s Self::State,
        _table: &'w crate::ecs::storage::Table,
    ) {
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(crate::ecs::components::id::ComponentId) -> bool,
    ) -> bool {
        A::matches_component_set(&state.0, set_contains_id)
            && B::matches_component_set(&state.1, set_contains_id)
    }
}

impl<A: QueryFilter, B: QueryFilter> QueryFilter for And<A, B> {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool {
        A::filter_fetch(&state.0, &mut fetch.0, entity, table_row)
            && B::filter_fetch(&state.1, &mut fetch.1, entity, table_row)
    }
}

impl<A: QueryFilter, B: QueryFilter> QueryFilter for (A, B) {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool {
        A::filter_fetch(&state.0, &mut fetch.0, entity, table_row)
            && B::filter_fetch(&state.1, &mut fetch.1, entity, table_row)
    }
}

impl<A: QueryFilter, B: QueryFilter, C: QueryFilter> QueryFilter for (A, B, C) {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: EntityId,
        table_row: u32,
    ) -> bool {
        A::filter_fetch(&state.0, &mut fetch.0, entity, table_row) && 
        B::filter_fetch(&state.1, &mut fetch.1, entity, table_row) && 
        C::filter_fetch(&state.2, &mut fetch.2, entity, table_row)
    }
}

pub struct Changed<T>(PhantomData<T>);

pub struct ChangedFetch<'w, T> {
    ptr: Option<&'w [UnsafeCell<Tick>]>,
    _marker: PhantomData<&'w T>,
    last_run : Tick,
    this_run : Tick
}


impl<T: Component> WorldQuery for Changed<T> {
    type Fetch<'w> = ChangedFetch<'w, T>;

    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w crate::ecs::worlds::World,
        _state: &'s Self::State,
        last_run: crate::ecs::change_detection::tick::Tick,
        this_run: crate::ecs::change_detection::tick::Tick,
    ) -> Self::Fetch<'w> {
        ChangedFetch {
            ptr: None,
            _marker: PhantomData,
            last_run,
            this_run
        }
    }

    fn init_state(world: &mut crate::ecs::worlds::World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w crate::ecs::storage::Table,
    ) {
        fetch.ptr = table
            .get_column(*state)
            .map(|column| column.get_changed_ticks_slice(table.entity_count() as usize))
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(crate::ecs::components::id::ComponentId) -> bool,
    ) -> bool {
        set_contains_id(*state)
    }
}


impl<T: Component> QueryFilter for Changed<T> {
    const IS_ARCHETYPAL: bool = true;

    fn filter_fetch(
        _state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        _entity: EntityId,
        table_row: u32,
    ) -> bool {
        fetch.ptr.map(|slice| {
            // SAFETY: The caller ensures `table_row` is in range.
            let tick = unsafe { slice.get_unchecked(table_row as usize) };

            (unsafe { *tick.get() }).clone().is_newer_than(fetch.last_run, fetch.this_run)
        }).unwrap()
    }
}