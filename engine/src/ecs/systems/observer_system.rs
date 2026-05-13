use crate::ecs::{bundle::Bundle, event::Event, observer::system_param::{On, OnEventState}, systems::{IntoSystem, function_system::{FunctionSystem, SystemParamFunction, SystemParamTuple}, system::System}};


pub trait ObserverSystem<E: Event, In = (), Out = ()>:
    System<In = (On<'static,  E>, In), Out = Out> + Send + 'static
{
    fn set_on_state(&mut self, event : *mut u8);
}

impl<E: Event, M : 'static, In: SystemParamTuple + 'static, Out : 'static, F> ObserverSystem<E, In, Out> for FunctionSystem<M, (On<'static, E>, In), Out, F>
where F : SystemParamFunction<M, Param = (On<'static, E>, In)> {
    fn set_on_state(&mut self, event : *mut u8) {
        if let Some(state) = &mut self.state {
                state.0 = OnEventState {
                    ptr: event as *mut E
                }
        }
    }
}


pub trait IntoObserverSystem<E: Event, M, In = (), Out = ()>: Send + 'static {
    type System: ObserverSystem<E, Out>;
    
    fn into_system(this: Self) -> Self::System;
}

impl<E: Event, M, In, Out, S> IntoObserverSystem<E, M, In, Out> for S
where
    S: IntoSystem<On<'static,  E>, Out, M> + Send + 'static,
    S::System: ObserverSystem<E, Out>,
    E: 'static
{
    type System = S::System;

    fn into_system(this: Self) -> Self::System {
        IntoSystem::into_system(this)
    }
}