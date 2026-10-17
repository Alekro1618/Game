use engine_macros::{Reflect, Resource, reflect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Resource, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub enum GameState {
    Cutscene,
    #[default]
    Menu,
    Simulator
}