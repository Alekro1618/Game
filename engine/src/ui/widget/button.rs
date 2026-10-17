
use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};


/// Marker struct for buttons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect, Component, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Button;
