use std::fmt::Formatter;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeStruct},
};

use crate::{
    asset::loader::LoadContext, ecs::{
        entities::id::EntityId,
        reflect::Reflect,
        type_register::TypeRegistry,
    }, scene::{
        deserialize::{ReflectDeserializer, TypedReflectDeserializer}, dynamic::{DynamicEntity, DynamicScene}, registration::TypeRegistrationDeserializer, serialize::TypedReflectSerializer,
    },
};

pub const SCENE_STRUCT: &str = "Scene";
pub const SCENE_RESOURCES: &str = "resources";
pub const SCENE_ENTITIES: &str = "entities";
pub const ENTITY_STRUCT: &str = "Entity";
pub const ENTITY_FIELD_COMPONENTS: &str = "components";

pub struct SceneSerializer<'a> {
    pub scene: &'a DynamicScene,
    pub registry: &'a TypeRegistry,
}

impl<'a> SceneSerializer<'a> {
    pub fn new(scene: &'a DynamicScene, registry: &'a TypeRegistry) -> Self {
        SceneSerializer { scene, registry }
    }
}

impl<'a> Serialize for SceneSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(SCENE_STRUCT, 2)?;
        state.serialize_field(
            SCENE_RESOURCES,
            &SceneMapSerializer {
                entries: &self.scene.resources,
                registry: self.registry,
            },
        )?;
        state.serialize_field(
            SCENE_ENTITIES,
            &EntitiesSerializer {
                entities: &self.scene.entities,
                registry: self.registry,
            },
        )?;
        state.end()
    }
}

/// Handles serialization of multiple entities as a map of entity id to serialized entity.
pub struct EntitiesSerializer<'a> {
    /// The entities to serialize.
    pub entities: &'a [DynamicEntity],
    /// Type registry in which the component types used by the entities are registered.
    pub registry: &'a TypeRegistry,
}

impl<'a> Serialize for EntitiesSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(self.entities.len()))?;
        for entity in self.entities {
            state.serialize_entry(
                &entity.entity,
                &EntitySerializer {
                    entity,
                    registry: self.registry,
                },
            )?;
        }
        state.end()
    }
}

/// Handles entity serialization as a map of component type to component value.
pub struct EntitySerializer<'a> {
    /// The entity to serialize.
    pub entity: &'a DynamicEntity,
    /// Type registry in which the component types used by the entity are registered.
    pub registry: &'a TypeRegistry,
}

impl<'a> Serialize for EntitySerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(ENTITY_STRUCT, 1)?;
        state.serialize_field(
            ENTITY_FIELD_COMPONENTS,
            &SceneMapSerializer {
                entries: &self.entity.components,
                registry: self.registry,
            },
        )?;
        state.end()
    }
}

/// Handles serializing a list of values with a unique type as a map of type to value.
///
/// Used to serialize scene resources in [`SceneSerializer`] and entity components in [`EntitySerializer`].
/// Note that having several entries of the same type in `entries` will lead to an error when using the RON format and
/// deserializing through [`SceneMapDeserializer`].
///
/// Note: The entries are sorted by type path before they're serialized.
pub struct SceneMapSerializer<'a> {
    /// List of boxed values of unique type to serialize.
    pub entries: &'a [Box<dyn Reflect>],
    /// Type registry in which the types used in `entries` are registered.
    pub registry: &'a TypeRegistry,
}

impl<'a> Serialize for SceneMapSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(self.entries.len()))?;
        let entries = self
            .entries
            .iter()
            .map(|entry| (entry.name(), entry))
            .collect::<Vec<_>>();

        for (name, reflect) in entries {
            state.serialize_entry(
                &name,
                &TypedReflectSerializer::new(reflect.as_reflect(), self.registry),
            )?;
        }
        state.end()
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum SceneField {
    Resources,
    Entities,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum EntityField {
    Components,
}

/// Handles scene deserialization.
pub struct SceneDeserializer<'a, 'c> {
    /// Type registry in which the components and resources types used in the scene to deserialize are registered.
    pub type_registry: &'a TypeRegistry,
    /// The [`LoadFromPath`] implementation allowing us to deserialize asset handles.
    pub text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> DeserializeSeed<'de> for SceneDeserializer<'a, '_> {
    type Value = DynamicScene;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            SCENE_STRUCT,
            &[SCENE_RESOURCES, SCENE_ENTITIES],
            SceneVisitor {
                type_registry: self.type_registry,
                text: self.text,
            },
        )
    }
}

struct SceneVisitor<'a, 'c> {
    type_registry: &'a TypeRegistry,
    text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> Visitor<'de> for SceneVisitor<'a, '_> {
    type Value = DynamicScene;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("scene struct")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let resources = seq
            .next_element_seed(SceneMapDeserializer {
                registry: self.type_registry,
                text: self.text,
            })?
            .ok_or_else(|| Error::missing_field(SCENE_RESOURCES))?;

        let entities = seq
            .next_element_seed(SceneEntitiesDeserializer {
                type_registry: self.type_registry,
                text: self.text,
            })?
            .ok_or_else(|| Error::missing_field(SCENE_ENTITIES))?;

        Ok(DynamicScene {
            resources,
            entities,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut resources = None;
        let mut entities = None;
        while let Some(key) = map.next_key()? {
            match key {
                SceneField::Resources => {
                    if resources.is_some() {
                        return Err(Error::duplicate_field(SCENE_RESOURCES));
                    }
                    resources = Some(map.next_value_seed(SceneMapDeserializer {
                        registry: self.type_registry,
                        text: self.text,
                    })?);
                }
                SceneField::Entities => {
                    if entities.is_some() {
                        return Err(Error::duplicate_field(SCENE_ENTITIES));
                    }
                    entities = Some(map.next_value_seed(SceneEntitiesDeserializer {
                        type_registry: self.type_registry,
                        text: self.text,
                    })?);
                }
            }
        }

        let resources = resources.ok_or_else(|| Error::missing_field(SCENE_RESOURCES))?;
        let entities = entities.ok_or_else(|| Error::missing_field(SCENE_ENTITIES))?;

        Ok(DynamicScene {
            resources,
            entities,
        })
    }
}

/// Handles deserialization for a collection of entities.
pub struct SceneEntitiesDeserializer<'a, 'c> {
    /// Type registry in which the component types used by the entities to deserialize are registered.
    pub type_registry: &'a TypeRegistry,
    /// The [`LoadFromPath`] implementation allowing us to deserialize asset handles.
    pub text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> DeserializeSeed<'de> for SceneEntitiesDeserializer<'a, '_> {
    type Value = Vec<DynamicEntity>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(SceneEntitiesVisitor {
            type_registry: self.type_registry,
            text: self.text,
        })
    }
}

struct SceneEntitiesVisitor<'a, 'c> {
    type_registry: &'a TypeRegistry,
    text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> Visitor<'de> for SceneEntitiesVisitor<'a, '_> {
    type Value = Vec<DynamicEntity>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("map of entities")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut entities = Vec::new();
        while let Some(entity) = map.next_key::<EntityId>()? {
            let entity = map.next_value_seed(SceneEntityDeserializer {
                entity,
                type_registry: self.type_registry,
                text: self.text,
            })?;
            entities.push(entity);
        }

        Ok(entities)
    }
}

/// Handle deserialization of an entity and its components.
pub struct SceneEntityDeserializer<'a, 'c> {
    /// Id of the deserialized entity.
    pub entity: EntityId,
    /// Type registry in which the component types used by the entity to deserialize are registered.
    pub type_registry: &'a TypeRegistry,
    /// The [`LoadFromPath`] implementation allowing us to deserialize asset handles.
    pub text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> DeserializeSeed<'de> for SceneEntityDeserializer<'a, '_> {
    type Value = DynamicEntity;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            ENTITY_STRUCT,
            &[ENTITY_FIELD_COMPONENTS],
            SceneEntityVisitor {
                entity: self.entity,
                registry: self.type_registry,
                text: self.text,
            },
        )
    }
}

struct SceneEntityVisitor<'a, 'c> {
    entity: EntityId,
    registry: &'a TypeRegistry,
    text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> Visitor<'de> for SceneEntityVisitor<'a, '_> {
    type Value = DynamicEntity;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("entities")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let components = seq
            .next_element_seed(SceneMapDeserializer {
                registry: self.registry,
                text: self.text,
            })?
            .ok_or_else(|| Error::missing_field(ENTITY_FIELD_COMPONENTS))?;

        Ok(DynamicEntity {
            entity: self.entity,
            components,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut components = None;
        while let Some(key) = map.next_key()? {
            match key {
                EntityField::Components => {
                    if components.is_some() {
                        return Err(Error::duplicate_field(ENTITY_FIELD_COMPONENTS));
                    }

                    components = Some(map.next_value_seed(SceneMapDeserializer {
                        registry: self.registry,
                        text: self.text,
                    })?);
                }
            }
        }

        let components = components
            .take()
            .ok_or_else(|| Error::missing_field(ENTITY_FIELD_COMPONENTS))?;
        Ok(DynamicEntity {
            entity: self.entity,
            components,
        })
    }
}

/// Handles deserialization of a sequence of values with unique types.
pub struct SceneMapDeserializer<'a, 'c> {
    /// Type registry in which the types of the values to deserialize are registered.
    pub registry: &'a TypeRegistry,
    /// The [`LoadFromPath`] implementation allowing us to deserialize asset handles.
    pub text: &'a mut LoadContext<'c>,
}

impl<'a , 'de> DeserializeSeed<'de> for SceneMapDeserializer<'a, '_> {
    type Value = Vec<Box<dyn Reflect>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(SceneMapVisitor {
            registry: self.registry,
            text: self.text,
        })
    }
}

struct SceneMapVisitor<'a, 'c> {
    registry: &'a TypeRegistry,
    text: &'a mut LoadContext<'c>,
}

impl<'a, 'de> Visitor<'de> for SceneMapVisitor<'a, '_> {
    type Value = Vec<Box<dyn Reflect>>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("map of reflect types")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut dynamic_properties = Vec::new();
        while let Some(entity) = seq.next_element_seed(ReflectDeserializer::new(self.registry))? {
            dynamic_properties.push(entity);
        }

        Ok(dynamic_properties)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some(registration) =
            map.next_key_seed(TypeRegistrationDeserializer::new(self.registry))?
        {
            let typed_reflect_deserializer = TypedReflectDeserializer::new(
                    registration,
                    self.registry,
            );

            let value: Box<dyn Reflect> = map.next_value_seed(
                typed_reflect_deserializer
            )?;

            let value = value.reflect_clone().unwrap();

            entries.push(value);
        }

        Ok(entries)
    }
}
