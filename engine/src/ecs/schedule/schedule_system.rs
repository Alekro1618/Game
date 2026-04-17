use crate::ecs::{schedule::config::{Schedulable, ScheduleConfig}, systems::system::System};

pub type ScheduleSystem = Box<dyn System<In = (), Out = ()>>;

impl Schedulable for ScheduleSystem {    
    fn into_schedulable(self) -> super::config::ScheduleConfig<Self>
    where
        Self: Sized {
        ScheduleConfig {
            node: self
        }
    }
}