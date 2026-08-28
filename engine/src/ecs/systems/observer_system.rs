use crate::ecs::{
    event::Event,
    observer::system_param::{On},
    systems::{
        IntoSystem,
        system::System,
    },
};

pub trait ObserverSystem<E: Event, Out = ()>:
    System<In = On<'static, E>, Out = Out> + Send + 'static
{

}

impl<E: Event, Out, T> ObserverSystem<E, Out> for T where
    T: System<In = On<'static, E>, Out = Out> + Send + 'static
{
    
}

pub trait IntoObserverSystem<E: Event, M, Out = ()>: Send + 'static {
    type System: ObserverSystem<E, Out>;

    fn into_system(this: Self) -> Self::System;
}

impl<E: Event, M, Out, S> IntoObserverSystem<E, M, Out> for S
where
    S: IntoSystem<On<'static, E>, Out, M> + Send + 'static,
    S::System: ObserverSystem<E, Out>,
    E: 'static,
{
    type System = S::System;

    fn into_system(this: Self) -> Self::System {
        IntoSystem::into_system(this)
    }
}
