use std::u32;


#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Tick {
    tick: u32,
}

impl Tick {
    pub const MAX: Self = Self::new(u32::MAX);

    #[inline]
    pub const fn new(tick: u32) -> Self {
        Self { tick }
    }

    #[inline]
    pub const fn get(self) -> u32 {
        self.tick
    }

    #[inline]
    pub fn set(&mut self, tick: u32) {
        self.tick = tick;
    }

    #[inline]
    pub fn is_newer_than(self, last_run: Tick, this_run: Tick) -> bool {
        let ticks_since_insert = this_run.relative_to(self).tick.min(u32::MAX);
        let ticks_since_system = this_run.relative_to(last_run).tick.min(u32::MAX);

        ticks_since_system > ticks_since_insert
    }

    #[inline]
    pub(crate) fn relative_to(self, other: Self) -> Self {
        let tick = self.tick.wrapping_sub(other.tick);
        Self { tick }
    }

}
