use crate::{ecs::{type_register::TypeRegistry, worlds::World}, error::error::Error, scene::dynamic::DynamicScene};

pub struct Scene {
    pub world: World,
}

impl Scene {
    pub fn new(world : World) -> Self {
        Self {
            world
        }
    }
    
    pub fn from_dynamic_scene(
        dynamic_scene: &DynamicScene,
        type_registry: &TypeRegistry,
    ) -> Result<Scene, Error> {
        let mut world = World::new();
        
        dynamic_scene.write_to_world_with(&mut world, type_registry)?;

        Ok(Self { world })
    }
}
