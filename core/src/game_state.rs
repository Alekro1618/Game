use engine::ecs::schedule::config::ScheduleConfig;
use engine_macros::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub enum GameState {
    Cutscene,
    #[default]
    Menu,
    Simulator
}

pub enum CurrentStage {
    Cutscene(Box<str>), 
    Level(Box<str>)
}