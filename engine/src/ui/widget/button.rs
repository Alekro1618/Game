
use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::{ecs::components::{Component, Mutable, StorageType}};

/// Marker struct for buttons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect, Component, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Button;
