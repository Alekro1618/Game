use crate::ecs::{schedule::schedule_system::ScheduleSystem, systems::IntoSystem};

pub trait IntoSchedule<T: Schedulable, Marker>: Sized {
    fn into_schedule(self) -> ScheduleConfig<T>;
}

pub trait Schedulable {
    fn into_schedulable(self) -> ScheduleConfig<Self>
    where
        Self: Sized;
}

pub struct ScheduleConfig<T: Schedulable> {
    pub(crate) node: T,
}

impl<F, Marker> IntoSchedule<ScheduleSystem, Marker> for F
where
    F: IntoSystem<(), (), Marker>,
{
    fn into_schedule(self) -> ScheduleConfig<ScheduleSystem> {
        ScheduleConfig {
            node: Box::new(IntoSystem::into_system(self)),
        }
    }
}
