use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, function_system::{F2, F3}, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::resources::GameSpeed};

use crate::{players::player::{PendingAction, Player}, utils::grid::SmoothToGrid};

pub mod player;
mod systems;
pub mod spawner;
pub mod player_type;

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app:  &mut engine::app::App) {
        // app.add_systems::<F3<Commands, Query<(EntityId, &mut SmoothToGrid)>, &GameSpeed>>(
        //     ScheduleLabel::FixedUpdate, advance_smooth
        // )
        // .add_observer::<PendingAction, F2<On<PendingAction>, (Commands, Query<(EntityId, &mut Player)>)>>(
        //     |on, (commands, query)| {
        //     }
        // )
        // .add_systems::<F3<Commands, Query<(EntityId, &mut Player)>, &ButtonInput<KeyCode>>>(
        //     ScheduleLabel::FixedUpdate, add_path
        // );
    }
}

