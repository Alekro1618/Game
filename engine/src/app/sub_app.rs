use std::collections::HashMap;

use crate::{
    app::{App, main_schedule::ScheduleLabel}, asset::{
        AssetServer, asset::{Asset, Assets}, event::AssetEvent, loader::AssetLoader,
    }, ecs::{
        event::Event, message::Message, resource::Resource, schedule::{config::IntoSchedule, schedule::Schedules, schedule_system::ScheduleSystem}, systems::observer_system::IntoObserverSystem, type_register::{AppTypeRegistry, GetTypeRegistration}, worlds::{FromWorld, World},
    }, window::Window,
};

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

    pub fn run_schedule(&mut self, label : ScheduleLabel) {
        self.world.run_schedule(label);
    }

    pub fn pre_start_up(&mut self) {
        self.world.run_schedule(ScheduleLabel::PreStartup);
    }

    pub fn start_up(&mut self) {
        self.world.run_schedule(ScheduleLabel::Startup);
    }

    pub fn post_start_up(&mut self) {
        self.world.run_schedule(ScheduleLabel::PostStartup);
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

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, resource : R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn add_message<M: Message>(&mut self) -> &mut Self {
        self.world.add_message::<M>();
        self
    }

    pub fn init_asset<A: Asset>(&mut self) -> &mut Self {
        let assets = Assets::<A>::default();
        let asset_server = self.world.get_or_init_resource::<AssetServer>();
        asset_server.register_asset(&assets);

        self.add_message::<AssetEvent<A>>()
            .add_systems(
                ScheduleLabel::PostUpdate,
                Assets::<A>::asset_events
                    .run_if(Assets::<A>::asset_events_condition)
            );
            
        self.world.insert_resource(assets);

        self
    }

    pub fn init_asset_loader<L: AssetLoader + FromWorld>(&mut self) -> &mut Self {
        let asset_loader = L::from_world(&mut self.world);
        let asset_server = self.world.get_or_init_resource::<AssetServer>();
        asset_server.register_loader(asset_loader);

        self
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

    pub fn set_window(&mut self, window: Window) -> &mut Self {
        let window = window;
        self.world.insert_resource(window);
        self
    }

    pub fn add_observer<E: Event, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, M>,
    ) -> &mut Self {
        self.world.add_observer(observer);
        self
    }

    pub fn register_type<T : GetTypeRegistration>(&mut self) -> &mut Self {
        if let Some(type_register) = self.world.get_resource_mut::<AppTypeRegistry>() {
            type_register.0.write().unwrap().register::<T>();
        }
        
        self
    }
}

#[derive(Default)]
pub struct SubApps {
    pub main: SubApp,
    pub sub_apps: HashMap<String, SubApp>,
}

impl SubApps {
    pub fn run_schedule(&mut self, label : ScheduleLabel) {
        self.main.run_schedule(label);
        for (_label, sub_app) in self.sub_apps.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.run_schedule(label);
            self.main.extract(&mut sub_app.world);
        }
    }

    pub fn clear_trackers(&mut self) {
        self.main.world.clear_trackers();
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
