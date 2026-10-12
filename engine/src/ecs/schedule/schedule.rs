use std::collections::HashMap;

use crate::{
    app::main_schedule::ScheduleLabel, ecs::{
        components::{Component, Mutable, StorageType}, resource::Resource, schedule::{self, config::{IntoSchedule, ScheduleConfig}, node::{SystemGraph, Systems}, schedule_system::ScheduleSystem}, worlds::World,
    },
};

pub struct Schedules {
    inner: HashMap<ScheduleLabel, Schedule>,
}

impl Default for Schedules {
    fn default() -> Self {
        Self {
            inner: ScheduleLabel::ALL
                .iter()
                .map(|label| (label.clone(), Schedule::new(*label)))
                .collect(),
        }
    }
}

impl Component for Schedules {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}
impl Resource for Schedules {}

impl Schedules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, schedule: Schedule) -> Option<Schedule> {
        self.inner.insert(schedule.label, schedule)
    }

    pub fn remove(&mut self, label: ScheduleLabel) -> Option<Schedule> {
        self.inner.remove(&label)
    }

    pub fn remove_temporarily(&mut self, label: ScheduleLabel) -> Option<Schedule> {
        let k = self.inner.remove(&label);

        k
    }

    pub fn remove_entry(&mut self, label: ScheduleLabel) -> Option<(ScheduleLabel, Schedule)> {
        self.inner.remove_entry(&label)
    }

    pub fn contains(&self, label: ScheduleLabel) -> bool {
        self.inner.contains_key(&label)
    }

    pub fn get(&self, label: ScheduleLabel) -> Option<&Schedule> {
        self.inner.get(&label)
    }

    pub fn get_mut(&mut self, label: ScheduleLabel) -> Option<&mut Schedule> {
        self.inner.get_mut(&label)
    }

    pub fn entry(&mut self, label: ScheduleLabel) -> &mut Schedule {
        self.inner
            .entry(label)
            .or_insert_with(|| Schedule::new(label))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ScheduleLabel, &Schedule)> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ScheduleLabel, &mut Schedule)> {
        self.inner.iter_mut()
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: ScheduleLabel,
        systems: impl IntoSchedule<ScheduleSystem, M>,
    ) -> &mut Self {
        self.entry(schedule).add_systems(systems);

        self
    }
}

pub struct Schedule {
    label: ScheduleLabel,
    systems: Systems,
    executor_initialized: bool,
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new(ScheduleLabel::default())
    }
}

impl Schedule {
    pub fn new(label: ScheduleLabel) -> Self {
        Self {
            label: label,
            systems: Systems::default(),
            executor_initialized: false,
        }
    }

    pub fn label(&self) -> ScheduleLabel {
        self.label
    }

    pub fn add_systems<M>(&mut self, systems: impl IntoSchedule<ScheduleSystem, M>) -> &mut Self {
        self.executor_initialized = false;
        let schedule = systems.into_schedule();
        
        self.systems.insert(SystemGraph::from_schedule(schedule));
        
        self
    }

    pub fn run(&mut self, world: &mut World) {
        if !self.executor_initialized {
            self.initialize(world);
        }
  
        for graph in self.systems.iter_mut() {
            graph.run(world);
        }
    }

    fn initialize(&mut self, world: &mut World) {
        self.sort_systems();
        for graph in self.systems.iter_mut() {
            graph.init(world);
        }
        self.executor_initialized = true;
    }

    fn sort_systems(&mut self) {
        //TODO:SORT
    }
}
