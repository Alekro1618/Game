use std::{any::TypeId, marker::PhantomData};



pub trait SystemSet: 'static + Send + Sync {
    fn id(&self) -> TypeId { TypeId::of::<Self>() }
    fn name(&self) -> &'static str { std::any::type_name::<Self>() }
}

#[derive(Debug, Clone)]
pub enum OrderingConstraint {
    Before(TypeId),
    After(TypeId),
}
