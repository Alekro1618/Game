use crate::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::type_register::TypeRegistryPlugin, scene::{dynamic::DynamicScene, loader::SceneLoader, scene_spawner::{SceneSpawner, scene_spawner, scene_spawner_system}}};

mod deserialize;
pub mod dynamic;
mod registration;
pub mod scene;
mod serde;
mod serialize;
mod loader;
pub mod components;
mod scene_spawner;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .init_asset::<DynamicScene>()
            .init_asset_loader::<SceneLoader>()
            .insert_resource(SceneSpawner::default())
            .add_systems(
                ScheduleLabel::SpawnScene, 
                (scene_spawner, scene_spawner_system)
            );
    }
}