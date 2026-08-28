use crate::ecs::{schedule::schedule_system::ScheduleSystem, worlds::World};

pub(crate) struct SystemNode {
    pub(crate) system: ScheduleSystem,
}

impl SystemNode {
    pub fn new(system: ScheduleSystem) -> Self {
        Self { system }
    }

    pub fn apply(&mut self, world: &mut World) {
        self.system.apply(world);
    }
}
