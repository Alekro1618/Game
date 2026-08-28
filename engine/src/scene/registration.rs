use core::{fmt, fmt::Formatter};
use serde::de::{DeserializeSeed, Error, Visitor};

use crate::ecs::type_register::{TypeRegistration, TypeRegistry};

pub struct TypeRegistrationDeserializer<'a> {
    registry: &'a TypeRegistry,
}

impl<'a> TypeRegistrationDeserializer<'a> {
    /// Creates a new [`TypeRegistrationDeserializer`].
    pub fn new(registry: &'a TypeRegistry) -> Self {
        Self { registry }
    }
}

impl<'a, 'de> DeserializeSeed<'de> for TypeRegistrationDeserializer<'a> {
    type Value = &'a TypeRegistration;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TypeRegistrationVisitor<'a>(&'a TypeRegistry);

        impl<'de, 'a> Visitor<'de> for TypeRegistrationVisitor<'a> {
            type Value = &'a TypeRegistration;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("string containing `type` entry for the reflected value")
            }

            fn visit_str<E>(self, type_path: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.0
                    .get_with_type_path(type_path)
                    .map_err(|err| Error::custom(err.to_string()))
            }
        }

        deserializer.deserialize_str(TypeRegistrationVisitor(self.registry))
    }
}
