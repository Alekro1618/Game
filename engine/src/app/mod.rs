use std::{collections::HashMap, num::NonZero, pin::Pin};

use crate::{
    animation::{PlayerPlugin, player::Player}, app::{
        main_schedule::ScheduleLabel,
        plugin::Plugin,
        sub_app::{SubApp, SubApps},
    }, asset::{
        asset::Asset,
        loader::AssetLoader,
    }, ecs::{
        event::Event, message::{Message, MessagePlugin, messages::Messages}, observer::distributed_storage::IntoObserver, resource::Resource, schedule::{config::IntoSchedule, schedule_system::ScheduleSystem}, systems::observer_system::IntoObserverSystem, type_register::{GetTypeRegistration, TypeRegistryPlugin}, worlds::FromWorld,
    }, exit::{ExitGame, ExitPlugin}, window::Window,
};

pub mod main_schedule;
pub mod plugin;
pub mod sub_app;

pub const FIXED_UPDATE: f32 = 2e-2;

type RunnerFn = Box<dyn FnMut(App) -> Pin<Box<dyn Future<Output = AppExit>>>>;

fn tick(app: &mut App, fixed_update: bool) -> Result<(), AppExit> {
    app.run_schedule(ScheduleLabel::First);

    if fixed_update {
        app.run_schedule(ScheduleLabel::FixedPreUpdate);
    }
    app.run_schedule(ScheduleLabel::PreUpdate);
    
    if fixed_update {
        app.run_schedule(ScheduleLabel::FixedUpdate);
       
    }
    app.run_schedule(ScheduleLabel::Update);

    app.run_schedule(ScheduleLabel::SpawnScene);

    if fixed_update {
        app.run_schedule(ScheduleLabel::FixedPostUpdate);
    }
    app.run_schedule(ScheduleLabel::PostUpdate);

    app.run_schedule(ScheduleLabel::AnimationSystems);

    app.run_schedule(ScheduleLabel::RenderSystems);

    if let Some(exit) = app.should_exit() {
        return Err(exit);
    };

    app.run_schedule(ScheduleLabel::Last);

    app.clear_trackers();

    

    Ok(())
}

async fn run_update(mut app: App) -> AppExit {
    let mut timer: f32 = 0.0;
    let mut fixed_update = true;

    app.run_schedule(ScheduleLabel::PreStartup);
    app.run_schedule(ScheduleLabel::Startup);
    app.run_schedule(ScheduleLabel::PostStartup);

    loop {
        //TODO:macroquad
        timer += macroquad::time::get_frame_time();
        if timer >= FIXED_UPDATE {
            fixed_update = true;
            timer -= FIXED_UPDATE;
        }

        if let Err(exit) = tick(&mut app, fixed_update) {
            return exit;
        }

        fixed_update = false;

        //TODO:macroquad
        macroquad::window::next_frame().await;
    }
}

fn wrap_run_update(app: App) -> Pin<Box<dyn Future<Output = AppExit>>> {
    Box::pin(run_update(app))
}

pub struct App {
    pub(crate) sub_apps: SubApps,
    pub(crate) runner: RunnerFn,
    //default_error_handler: Option<ErrorHandler>, TODO:Add
}

impl Default for App {
    fn default() -> Self {
        let mut app = App::empty();
        app.sub_apps.main.update_schedule = Some(ScheduleLabel::Main);

        app.add_plugin(TypeRegistryPlugin)
            .add_plugin(ExitPlugin);
        // app.add_message::<AppExit>();
        app
    }
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn empty() -> App {
        Self {
            sub_apps: SubApps {
                main: SubApp::new(),
                sub_apps: HashMap::default(),
            },
            runner: Box::new(wrap_run_update),
        }
    }

    pub fn run_schedule(&mut self, label : ScheduleLabel) {
        self.sub_apps.run_schedule(label);
    }

    pub fn clear_trackers(&mut self) {
        self.sub_apps.clear_trackers();
    }

    pub async fn run(&mut self) -> AppExit {
        let mut runner = core::mem::replace(&mut self.runner, Box::new(wrap_run_update));
        let app = core::mem::replace(self, App::empty());
        (runner)(app).await
    }

    pub fn should_exit(&self) -> Option<AppExit> {
        let messages = self.sub_apps.main.world().get_resource::<Messages<ExitGame>>()?;
        for message in messages.iter() {
            match message {
                ExitGame::Force => return Some(AppExit::Success),
                ExitGame::Error(_error) => todo!(),
            }
        }

        None
    }

    pub fn main_mut(&mut self) -> &mut SubApp {
        &mut self.sub_apps.main
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn init_resource<R : Resource + FromWorld>(&mut self) -> &mut Self {
        self.main_mut().init_resource::<R>();
        self
    }

    pub fn insert_resource<R : Resource>(&mut self, resource : R) -> &mut Self {
        self.main_mut().insert_resource(resource);
        self
    }

    pub fn add_message<M: Message>(&mut self) -> &mut Self {
        self.main_mut().add_message::<M>();
        self.add_plugin(MessagePlugin::<M>::default());
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: ScheduleLabel,
        systems: impl IntoSchedule<ScheduleSystem, M>,
    ) -> &mut Self {
        self.main_mut().add_systems(schedule, systems);
        self
    }

    pub fn add_observer<E: Event, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, M>,
    ) -> &mut Self {
        self.main_mut().add_observer(observer);
        self
    }

    pub fn insert_sub_app(&mut self, label: impl ToString, sub_app: SubApp) {
        self.sub_apps.sub_apps.insert(label.to_string(), sub_app);
    }

    pub fn set_window(&mut self, window: Window) -> &mut Self {
        self.main_mut().set_window(window);
        self
    }

    pub fn init_asset<A: Asset>(&mut self) -> &mut Self {
        self.main_mut().init_asset::<A>();
        self
    }

    pub fn init_asset_loader<L: AssetLoader + FromWorld>(&mut self) -> &mut Self {
        self.main_mut().init_asset_loader::<L>();
        self
    }

    pub fn register_player<P: Player>(&mut self) -> &mut Self {
        self.add_plugin(PlayerPlugin::<P>::new());

        self
    }

    pub fn register_type<T : GetTypeRegistration>(&mut self) -> &mut Self {
        self.main_mut().register_type::<T>();
        self
    }
    // pub fn add_message<M : Message>(&mut self) -> Self {
    //     self.main_mut().add_message::<M>();
    //     self
    // }
}

pub enum AppExit {
    Success,
    Error(NonZero<u8>),
}

impl AppExit {
    #[must_use]
    pub const fn error() -> Self {
        Self::Error(NonZero::<u8>::MIN)
    }

    /// Returns `true` if `self` is a [`AppExit::Success`].
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, AppExit::Success)
    }

    /// Returns `true` if `self` is a [`AppExit::Error`].
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, AppExit::Error(_))
    }

    /// Creates a [`AppExit`] from a code.
    ///
    /// When `code` is 0 a [`AppExit::Success`] is constructed otherwise a
    /// [`AppExit::Error`] is constructed.
    #[must_use]
    pub const fn from_code(code: u8) -> Self {
        match NonZero::<u8>::new(code) {
            Some(code) => Self::Error(code),
            None => Self::Success,
        }
    }
}
