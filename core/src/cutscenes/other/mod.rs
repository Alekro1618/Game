use engine::{app::plugin::Plugin, asset::handle::Handle, image::Image};
use engine_macros::Component;

use crate::cutscenes::other::system::on_background_transition;

mod system;
pub mod event;

#[derive(Component)]
pub struct Background;

pub struct OtherPlugin;

impl Plugin for OtherPlugin {
    fn build(self, app: &mut engine::app::App) {
        app
            .add_observer(on_background_transition);
    }
}