use engine_macros::Message;

use crate::{app::plugin::Plugin, error::error::Error};


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