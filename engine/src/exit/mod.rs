use engine_macros::{Event, Message, Resource};

use crate::{app::plugin::Plugin, ecs::observer::system_param::On, error::error::Error};


#[derive(Message)]
pub enum ExitGame {
    Force,
    Error(Error)
}

pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.add_message::<ExitGame>();
    }
}