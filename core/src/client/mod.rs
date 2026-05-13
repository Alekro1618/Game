use engine::app::plugin::Plugin;

pub mod queue;
mod systems;
pub mod client;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app:  &mut engine::app::App) {

    }
}