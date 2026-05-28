use crate::ecs::{bundle::Bundle, event::Event, observer::system_param::{On, OnEventState}, systems::{IntoSystem, function_system::{F1, F2, F3, F4, FunctionSystem, SystemParamFunction, SystemParamTuple}, system::System}};


pub trait ObserverSystem<E: Event, Out = ()>:
    System<In = On<'static, E>, Out = Out> + Send + 'static
{
    fn set_on_state(&mut self, event : *mut u8);
}

impl<E: Event, Out : 'static, F> ObserverSystem<E, Out> for FunctionSystem<F1<On<'static, E>>, On<'static, E>, Out, F>
where F : SystemParamFunction<F1<On<'static, E>>, Param = On<'static, E>> {
    fn set_on_state(&mut self, event : *mut u8) {
        if let Some(state) = &mut self.state {
                *state = OnEventState {
                    ptr: event as *mut E
                }
        }
    }
}

impl<E: Event, Out : 'static, A : 'static, F> ObserverSystem<E, Out> for FunctionSystem<F2<On<'static, E>, A>, On<'static, E>, Out, F>
where  
    A : SystemParamTuple,
    F : SystemParamFunction<F2<On<'static, E>, A>, Param = (On<'static, E>, A)> {
    fn set_on_state(&mut self, event : *mut u8) {
        if let Some(state) = &mut self.state {
            state.0 = OnEventState {
                ptr: event as *mut E
            }
        }
    }
}

impl<E: Event, Out : 'static, A : 'static, B : 'static, F> ObserverSystem<E, Out> for FunctionSystem<F3<On<'static, E>, A, B>, On<'static, E>, Out, F>
where  
    A : SystemParamTuple,
    B : SystemParamTuple,
    F : SystemParamFunction<F3<On<'static, E>, A, B>, Param = (On<'static, E>, A, B)> {
    fn set_on_state(&mut self, event : *mut u8) {
        if let Some(state) = &mut self.state {
            state.0 = OnEventState {
                ptr: event as *mut E
            }
        }
    }
}

impl<E: Event, Out : 'static, A : 'static, B : 'static, C : 'static, F> ObserverSystem<E, Out> for FunctionSystem<F4<On<'static, E>, A, B, C>, On<'static, E>, Out, F>
where  
    A : SystemParamTuple,
    B : SystemParamTuple,
    C : SystemParamTuple,
    F : SystemParamFunction<F4<On<'static, E>, A, B, C>, Param = (On<'static, E>, A, B, C)> {
    fn set_on_state(&mut self, event : *mut u8) {
        if let Some(state) = &mut self.state {
            state.0 = OnEventState {
                ptr: event as *mut E
            }
        }
    }
}


pub trait IntoObserverSystem<E: Event, M, Out = ()>: Send + 'static {
    type System: ObserverSystem<E, Out>;
    
    fn into_system(this: Self) -> Self::System;
}

impl<E: Event, M, Out, S> IntoObserverSystem<E, M, Out> for S
where
    S: IntoSystem<On<'static, E>, Out, M> + Send + 'static,
    S::System: ObserverSystem<E, Out>,
    E: 'static
{
    type System = S::System;

    fn into_system(this: Self) -> Self::System {
        IntoSystem::into_system(this)
    }
}