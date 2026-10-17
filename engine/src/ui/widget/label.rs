use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};


/// Marker struct for labels
#[derive(Debug, Default, Component, Reflect, Serialize, Deserialize, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Label;
