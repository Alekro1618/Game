use std::sync::{Arc, RwLock};

use crate::{asset::loader::AssetLoader, ecs::{type_register::{AppTypeRegistry, TypeRegistry}, worlds::{FromWorld, World}}, error::error::{Error, ErrorKind}, scene::{dynamic::DynamicScene, scene::Scene, serde::SceneDeserializer}, ui::ui_node::OverflowClipBox::PaddingBox};
use ron::de::SpannedError;
use serde::de::DeserializeSeed;

pub struct SceneLoader {
    type_registry: Arc<RwLock<TypeRegistry>>,
}

impl FromWorld for SceneLoader {
    fn from_world(world: &mut World) -> Self {
        let type_registry = world.resource::<AppTypeRegistry>();
        SceneLoader {
            type_registry: type_registry.0.clone(),
        }
    }
}

pub enum SceneLoadingError {
    IO(String),
    Ron(SpannedError)
}

impl Into<Error> for SceneLoadingError {
    fn into(self) -> Error {
        match self {
            SceneLoadingError::IO(message) => Error::new(ErrorKind::IO, message),
            SceneLoadingError::Ron(spanned_error) => Error::new(ErrorKind::Asset, spanned_error.to_string()),
        }
    }
}

impl AssetLoader for SceneLoader {
    type Asset = DynamicScene;

    type Error = SceneLoadingError;

    fn load(
        &self,
        reader: &mut Vec<u8>,
        load_context: &mut crate::asset::loader::LoadContext<'_>,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let mut deserializer = ron::de::Deserializer::from_bytes(reader)
                .map_err(|e| SceneLoadingError::Ron(e))?;

            let scene_deserializer = SceneDeserializer {
                type_registry: &self.type_registry.read().unwrap(),
                text: load_context,
            };

            scene_deserializer
                .deserialize(&mut deserializer)
                .map_err(|e| SceneLoadingError::Ron(deserializer.span_error(e)))
        }
    }
}