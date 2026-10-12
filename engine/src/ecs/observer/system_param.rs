use std::{any::TypeId, marker::PhantomData};

use engine_macros::Event;

use crate::ecs::{
    event::Event, resource::Resource, systems::system::SystemInput,
};

pub struct On<'w, E: Event> {
    event_type: TypeId,
    event: &'w mut E,
}

impl<'w, E: Event> On<'w, E> {
    /// Creates a new instance of [`On`] for the given triggered event.
    pub fn new(event: &'w mut E) -> Self {
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


impl<'w, E: Event> SystemInput for On<'w, E> {}

#[derive(Event)]
pub struct ResourceChanged<R: Resource>(PhantomData<R>);