
use std::collections::HashMap;

use engine_macros::Asset;
use serde::Serialize;

use crate::{
    asset::asset::Asset, ecs::{
        entities::id::EntityId, reflect::Reflect, type_register::{AppTypeRegistry, ReflectComponent, ReflectResource, TypeRegistry}, worlds::World,
    }, error::error::{Error, ErrorKind}, scene::{scene::Scene, serde::SceneSerializer},
};

pub struct DynamicEntity {
    pub entity: EntityId,
    pub components: Vec<Box<dyn Reflect>>,
}

#[derive(Asset)]
pub struct DynamicScene {
    pub entities: Vec<DynamicEntity>,
    pub resources: Vec<Box<dyn Reflect>>,
}

impl DynamicScene {
    pub fn from_scene(scene: &Scene, type_registry: &TypeRegistry) -> Self {
        Self::from_world_with(&scene.world, type_registry)
    }

    pub fn from_world(world: &World, type_registry: &TypeRegistry) -> Self {
        Self::from_world_with(world, type_registry)
    }

    pub fn from_world_with(world: &World, type_registry: &TypeRegistry) -> Self {
        DynamicSceneBuilder::from_world(world, type_registry)
            .extract_entities(
                // we do this instead of a query, in order to completely sidestep default query filters.
                // while we could use `Allow<_>`, this wouldn't account for custom disabled components
                world.archetypes().iter().flat_map(|arch| arch.entites()),
            )
            .extract_resources()
            .build()
    }

    pub fn serialize(&self, registry: &TypeRegistry) -> Result<String, ron::Error> {
        serialize_ron(SceneSerializer::new(self, registry))
    }

    pub fn write_to_world(
        &self,
        world: &mut World,
    ) -> Result<HashMap<EntityId, EntityId>, Error> {
        let registry = world.resource::<AppTypeRegistry>().clone();
        self.write_to_world_with(world, &registry.0.read().unwrap())
    }

    pub fn write_to_world_with(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
    ) -> Result<HashMap<EntityId, EntityId>, Error> {
        let entities : Vec<(&DynamicEntity, EntityId)>= self
            .entities
            .iter()
            .map(|entity| (entity, world.spawn_empty()))
            .collect();

        for (scene_entity, entity_id) in entities.iter() {
            // Apply/ add each component to the given entity.
            for component in &scene_entity.components {
                let type_id = component.get_type_id().ok_or_else(|| {
                    Error::new(ErrorKind::ECS, format!("No represented type {}", component.name()))
                })?;
                let registration = type_registry.get(type_id).ok_or_else(|| {
                    Error::new(ErrorKind::ECS, format!("Unregistered but reflected type {}", component.name()))
                })?;
                let reflect_component =
                    registration.data::<ReflectComponent>().ok_or_else(|| {
                        Error::new(ErrorKind::ECS, format!("Unregistered component {}", component.name()))
                    })?;

                let component_id = reflect_component.register_component(world);

                reflect_component.insert(
                    world, 
                    *entity_id, 
                    component.as_reflect(), 
                    type_registry
                );
            }
        }

        // Insert resources after all entities have been added to the world.
        // This ensures the entities are available for the resources to reference during mapping.
        for resource in &self.resources {
            let type_id = resource.get_type_id().ok_or_else(|| {
                Error::new(ErrorKind::ECS, format!("No represented type {}", resource.name()))
            })?;
            let registration = type_registry.get(type_id).ok_or_else(|| {
                Error::new(ErrorKind::ECS, format!("Unregistered but reflected type {}", resource.name()))
            })?;
            let reflect_resource = registration.data::<ReflectResource>().ok_or_else(|| {
                Error::new(ErrorKind::ECS, format!("Unregistered component {}", resource.name()))
            })?;

            let resource_id = reflect_resource.register_component(world);

            reflect_resource.insert(
                world,
                resource.as_reflect(),
                type_registry
            );
        }

        Ok(entities
            .into_iter()
            .map(|(d, e)| (d.entity, e))
            .collect()
        )
    }

}


pub struct DynamicSceneBuilder<'w> {
    world: &'w World,
    extracted_resources: Vec<Box<dyn Reflect>>,
    extracted_entities: Vec<DynamicEntity>,
    type_registry: &'w TypeRegistry,
}

impl<'w> DynamicSceneBuilder<'w> {
    pub fn from_world(world: &'w World, type_registry: &'w TypeRegistry) -> Self {
        Self {
            world,
            extracted_resources: Vec::new(),
            extracted_entities: Vec::new(),
            type_registry,
        }
    }

    pub fn extract_entities(mut self, entities: impl Iterator<Item = EntityId>) -> Self {
        for entity in entities {
            let mut entry = DynamicEntity {
                entity,
                components: Vec::new(),
            };
            println!("id: {}", entity);

            let location = self.world.entities().get_spawned(entity).unwrap();
            println!("location: {:#?}", location);

            let table = self
                .world
                .storages()
                .get_table(location.table_id as usize)
                .unwrap();
            let row_ref = unsafe { table.get_row(location.table_row as usize).unwrap() };

            for element_ref in row_ref.into_iter() {
                let type_id = self
                    .world
                    .components()
                    .get_info(element_ref.component_id())
                    .unwrap()
                    .type_id()
                    .unwrap();

                let type_registration = self.type_registry.get(type_id).unwrap();

                let component = type_registration
                    .data::<ReflectComponent>()
                    .unwrap()
                    .reflect(element_ref);

                let component = component.reflect_clone().unwrap();

                entry.components.push(component);
            }

            self.extracted_entities.push(entry);
        }

        self
    }

    pub fn build(self) -> DynamicScene {
        DynamicScene {
            resources: self.extracted_resources,
            entities: self.extracted_entities,
        }
    }

    pub fn extract_resources(mut self) -> Self {
        for element_ref in self.world.resource_entities().iter() {
            let type_id = self
                .world
                .components()
                .get_info(element_ref.component_id())
                .unwrap()
                .type_id()
                .unwrap();

            let type_registration = self.type_registry.get(type_id).unwrap();

            let resource = type_registration
                .data::<ReflectResource>()
                .unwrap()
                .reflect(element_ref);

            let resource = resource.reflect_clone().unwrap();

            self.extracted_resources.push(resource);
        }

        self
    }
}

pub fn serialize_ron<S>(serialize: S) -> Result<String, ron::Error>
where
    S: Serialize,
{
    let pretty_config = ron::ser::PrettyConfig::default()
        .indentor("  ".to_string())
        .new_line("\n".to_string());
    ron::ser::to_string_pretty(&serialize, pretty_config)
}

#[cfg(test)]
mod test {
    use std::any::{Any, TypeId};

    use downcast_rs::Downcast;
    use engine_macros::{Component, Reflect, Resource, reflect};
use serde::Serialize;

    use crate::{
        ecs::{
            components::{Component, Mutable, StorageType},
            reflect::Reflect,
            type_register::{
                self, FromType, GetTypeRegistration, ReflectComponent, ReflectSerialize,
                TypeRegistration, TypeRegistry,
            },
            worlds::World,
        },
        scene::dynamic::DynamicScene,
    };

    #[derive(Serialize, Clone, Copy, Reflect, Component)]
    #[reflect(Serialize, Component)]
    pub struct A;

    #[derive(Serialize, Clone, Copy, Reflect, Component)]
    #[reflect(Serialize, Component)]
    pub struct B {
        number: u32,
        check: bool,
    }

    #[derive(Serialize, Clone, Reflect, Resource)]
    #[reflect(Serialize, Resource)]
    pub struct C {
        list : Vec<u32>
    }


    #[test]
    pub fn example_serialize_and_deserialize() {
        let mut world = World::new();

        world.spawn((A,));
        world.spawn((
            A,
            B {
                number: 67,
                check: true,
            },
        ));

        world.insert_resource(C {
            list: vec![1, 2, 3]
        });

        let mut type_registry = TypeRegistry::new();
        type_registry.register::<A>();
        type_registry.register::<B>();
        type_registry.register::<C>();

        let dynamic_scene = DynamicScene::from_world(&world, &type_registry);
        let serialized = dynamic_scene.serialize(&type_registry).unwrap();
        println!("{}", serialized);
    }
}
