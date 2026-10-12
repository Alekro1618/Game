use engine_macros::{Component, Reflect, reflect};

use crate::{asset::handle::Handle, scene::dynamic::DynamicScene};


#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct DynamicSceneRoot(pub Handle<DynamicScene>);