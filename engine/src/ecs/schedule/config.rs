use crate::{ecs::{change_detection::params::Mut, resource::Resource, schedule::schedule_system::ScheduleSystem, systems::{IntoSystem, system::System}}, input::button_input::ButtonInput};
use std::hash::Hash;

pub type BoxedCondition<In = ()> = Box<dyn System<In = In, Out = bool>>;

pub trait SystemCondition<Marker, In = ()>: IntoSystem<In, bool, Marker> {}

impl<Marker, In, F> SystemCondition<Marker, In> for F
where F: IntoSystem<In, bool, Marker> {}

pub trait IntoSchedule<T: Schedulable, Marker>: Sized {
    fn into_schedule(self) -> ScheduleConfig<T>;

    fn run_if<M>(self, condition: impl SystemCondition<M>) -> ScheduleConfig<T> {
        self.into_schedule().run_if(condition)
    }
}

pub trait Schedulable {
    fn into_schedulable(self) -> ScheduleConfig<Self>
    where
        Self: Sized;
}

pub enum ScheduleConfig<T: Schedulable> {
    One {
        node: T,
        conditions: Vec<BoxedCondition>,
    },
    Many  {
        children : Vec<ScheduleConfig<T>>,
        conditions: Vec<BoxedCondition>,
    }
}

impl<T : Schedulable> IntoSchedule<T, ()> for ScheduleConfig<T> {
    fn into_schedule(self) -> ScheduleConfig<T> {
        self
    }

    fn run_if<M>(mut self, condition: impl SystemCondition<M>) -> ScheduleConfig<T> {
        match &mut self {
            ScheduleConfig::One { 
                node: _, 
                conditions 
            } => {
                conditions.push(new_condition(condition));
            },
            ScheduleConfig::Many { 
                children: _, 
                conditions 
            } => {
                conditions.push(new_condition(condition));
            }
        }

        self
    }
}

impl<F, Marker> IntoSchedule<ScheduleSystem, Marker> for F
where
    F: IntoSystem<(), (), Marker>,
{
    fn into_schedule(self) -> ScheduleConfig<ScheduleSystem> {
        ScheduleConfig::One {
            node: Box::new(IntoSystem::into_system(self)),
            conditions: Vec::new()
        }
    }
}

pub struct ScheduleConfigTupleMarker;
macro_rules! impl_into_schedule {
    ($(($param:ident, $marker:ident)),*) => {
        impl<$($param),*, $($marker),*,> IntoSchedule<ScheduleSystem, (ScheduleConfigTupleMarker, $($marker),*,)> for ($($param,)+)
        where 
            $($param : IntoSchedule<ScheduleSystem, $marker>),*,
        {
            fn into_schedule(self) -> ScheduleConfig<ScheduleSystem> {
                let ($($param,)+) = self;
                ScheduleConfig::Many {
                    children: vec![$($param.into_schedule(),)+], 
                    conditions: Vec::new() 
                }
            }
        }
    };
}

impl_into_schedule!((A, M1), (B, M2));
impl_into_schedule!((A, M1), (B, M2), (C, M3));
impl_into_schedule!((A, M1), (B, M2), (C, M3), (D, M4));
impl_into_schedule!((A, M1), (B, M2), (C, M3), (D, M4), (E, M5));





fn new_condition<M>(condition: impl SystemCondition<M>) -> BoxedCondition {
    let condition_system = IntoSystem::into_system(condition);
    Box::new(condition_system)
}


pub fn in_state<S : Resource + PartialEq + Eq>(state: S) -> impl FnMut(&S) -> bool {
    move |current_state: &S| {
        current_state.eq(&state)
    }
}

pub fn resource_changed<S : Resource>() -> impl FnMut(Mut<S>) -> bool {
    move |resource: Mut<'_, S>| {
        resource.is_changed()
    }
}

pub fn changed_state_to<S : Resource + PartialEq + Eq>(state : S) -> impl FnMut(Mut<S>) -> bool {
    move |current_state: Mut<'_, S>| {
        current_state.is_changed() && current_state.eq(&state)
    }
}

pub fn input_just_pressed<T : Clone + Eq + Hash + Send + Sync + 'static>(key_code : T) -> impl FnMut(&ButtonInput<T>) -> bool {
    move |button_input : &ButtonInput<T>| {
        button_input.just_pressed(key_code.clone())
    }
}

pub fn input_pressed<T : Clone + Eq + Hash + Send + Sync + 'static>(key_code : T) -> impl FnMut(&ButtonInput<T>) -> bool {
    move |button_input : &ButtonInput<T>| {
        button_input.pressed(key_code.clone())
    }
}

pub fn input_just_released<T : Clone + Eq + Hash + Send + Sync + 'static>(key_code : T) -> impl FnMut(&ButtonInput<T>) -> bool {
    move |button_input : &ButtonInput<T>| {
        button_input.just_released(key_code.clone())
    }
}