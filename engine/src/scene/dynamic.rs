
use serde::Serialize;

use crate::{
    asset::asset::Asset,
    ecs::{
        entities::id::EntityId,
        reflect::Reflect,
        type_register::{ReflectComponent, TypeRegistry},
        worlds::World,
    },
    scene::{scene::Scene, serde::SceneSerializer},
};

pub struct DynamicEntity {
    pub entity: EntityId,
    pub components: Vec<Box<dyn Reflect>>,
}

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
}

impl Asset for DynamicScene {}

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

            let component = type_registration
                .data::<ReflectComponent>()
                .unwrap()
                .reflect(element_ref);

            let component = component.reflect_clone().unwrap();

            self.extracted_resources.push(component);
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
    use std::any::Any;

    use downcast_rs::Downcast;
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

    #[derive(Serialize, Clone, Copy)]
    pub struct A;

    impl Component for A {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    impl Reflect for A {
        fn as_reflect(&self) -> &dyn Reflect {
            self as &dyn Reflect
        }

        fn into_reflect(self) -> Box<dyn Reflect> {
            Box::new(self)
        }

        fn reflect_clone(&self) -> Result<Box<dyn Reflect>, crate::error::error::Error> {
            Ok(Box::new(A))
        }

        fn name(&self) -> String {
            "A".into()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl GetTypeRegistration for A {
        fn get_type_registration() -> TypeRegistration {
            let mut type_register = TypeRegistration::of::<A>();
            type_register.insert::<ReflectComponent>(FromType::<Self>::from_type());
            type_register.insert::<ReflectSerialize>(FromType::<Self>::from_type());
            type_register
        }
    }

    #[derive(Serialize)]
    pub struct B {
        number: u32,
        check: bool,
    }

    impl Component for B {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    impl Reflect for B {
        fn as_reflect(&self) -> &dyn Reflect {
            self as &dyn Reflect
        }

        fn into_reflect(self) -> Box<dyn Reflect> {
            Box::new(self)
        }

        fn reflect_clone(&self) -> Result<Box<dyn Reflect>, crate::error::error::Error> {
            Ok(Box::new(Self {
                number: self.number,
                check: self.check,
            }))
        }

        fn name(&self) -> String {
            "B".into()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl GetTypeRegistration for B {
        fn get_type_registration() -> TypeRegistration {
            let mut type_register = TypeRegistration::of::<B>();
            type_register.insert::<ReflectComponent>(FromType::<Self>::from_type());
            type_register.insert::<ReflectSerialize>(FromType::<Self>::from_type());
            type_register
        }
    }

    #[test]
    pub fn example_serialize() {
        let mut world = World::new();

        world.spawn((A,));
        world.spawn((
            A,
            B {
                number: 67,
                check: true,
            },
        ));
        let mut type_registry = TypeRegistry::new();
        type_registry.register::<A>();
        type_registry.register::<B>();

        let dynamic_scene = DynamicScene::from_world(&world, &type_registry);
        let serialized = dynamic_scene.serialize(&type_registry).unwrap();
    }
}
