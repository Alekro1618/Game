use std::{collections::HashMap, num::NonZero, pin::Pin};

use macroquad::{color::WHITE, window::{clear_background, next_frame}};

use crate::{app::{main_schedule::ScheduleLabel, plugin::Plugin, sub_app::{SubApp, SubApps}}, ecs::schedule::{config::IntoSchedule, schedule_system::ScheduleSystem}};

pub mod main_schedule;
pub mod plugin;
pub mod sub_app;


type RunnerFn = Box<dyn FnMut(App) -> Pin<Box<dyn Future<Output = AppExit>>>>;

fn _run_once(mut app: App) -> AppExit {
    app.update();

    app.should_exit().unwrap_or(AppExit::Success)
}

fn tick(app : &mut App) -> Result<(), AppExit> {
    app.update();

    if let Some(exit) = app.should_exit() {
        return Err(exit);
    };

    Ok(())
}

async fn run_update(mut app: App) -> AppExit {
    app.start_up();
    loop {
        clear_background(WHITE);
        if let Err(exit) =  tick(&mut app) {
            app.end();
            return exit;
        }
        next_frame().await;
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

    pub fn start_up(&mut self) {
        self.sub_apps.start_up();
    } 

    pub fn update(&mut self) {
        self.sub_apps.update();
    }

    pub fn end(&mut self) {
        self.sub_apps.end();
    } 


    pub async fn run(&mut self) -> AppExit {
        let mut runner = core::mem::replace(&mut self.runner, Box::new(wrap_run_update));
        let app = core::mem::replace(self, App::empty());
        (runner)(app).await
    }

    pub fn should_exit(&self) -> Option<AppExit> {
        // let mut reader = MessageCursor::default();

        // let messages = self.world().get_resource::<Messages<AppExit>>()?;
        // let mut messages = reader.read(messages);

        // if messages.len() != 0 {
        //     return Some(
        //         messages
        //             .find(|exit| exit.is_error())
        //             .cloned()
        //             .unwrap_or(AppExit::Success),
        //     );
        // }

        None
    }

    pub fn main_mut(&mut self) -> &mut SubApp {
        &mut self.sub_apps.main
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin : P) -> &mut Self {
        plugin.build(self);
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

    pub fn insert_sub_app(&mut self, label: impl ToString, sub_app: SubApp) {
        self.sub_apps.sub_apps.insert(label.to_string(), sub_app);
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
