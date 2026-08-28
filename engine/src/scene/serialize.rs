
use serde::{Serialize, ser::Error};

use crate::ecs::{
    reflect::Reflect,
    type_register::{ReflectSerialize, TypeRegistry},
};

pub struct TypedReflectSerializer<'a> {
    value: &'a dyn Reflect,
    registry: &'a TypeRegistry,
}

impl<'a> TypedReflectSerializer<'a> {
    pub fn new(value: &'a dyn Reflect, registry: &'a TypeRegistry) -> Self {
        Self { value, registry }
    }
}

impl Serialize for TypedReflectSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match try_custom_serialize(self.value, self.registry, serializer) {
            Ok(result) => return result,
            Err((_, error)) => return Err(error),
        }
    }
}

fn try_custom_serialize<S: serde::Serializer>(
    value: &dyn Reflect,
    type_registry: &TypeRegistry,
    serializer: S,
) -> Result<Result<S::Ok, S::Error>, (S, S::Error)> {
    let Some(registration) = type_registry.get(value.type_id()) else {
        return Err((
            serializer,
            Error::custom(format!(
                "Type {} wasn't registred. Unable to do serialization.",
                value.name()
            )),
        ));
    };

    if let Some(reflect_serialize) = registration.data::<ReflectSerialize>() {
        Ok(reflect_serialize.serialize(value, serializer))
    } else {
        Err((
            serializer,
            Error::custom(format!(
                "Type {} doesn't contain serialization.",
                value.name()
            )),
        ))
    }
}

// pub fn serialize_ron<S>(serialize: S) -> Result<String, ron::Error>
// where
//     S: Serialize,
// {
//     let pretty_config = ron::ser::PrettyConfig::default()
//         .indentor("  ".to_string())
//         .new_line("\n".to_string());
//     ron::ser::to_string_pretty(&serialize, pretty_config)
// }
