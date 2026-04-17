use std::{cell::UnsafeCell, marker::PhantomData, ptr::NonNull};

use crate::ecs::{archetype::ArchetypeId, change_detection::{params::Mut, tick::Tick}, components::{Component, id::{ComponentId, ComponentIdsArray}}, entities::id::EntityId, observers::AnyNamedSystem, query::query_world::WorldQuery, storage::Table, worlds::World};

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

    fn init_state(world: &mut World) -> Self::State {
        ()
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        true
    }
    
    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    ) {
        ()
    }
    
    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        ()
    }
}

pub struct ReadFetch<'w, T : Component> {
    ptr : Option<*const T>,
    _marker : PhantomData<&'w T>
}

impl<T: Component> Clone for ReadFetch<'_, T> {
    fn clone(&self) -> Self {
        todo!()
    }
}



impl<T : Component> WorldQuery for &T {
    type Fetch<'w> = ReadFetch<'w, T>;
    type State = ComponentId;
    
    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        ReadFetch {
            ptr : None,
            _marker : PhantomData
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
        *fetch = ReadFetch { 
            ptr: table.get_column(*state).map(|s| s.as_ptr().cast::<T>()), 
            _marker: PhantomData 
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
        _entity : EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        Some(&*fetch.ptr?.add(table_row as usize))
    }
}

pub struct WriteFetch<'w, T : Component> {
    ptr : Option<*mut T>,
    _marker : PhantomData<&'w T>
}

impl<T: Component> Clone for WriteFetch<'_, T> {
    fn clone(&self) -> Self {
        todo!()
    }
}



impl<'__w, T : Component> WorldQuery for &'__w mut T {
    type Fetch<'w> = WriteFetch<'w, T>;
    type State = ComponentId;
    
    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w> {
        WriteFetch {
            ptr : None,
            _marker : PhantomData
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
        fetch.ptr = match table.get_column(*state) {
            Some(column) => Some(column.as_mut_ptr().cast::<T>()),
            None => None,
        }
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
        _entity : EntityId,
        table_row: u32,
    ) -> Option<Self::Item<'w, 's>> {
        Some(Mut { value: &mut *fetch.ptr?.add(table_row as usize) })
    }

}

impl<A : QueryData, B : QueryData> WorldQuery for (A, B) {
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
            B::init_fetch(world, &state.1, last_run, this_run)
        )
        
    }

    fn init_state(world: &mut World) -> Self::State {
        (
            A::init_state(world),
            B::init_state(world)
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
        }
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        A::matches_component_set(&state.0, set_contains_id) && B::matches_component_set(&state.1, set_contains_id)
    }
}

impl<A : QueryData, B : QueryData> QueryData for (A, B) {
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
            let b  = B::fetch(&state.1, &mut fetch.1, entity, table_row)?;
            Some((a, b))
        }
    }
}

// impl<T : Component> WorldQuery for Option<&T> {
//     type Fetch<'w> = Option<ReadFetch<'w, T>>;
//     type State = ComponentId;
    
//     fn init_fetch<'w, 's>(
//         world: &'w World,
//         state: &'s Self::State,
//         last_run: Tick,
//         this_run: Tick,
//     ) -> Self::Fetch<'w> {
//         None
//     }
    
//     fn init_state(world: &mut World) -> Self::State {
//         world.register_component::<T>()
//     }
    
//     unsafe fn set_table<'w, 's>(
//         fetch: &mut Self::Fetch<'w>,
//         state: &'s Self::State,
//         table: &'w Table,
//     ) {
//         if table.has_component(*state) {
//             *fetch = Some(ReadFetch { 
//                 data: unsafe { table.get_slice(*state) }, 
//                 _marker: PhantomData 
//             })
//         } else {
//             *fetch = None
//         }
        
//     }
    
//     fn matches_component_set(
//         &state: &Self::State,
//         set_contains_id: &impl Fn(ComponentId) -> bool,
//     ) -> bool {
//         true
//     }
// }

// impl<T: Component> QueryData for Option<&T> {
//     const IS_READ_ONLY: bool = true;
//     const IS_ARCHETYPAL: bool = true;

//     type Item<'w, 's> = Option<&'w T>;

//     fn shrink<'wlong: 'wshort, 'wshort, 's>(
//         item: Self::Item<'wlong, 's>,
//     ) -> Self::Item<'wshort, 's> {
//         item
//     }

//     unsafe fn fetch<'w, 's>(
//         _state: &'s Self::State,
//         fetch: &mut Self::Fetch<'w>,
//         _entity : EntityId,
//         table_row: u32,
//     ) -> Option<Self::Item<'w, 's>> {
//         match fetch {
//             Some(fetch) => {
//                 let d = fetch.data.get(table_row as usize)?;
//                 let v = unsafe { & *d.get() };
//                 Some(Some(v))
//             },
//             None => Some(None),
//         }
//     }
// }