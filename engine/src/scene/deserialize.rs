use std::fmt::{self, Formatter};

use serde::{
    Deserialize, Deserializer,
    de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
};

use crate::{
    asset::loader::LoadContext, ecs::{
        reflect::Reflect, type_register::{ReflectDeserialize, ReflectHandle, TypeRegistration, TypeRegistry},
    }, scene::registration::TypeRegistrationDeserializer,
};

pub struct TypedReflectDeserializer<'a, 'c> {
    registration: &'a TypeRegistration,
    registry: &'a TypeRegistry,
    context : &'a mut LoadContext<'c>
}

impl<'a, 'c> TypedReflectDeserializer<'a, 'c> {
    pub fn new(registration: &'a TypeRegistration, registry: &'a TypeRegistry, context : &'a mut LoadContext<'c>) -> Self {
        Self {
            registration,
            registry,
            context
        }
    }
}

impl<'de> DeserializeSeed<'de> for TypedReflectDeserializer<'_, '_> {
    type Value = Box<dyn Reflect>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if let Some(deserialize_reflect) = self.registration.data::<ReflectDeserialize>() {
            let mut value = deserialize_reflect.deserialize(deserializer)?;
            
            if let Some(reflect_handle) = self.registration.data::<ReflectHandle>() {
                reflect_handle.replace_handles(&mut value, self.context);
            }

            return Ok(value);
        } else {
            panic!("Doesn't have deserializor")
        }
    }
}

pub struct ReflectDeserializer<'a, 'c> {
    registry: &'a TypeRegistry,
    context : &'a mut LoadContext<'c>
}

impl<'a, 'c> ReflectDeserializer<'a, 'c> {
    pub fn new(registry: &'a TypeRegistry, context : &'a mut LoadContext<'c>) -> Self {
        Self { registry, context }
    }
}

impl<'de> DeserializeSeed<'de> for ReflectDeserializer<'_, '_> {
    type Value = Box<dyn Reflect>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UntypedReflectDeserializerVisitor<'a, 'c> {
            registry: &'a TypeRegistry,
            context: &'a mut LoadContext<'c>
        }

        impl<'de> Visitor<'de> for UntypedReflectDeserializerVisitor<'_, '_> {
            type Value = Box<dyn Reflect>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter
                    .write_str("map containing `type` and `value` entries for the reflected value")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let registration = map
                    .next_key_seed(TypeRegistrationDeserializer::new(self.registry))?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &"a single entry"))?;

                let value = map.next_value_seed(TypedReflectDeserializer::new(
                    registration,
                    self.registry,
                    self.context
                ))?;

                println!("{}", value.name());

                if map.next_key::<IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::invalid_length(2, &"a single entry"));
                }

                let value= value.reflect_clone().unwrap();

                Ok(value)
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Bool(v), &self))
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Signed(v), &self))
            }

            fn visit_i128<E>(self, _v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                //TODO: Change from "" to smth
                Err(Error::invalid_type(serde::de::Unexpected::Other(""), &self))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_u64(v as u64)
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_u64(v as u64)
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_u64(v as u64)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(
                    serde::de::Unexpected::Unsigned(v),
                    &self,
                ))
            }

            fn visit_u128<E>(self, _v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                //TODO: Change from "" to smth
                Err(Error::invalid_type(serde::de::Unexpected::Other(""), &self))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_f64(v as f64)
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Float(v), &self))
            }

            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(v.encode_utf8(&mut [0u8; 4]))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Str(v), &self))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(v)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Bytes(v), &self))
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_bytes(v)
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_bytes(&v)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Option, &self))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let _ = deserializer;
                Err(Error::invalid_type(serde::de::Unexpected::Option, &self))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Err(Error::invalid_type(serde::de::Unexpected::Unit, &self))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let _ = deserializer;
                Err(Error::invalid_type(
                    serde::de::Unexpected::NewtypeStruct,
                    &self,
                ))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let _ = seq;
                Err(Error::invalid_type(serde::de::Unexpected::Seq, &self))
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                let _ = data;
                Err(Error::invalid_type(serde::de::Unexpected::Enum, &self))
            }
        }

        deserializer.deserialize_map(UntypedReflectDeserializerVisitor {
            registry: self.registry,
            context: self.context
        })
    }
}
