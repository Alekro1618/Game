use std::{collections::{HashMap}};

use crate::{app::{App, main_schedule::ScheduleLabel}, ecs::{event::Event, observer::distributed_storage::IntoObserver, schedule::{config::IntoSchedule, schedule::Schedules, schedule_system::ScheduleSystem}, systems::observer_system::IntoObserverSystem, worlds::World}};



type ExtractFn = Box<dyn FnMut(&mut World, &mut World) + Send>;

pub struct SubApp {
    world: World,
    pub update_schedule: Option<ScheduleLabel>,
    extract: Option<ExtractFn>,
}

impl Default for SubApp {
    fn default() -> Self {
        let mut world = World::new();
        world.init_resource::<Schedules>();
        Self {
            world,
            update_schedule: None,
            extract: None,
        }
    }
}

impl SubApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn _run_as_app<F>(&mut self, f: F)
    where
        F: FnOnce(&mut App),
    {
        let mut app = App::empty();
        core::mem::swap(self, &mut app.sub_apps.main);
        f(&mut app);
        core::mem::swap(self, &mut app.sub_apps.main);
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn run_default_schedule(&mut self) {
        if let Some(label) = self.update_schedule {
            self.world.run_schedule(label);
        }
    }

    pub fn start_up(&mut self) {
        self.world.run_schedule(ScheduleLabel::Startup);
    }

    pub fn pre_update(&mut self) {
        self.world.run_schedule(ScheduleLabel::PreUpdate);
    }

    pub fn update(&mut self) {
        self.world.run_schedule(ScheduleLabel::Update);
    }

    pub fn post_update(&mut self) {
        self.world.run_schedule(ScheduleLabel::PostUpdate);
    }

    pub fn fixed_update(&mut self) {
        self.world.run_schedule(ScheduleLabel::FixedUpdate);
    }

    pub fn end(&mut self) {
        self.world.run_schedule(ScheduleLabel::Last);
    }

    pub fn extract(&mut self, world: &mut World) {
        if let Some(f) = self.extract.as_mut() {
            f(world, &mut self.world);
        }
    }

    pub fn set_extract<F>(&mut self, extract: F) -> &mut Self
    where
        F: FnMut(&mut World, &mut World) + Send + 'static,
    {
        self.extract = Some(Box::new(extract));
        self
    }

    pub fn take_extract(&mut self) -> Option<ExtractFn> {
        self.extract.take()
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: ScheduleLabel,
        systems: impl IntoSchedule<ScheduleSystem, M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();
        schedules.add_systems(schedule, systems);

        self
    }

    pub fn add_observer<E: Event, M>(&mut self, observer: impl IntoObserverSystem<E, M>) -> &mut Self {
        self.world.add_observer(observer);
        self
    }

    // pub fn add_message<M: Message>(&mut self) -> &mut Self {
    //     MessageRegistry::register_message::<T>(self.world_mut());

    //     self
    // }
}

#[derive(Default)]
pub struct SubApps {
    pub main: SubApp,
    pub sub_apps: HashMap<String, SubApp>,
}

impl SubApps {
    pub fn start_up(&mut self) {
        self.main.start_up();
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.start_up();
            sub_app.extract(&mut self.main.world);
        }
    }

    pub fn pre_update(&mut self) {
        self.main.pre_update();
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.pre_update();
            sub_app.extract(&mut self.main.world);
        }
    }

    pub fn update(&mut self) {
        self.main.update();
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.update();
            sub_app.extract(&mut self.main.world);
        }
    }

    pub fn post_update(&mut self) {
        self.main.post_update();
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.post_update();
            sub_app.extract(&mut self.main.world);
        }
    }


     pub fn fixed_update(&mut self) {
        self.main.fixed_update();
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.fixed_update();
            sub_app.extract(&mut self.main.world);
        }
    }



    pub fn end(&mut self) {
        self.main.end();
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.end();
            sub_app.extract(&mut self.main.world);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &SubApp> + '_ {
        core::iter::once(&self.main).chain(self.sub_apps.values())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SubApp> + '_ {
        core::iter::once(&mut self.main).chain(self.sub_apps.values_mut())
    }

    pub fn update_subapp_by_label(&mut self, label: String) {
        if let Some(sub_app) = self.sub_apps.get_mut(&label) {
            sub_app.extract(&mut self.main.world);
            sub_app.update();
        }
    }
}
