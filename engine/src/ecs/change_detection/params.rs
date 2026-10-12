use std::ops::{Deref, DerefMut};

use crate::ecs::change_detection::tick::Tick;

pub struct Mut<'w, T: ?Sized> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

impl<'w, T: ?Sized> Mut<'w, T> {
    #[inline]
    pub fn new(
        value: &'w mut T,
        added: &'w mut Tick,
        changed: &'w mut Tick,
        last_run: Tick,
        this_run: Tick,
    ) -> Self {
        Self { value, ticks: ComponentTicksMut { added, changed, last_run, this_run } }
    }

    #[inline]
    pub fn is_changed(&self) -> bool {
        self.is_changed_after(self.ticks.last_run)
    }

     #[inline]
    fn is_changed_after(&self, other: Tick) -> bool {
        self.ticks.changed.is_newer_than(other, self.ticks.this_run)
    }
}

impl<'w, T: ?Sized> Deref for Mut<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'w, T: ?Sized> DerefMut for Mut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        *self.ticks.changed = self.ticks.this_run;
        self.value
    }
}


pub(crate) struct ComponentTicksMut<'w> {
    pub(crate) added: &'w mut Tick,
    pub(crate) changed: &'w mut Tick,
    pub(crate) last_run: Tick,
    pub(crate) this_run: Tick,
}

