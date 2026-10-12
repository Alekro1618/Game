use std::any::{Any, TypeId};


use crate::error::error::Error;

pub trait Reflect: Send + Sync + Any
where
    Self: 'static,
{
    fn as_reflect(&self) -> &dyn Reflect;
    fn into_reflect(self) -> Box<dyn Reflect>;
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, Error>;
    fn name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn get_type_id(&self) -> Option<TypeId>;
}

impl dyn Reflect {
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait FromReflect: Reflect + Sized {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self>;
}