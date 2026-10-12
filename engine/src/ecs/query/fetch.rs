use std::{cell::UnsafeCell, marker::PhantomData};

use crate::ecs::{
    change_detection::{params::{ComponentTicksMut, Mut}, tick::Tick}, components::{Component, id::ComponentId}, entities::id::EntityId, query::query_world::WorldQuery, storage::Table, worlds::World,
};

pub trait QueryData: WorldQuery {
    const IS_READ_ONLY: bool;
    const IS_ARCHETYPAL: bool;
    type Item<'w, 's>;

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's>;

    unsafe fn fetch<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>>;
}

impl WorldQuery for () {
    type Fetch<'w> = ();

    type State = ();

    fn init_state(_world: &mut World) -> Self::State {
        ()
    }

    fn matches_component_set(
        _state: &Self::State,
        _set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        true
    }

    unsafe fn set_table<'w, 's>(
        _fetch: &mut Self::Fetch<'w>,
        _state: &'s Self::State,
        _table: &'w Table,
    ) {
        ()
    }

    fn init_fetch<'w, 's>(
        _world: &'w World,
        _state: &'s Self::State,
        _last_run: Tick,
        _this_run: Tick,
    ) -> Self::Fetch<'w> {
        ()
    }
}

pub struct ReadFetch<'w, T> {
    ptr: Option<&'w [UnsafeCell<T>]>,
    _marker: PhantomData<&'w T>,
}

impl WorldQuery for EntityId {
    type Fetch<'w> = ();

    type State = ();

    fn init_fetch<'w, 's>(
        _world: &'w World,
        _state: &'s Self::State,
        _last_run: Tick,
        _this_run: Tick,
    ) -> Self::Fetch<'w> {
    }

    fn init_state(_world: &mut World) -> Self::State {}

    unsafe fn set_table<'w, 's>(
        _fetch: &mut Self::Fetch<'w>,
        _state: &'s Self::State,
        _table: &'w Table,
    ) {
    }

    fn matches_component_set(
        _state: &Self::State,
        _set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        true
    }
}

impl QueryData for EntityId {
    const IS_READ_ONLY: bool = false;

    const IS_ARCHETYPAL: bool = false;

    type Item<'w, 's> = EntityId;

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        item
    }

    unsafe fn fetch<'w, 's>(
        _state: &'s Self::State,
        _fetch: &mut Self::Fetch<'w>,
        entity: EntityId,
        _table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        Some(entity)
    }
}

impl<T: Component> WorldQuery for &T {
    type Fetch<'w> = ReadFetch<'w, T>;
    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w World,
        _state: &'s Self::State,
        _last_run: Tick,
        _this_run: Tick,
    ) -> Self::Fetch<'w> {
        ReadFetch {
            ptr: None,
            _marker: PhantomData,
        }
    }

    fn init_state(world: &mut World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        unsafe {
            fetch.ptr = table
                .get_column(*state)
                .map(|column| column.get_data_slice(table.entity_count() as usize))
        }
    }

    fn matches_component_set(
        &state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        set_contains_id(state)
    }
}

impl<T: Component> QueryData for &T {
    const IS_READ_ONLY: bool = true;
    const IS_ARCHETYPAL: bool = true;

    type Item<'w, 's> = &'w T;

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        item
    }

    unsafe fn fetch<'w, 's>(
        _state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        _entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        unsafe { Some(&*fetch.ptr?[table_row as usize].get()) }
    }
}

pub struct WriteFetch<'w, T: Component> {
    ptr: Option<&'w [UnsafeCell<T>]>,
    changed_ticks : Option<&'w [UnsafeCell<Tick>]>,
    added_ticks : Option<&'w [UnsafeCell<Tick>]>,
    last_run : Tick,
    this_run : Tick,
    _marker: PhantomData<&'w T>,
}

impl<'__w, T: Component> WorldQuery for &'__w mut T {
    type Fetch<'w> = WriteFetch<'w, T>;
    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w World,
        _state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        WriteFetch {
            ptr: None,
            _marker: PhantomData,
            last_run,
            this_run,
            changed_ticks: None,
            added_ticks: None,
        }
    }

    fn init_state(world: &mut World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        let column = table
                .get_column(*state);
        
        unsafe {
            fetch.ptr = column
                .map(|column| column.get_data_slice(table.entity_count()));
        }
        fetch.changed_ticks = column.map(|column| column.get_changed_ticks_slice(table.entity_count()));
        fetch.added_ticks = column.map(|column| column.get_added_ticks_slice(table.entity_count()));
    }

    fn matches_component_set(
        &state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        set_contains_id(state)
    }
}

impl<'__w, T: Component> QueryData for &'__w mut T {
    const IS_READ_ONLY: bool = false;
    const IS_ARCHETYPAL: bool = true;

    type Item<'w, 's> = Mut<'w, T>;

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        item
    }

    unsafe fn fetch<'w, 's>(
        _state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        _entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        unsafe {
            Some(Mut {
                value: &mut *fetch.ptr?[table_row as usize].get(),
                ticks: ComponentTicksMut {
                    added: &mut *fetch.added_ticks?[table_row as usize].get(),
                    changed: &mut *fetch.changed_ticks?[table_row as usize].get(),
                    last_run: fetch.last_run,
                    this_run: fetch.this_run,
                },
            })
        }
    }
}

impl<A: WorldQuery, B: WorldQuery> WorldQuery for (A, B) {
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>);
    type State = (A::State, B::State);

    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        (
            A::init_fetch(world, &state.0, last_run, this_run),
            B::init_fetch(world, &state.1, last_run, this_run),
        )
    }

    fn init_state(world: &mut World) -> Self::State {
        (A::init_state(world), B::init_state(world))
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        unsafe {
            A::set_table(&mut fetch.0, &state.0, table);
            B::set_table(&mut fetch.1, &state.1, table);
        }
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        A::matches_component_set(&state.0, set_contains_id)
            && B::matches_component_set(&state.1, set_contains_id)
    }
}

impl<A: QueryData, B: QueryData> QueryData for (A, B) {
    const IS_READ_ONLY: bool = A::IS_READ_ONLY && B::IS_READ_ONLY;

    const IS_ARCHETYPAL: bool = A::IS_ARCHETYPAL && B::IS_ARCHETYPAL;

    type Item<'w, 's> = (A::Item<'w, 's>, B::Item<'w, 's>);

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        todo!()
    }

    unsafe fn fetch<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        unsafe {
            let a = A::fetch(&state.0, &mut fetch.0, entity, table_row)?;
            let b = B::fetch(&state.1, &mut fetch.1, entity, table_row)?;
            Some((a, b))
        }
    }
}

impl<A: WorldQuery, B: WorldQuery, C: WorldQuery> WorldQuery for (A, B, C) {
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>, C::Fetch<'w>);
    type State = (A::State, B::State, C::State);

    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        (
            A::init_fetch(world, &state.0, last_run, this_run),
            B::init_fetch(world, &state.1, last_run, this_run),
            C::init_fetch(world, &state.2, last_run, this_run),
        )
    }

    fn init_state(world: &mut World) -> Self::State {
        (
            A::init_state(world),
            B::init_state(world),
            C::init_state(world),
        )
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        unsafe {
            A::set_table(&mut fetch.0, &state.0, table);
            B::set_table(&mut fetch.1, &state.1, table);
            C::set_table(&mut fetch.2, &state.2, table);
        }
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        A::matches_component_set(&state.0, set_contains_id)
            && B::matches_component_set(&state.1, set_contains_id)
            && C::matches_component_set(&state.2, set_contains_id)
    }
}

impl<A: QueryData, B: QueryData, C: QueryData> QueryData for (A, B, C) {
    const IS_READ_ONLY: bool = A::IS_READ_ONLY && B::IS_READ_ONLY && C::IS_READ_ONLY;

    const IS_ARCHETYPAL: bool = A::IS_ARCHETYPAL && B::IS_ARCHETYPAL && C::IS_ARCHETYPAL;

    type Item<'w, 's> = (A::Item<'w, 's>, B::Item<'w, 's>, C::Item<'w, 's>);

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        todo!()
    }

    unsafe fn fetch<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        unsafe {
            let a = A::fetch(&state.0, &mut fetch.0, entity, table_row)?;
            let b = B::fetch(&state.1, &mut fetch.1, entity, table_row)?;
            let c = C::fetch(&state.2, &mut fetch.2, entity, table_row)?;
            Some((a, b, c))
        }
    }
}

impl<A: WorldQuery, B: WorldQuery, C: WorldQuery, D: WorldQuery> WorldQuery for (A, B, C, D) {
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>, C::Fetch<'w>, D::Fetch<'w>);
    type State = (A::State, B::State, C::State, D::State);

    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        (
            A::init_fetch(world, &state.0, last_run, this_run),
            B::init_fetch(world, &state.1, last_run, this_run),
            C::init_fetch(world, &state.2, last_run, this_run),
            D::init_fetch(world, &state.3, last_run, this_run),
        )
    }

    fn init_state(world: &mut World) -> Self::State {
        (
            A::init_state(world),
            B::init_state(world),
            C::init_state(world),
            D::init_state(world),
        )
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        unsafe {
            A::set_table(&mut fetch.0, &state.0, table);
            B::set_table(&mut fetch.1, &state.1, table);
            C::set_table(&mut fetch.2, &state.2, table);
            D::set_table(&mut fetch.3, &state.3, table);
        }
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        A::matches_component_set(&state.0, set_contains_id)
            && B::matches_component_set(&state.1, set_contains_id)
            && C::matches_component_set(&state.2, set_contains_id)
            && D::matches_component_set(&state.3, set_contains_id)
    }
}

impl<A: QueryData, B: QueryData, C: QueryData, D: QueryData> QueryData for (A, B, C, D) {
    const IS_READ_ONLY: bool =
        A::IS_READ_ONLY && B::IS_READ_ONLY && C::IS_READ_ONLY && D::IS_READ_ONLY;

    const IS_ARCHETYPAL: bool =
        A::IS_ARCHETYPAL && B::IS_ARCHETYPAL && C::IS_ARCHETYPAL && D::IS_ARCHETYPAL;

    type Item<'w, 's> = (
        A::Item<'w, 's>,
        B::Item<'w, 's>,
        C::Item<'w, 's>,
        D::Item<'w, 's>,
    );

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        todo!()
    }

    unsafe fn fetch<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        unsafe {
            let a = A::fetch(&state.0, &mut fetch.0, entity, table_row)?;
            let b = B::fetch(&state.1, &mut fetch.1, entity, table_row)?;
            let c = C::fetch(&state.2, &mut fetch.2, entity, table_row)?;
            let d = D::fetch(&state.3, &mut fetch.3, entity, table_row)?;
            Some((a, b, c, d))
        }
    }
}

impl<T: Component> WorldQuery for Option<&T> {
    type Fetch<'w> = ReadFetch<'w, T>;
    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w World,
        _state: &'s Self::State,
        _last_run: Tick,
        _this_run: Tick,
    ) -> Self::Fetch<'w> {
        ReadFetch {
            ptr: None,
            _marker: PhantomData,
        }
    }

    fn init_state(world: &mut World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        unsafe {
            fetch.ptr = table
                .get_column(*state)
                .map(|column| column.get_data_slice(table.entity_count() as usize))
        }
    }

    fn matches_component_set(
        &_state: &Self::State,
        _set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        true
    }
}

impl<T: Component> QueryData for Option<&T> {
    const IS_READ_ONLY: bool = true;
    const IS_ARCHETYPAL: bool = true;

    type Item<'w, 's> = Option<&'w T>;

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        item
    }

    unsafe fn fetch<'w, 's>(
        _state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        _entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        unsafe { Some(fetch.ptr.map(|slice| &*slice[table_row as usize].get())) }
    }
}

impl<T: Component> WorldQuery for Option<&mut T > {
    type Fetch<'w> = WriteFetch<'w, T>;
    type State = ComponentId;

    fn init_fetch<'w, 's>(
        _world: &'w World,
        _state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        WriteFetch {
            ptr: None,
            _marker: PhantomData,
            last_run,
            this_run,
            changed_ticks: None,
            added_ticks: None,
        }
    }

    fn init_state(world: &mut World) -> Self::State {
        world.register_component::<T>()
    }

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        let column = table
            .get_column(*state);
        
        unsafe {
            fetch.ptr = column
                .map(|column| column.get_data_slice(table.entity_count()));
        }

        fetch.changed_ticks = column.map(|column| column.get_changed_ticks_slice(table.entity_count()));
        fetch.added_ticks = column.map(|column| column.get_added_ticks_slice(table.entity_count()));
    }

    fn matches_component_set(
        &_state: &Self::State,
        _set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        true
    }
}

impl<T: Component> QueryData for Option<&mut T> {
    const IS_READ_ONLY: bool = true;
    const IS_ARCHETYPAL: bool = true;

    type Item<'w, 's> = Option<Mut<'w, T>>;

    fn shrink<'wlong: 'wshort, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's> {
        item
    }

    unsafe fn fetch<'w, 's>(
        _state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        _entity: EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        let added_ticks = fetch.added_ticks?;
        let changed_ticks = fetch.changed_ticks?;
        
        unsafe { Some(
            fetch.ptr.map(|p| 
                Mut {
                    value: &mut *p[table_row as usize].get(),
                    ticks: ComponentTicksMut {
                        added: &mut *added_ticks[table_row as usize].get(),
                        changed: &mut *changed_ticks[table_row as usize].get(),
                        last_run: fetch.last_run,
                        this_run: fetch.this_run,
                    },
                }
            )
        )
        }
    }
}