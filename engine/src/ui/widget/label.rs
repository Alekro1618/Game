use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::ecs::components::{Component, Mutable, StorageType};

/// Marker struct for labels
#[derive(Debug, Default, Component, Reflect, Serialize, Deserialize, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Label;
