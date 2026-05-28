use std::{any::{Any, TypeId}, collections::HashMap};

use downcast_rs::{Downcast, impl_downcast};
use ron::value;
use serde::{Deserialize, Serialize};

use crate::ecs::{components::Component, relfect::Reflect, storage::{element_ref::ElementRef, row::RowRef}};


pub trait TypeData: Downcast + Send + Sync
where Self: 'static {}

impl_downcast!(TypeData);

impl<T : 'static + Send + Sync> TypeData for T {} 

pub trait FromType<T> {
    fn from_type() -> Self;
    fn insert_dependencies(_type_registration: &mut TypeRegistration) {}
}

pub trait GetTypeRegistration: 'static {
    fn get_type_registration() -> TypeRegistration;
}

pub struct TypeRegistration {
    data : HashMap<TypeId, Box<dyn TypeData>>,
}

impl TypeRegistration {
    pub fn of<T>() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn insert<D: TypeData>(&mut self, data : D) {
        self.data.insert(TypeId::of::<D>(), Box::new(data));   
    }

    pub fn data<D : TypeData>(&self) -> Option<&D> {
        self.data
            .get(&TypeId::of::<D>())
            .and_then(|value| value.downcast_ref())
    }
}


pub struct TypeRegistry {
    registrations : HashMap<TypeId, TypeRegistration>,
}

impl TypeRegistry {
    pub fn empty() -> Self {
        Self { registrations: HashMap::new() }
    }

    pub fn new() -> Self {
        let mut registry = Self::empty();

        registry
    }

    pub fn register<T : GetTypeRegistration>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        let registration = T::get_type_registration();
        
        self.insert(type_id, registration);
        
        self
    }

    fn insert(&mut self, type_id : TypeId, serialize_method : TypeRegistration) {
        self.registrations.insert(type_id, serialize_method);
    }


    pub fn get(&self, type_id : TypeId) -> Option<&TypeRegistration> {
        self.registrations.get(&type_id)
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

pub struct ReflectSerialize {
    get_serialize: fn(value: &dyn Reflect) -> &'_ dyn erased_serde::Serialize,
}

impl<T: Reflect + erased_serde::Serialize> FromType<T> for ReflectSerialize {
    fn from_type() -> Self {
        ReflectSerialize {
            get_serialize: |value| {
                value
                    .downcast_ref::<T>()
                    .unwrap_or_else(|| {
                        panic!(
                            "failed when called on type"
                        );
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
    pub fn reflect<'w>(&self, element_ref : ElementRef<'w>) -> &'w dyn Reflect {
        (self.reflect)(element_ref)
    }
}

impl<C: Component + Reflect> FromType<C> for ReflectComponent {
    fn from_type() -> Self {
        ReflectComponent {  
            reflect: |entity: ElementRef<'_>| unsafe { entity.get::<C>() }
        }
    }
}