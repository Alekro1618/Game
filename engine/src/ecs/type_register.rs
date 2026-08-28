use std::{
    any::TypeId,
    collections::HashMap,
};

use downcast_rs::{Downcast, impl_downcast};
use serde::{Deserialize, Serialize};

use crate::{
    ecs::{components::Component, reflect::Reflect, storage::element_ref::ElementRef},
    error::error::{Error, ErrorKind},
};

pub trait TypeData: Downcast + Send + Sync
where
    Self: 'static,
{
}

impl_downcast!(TypeData);

impl<T: 'static + Send + Sync> TypeData for T {}

pub trait FromType<T> {
    fn from_type() -> Self;
    fn insert_dependencies(_type_registration: &mut TypeRegistration) {}
}

pub trait GetTypeRegistration: 'static {
    fn get_type_registration() -> TypeRegistration;
}

pub struct TypeRegistration {
    data: HashMap<TypeId, Box<dyn TypeData>>,
}

impl TypeRegistration {
    pub fn of<T: Reflect>() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert<D: TypeData>(&mut self, data: D) {
        self.data.insert(TypeId::of::<D>(), Box::new(data));
    }

    pub fn data<D: TypeData>(&self) -> Option<&D> {
        self.data
            .get(&TypeId::of::<D>())
            .and_then(|value| value.downcast_ref())
    }
}

pub struct TypeRegistry {
    registrations: HashMap<TypeId, TypeRegistration>,
    type_path_to_id: HashMap<&'static str, TypeId>,
}

impl TypeRegistry {
    pub fn empty() -> Self {
        Self {
            registrations: HashMap::new(),
            type_path_to_id: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        let registry = Self::empty();
        //TODO: REGISTER BASIC TYPES
        registry
    }

    pub fn register<T: GetTypeRegistration>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        let registration = T::get_type_registration();

        self.insert(type_id, registration);

        self
    }

    fn insert(&mut self, type_id: TypeId, serialize_method: TypeRegistration) {
        self.registrations.insert(type_id, serialize_method);
    }

    pub fn get(&self, type_id: TypeId) -> Option<&TypeRegistration> {
        self.registrations.get(&type_id)
    }

    pub fn get_with_type_path(&self, type_path: &str) -> Result<&TypeRegistration, Error> {
        let type_id = self.type_path_to_id.get(type_path).ok_or(Error::new(
            ErrorKind::System,
            format!("Type path {} wasn't registered", type_path),
        ))?;

        self.get(*type_id).ok_or(Error::new(
            ErrorKind::System,
            format!("No type registry for given type path {}", type_path),
        ))
    }
}

pub struct ReflectDeserialize {
    pub func: fn(
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn Reflect>, erased_serde::Error>,
}

impl ReflectDeserialize {
    pub fn deserialize<'de, D>(&self, deserializer: D) -> Result<Box<dyn Reflect>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
        (self.func)(&mut erased)
            .map_err(<<D as serde::Deserializer<'de>>::Error as serde::de::Error>::custom)
    }
}

impl<T: Reflect + for<'a> Deserialize<'a>> FromType<T> for ReflectDeserialize {
    fn from_type() -> Self {
        ReflectDeserialize {
            func: |deserializer| Ok(Box::new(T::deserialize(deserializer)?)),
        }
    }
}

pub struct ReflectSerialize {
    get_serialize: fn(value: &dyn Reflect) -> &'_ dyn erased_serde::Serialize,
}

impl<T: Reflect + erased_serde::Serialize> FromType<T> for ReflectSerialize {
    fn from_type() -> Self {
        ReflectSerialize {
            get_serialize: |value| {
                value.downcast_ref::<T>().unwrap_or_else(|| {
                    panic!("failed when called on type");
                })
            },
        }
    }
}

impl ReflectSerialize {
    pub fn get_serialize<'a>(&self, value: &'a dyn Reflect) -> &'a dyn erased_serde::Serialize {
        (self.get_serialize)(value)
    }

    /// Serializes a reflected value.
    pub fn serialize<S>(&self, value: &dyn Reflect, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.get_serialize)(value).serialize(serializer)
    }
}

pub struct ReflectComponent {
    pub reflect: for<'w> fn(ElementRef<'w>) -> &'w dyn Reflect,
}

impl ReflectComponent {
    pub fn reflect<'w>(&self, element_ref: ElementRef<'w>) -> &'w dyn Reflect {
        (self.reflect)(element_ref)
    }
}

impl<C: Component + Reflect> FromType<C> for ReflectComponent {
    fn from_type() -> Self {
        ReflectComponent {
            reflect: |entity: ElementRef<'_>| unsafe { entity.get::<C>() },
        }
    }
}
