use engine::ecs::schedule::config::ScheduleConfig;
use engine_macros::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub enum GameState {
    Cutscene,
    Menu,
    #[default]
    Simulator
}