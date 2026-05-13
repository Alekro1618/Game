use std::{any::TypeId, ptr::{self, dangling}};

use crate::ecs::{entities::id::EntityId, event::{Event}, systems::function_system::SystemParamTuple, worlds::World};

pub struct On<'w, E: Event> {
    event_type : TypeId,
    event: &'w mut E,
}

impl<'w, E: Event> On<'w, E> {
    /// Creates a new instance of [`On`] for the given triggered event.
    pub fn new(
        event: &'w mut E,
    ) -> Self {
        Self {
            event_type: TypeId::of::<E>(),
            event,
        }
    }

    /// Returns a reference to the triggered event.
    pub fn event(&self) -> &E {
        self.event
    }

    /// Returns a mutable reference to the triggered event.
    pub fn event_mut(&mut self) -> &mut E {
        self.event
    }
}

pub struct OnEventState<E : Event> {
    pub ptr : *mut E
}
unsafe impl<E: Event + 'static> Send for OnEventState<E> {

}
unsafe impl<E: Event + 'static> Sync for OnEventState<E> {
    
}

impl<E: Event + 'static> SystemParamTuple for On<'_, E> {
    type State = OnEventState<E>;

    type Item<'w> =  On<'w, E>;

    fn init_state(world: &mut World) -> Self::State {
        OnEventState {
            ptr : ptr::null_mut()
        }
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        unsafe  {
            On::new(&mut *state.ptr)
        }
    }
}