use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin},
    ecs::systems::{commands::Commands},
};

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.add_systems(ScheduleLabel::FixedUpdate, debug);
    }
}

pub fn debug(mut commands: Commands) {
    commands.log_archetype();
}
