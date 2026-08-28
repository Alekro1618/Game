use std::marker::PhantomData;

use engine::app::{main_schedule::ScheduleLabel, plugin::Plugin};

use crate::minigames::minigame::MiniGame;

pub mod minigame;
mod cutter;
mod mixing;
mod systems;


pub struct GeneralMiniGamePlugin;

impl Plugin for GeneralMiniGamePlugin {
    fn build(self, _app:  &mut engine::app::App) {
        
    }
}

pub struct MiniGamePlugin<M : MiniGame>(PhantomData<M>);

impl<M : MiniGame> MiniGamePlugin<M> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<M : MiniGame> Plugin for MiniGamePlugin<M> {
    fn build(self, app:  &mut engine::app::App) {

    }
}

