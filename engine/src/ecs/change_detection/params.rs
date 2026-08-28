use std::ops::{Deref, DerefMut};

use crate::ecs::change_detection::tick::Tick;

pub struct Mut<'w, T: ?Sized> {
    pub(crate) value: &'w mut T,
    //pub(crate) ticks: ComponentTicksMut<'w>,
}

impl<'w, T: ?Sized> Mut<'w, T> {
    pub fn new(
        value: &'w mut T,
        _added: &'w mut Tick,
        _last_changed: &'w mut Tick,
        _last_run: Tick,
        _this_run: Tick,
    ) -> Self {
        Self { value }
    }

    // pub fn set_ticks(&mut self, last_run: Tick, this_run: Tick) {
    //     self.ticks.last_run = last_run;
    //     self.ticks.this_run = this_run;
    // }
}

impl<'w, T: ?Sized> Deref for Mut<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'w, T: ?Sized> DerefMut for Mut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
