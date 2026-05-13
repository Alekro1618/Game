use crate::ecs::systems::{function_system::{FunctionSystem, SystemParamFunction}, system::System};

pub mod query;
pub mod system;
pub mod node;
pub mod function_system;
pub mod commands;
pub mod observer_system;

pub trait IntoSystem<In, Out, Marker>: Sized {
    type System: System<In = In, Out = Out>;
    fn into_system(this: Self) -> Self::System;
}


impl<Marker, In, Out, F> IntoSystem<In, Out, Marker> for F
where
    Marker: 'static,
    In: 'static,
    Out: 'static,
    F: SystemParamFunction<Marker>,
{
    type System = FunctionSystem<Marker, In, Out, F>;
    fn into_system(func: Self) -> Self::System {
        FunctionSystem::new(func)
    }
}