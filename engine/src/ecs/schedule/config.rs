use crate::{ecs::{resource::Resource, schedule::schedule_system::ScheduleSystem, systems::{IntoSystem, system::System}}, input::button_input::ButtonInput};
use std::hash::Hash;

pub type BoxedCondition<In = ()> = Box<dyn System<In = In, Out = bool>>;

pub trait SystemCondition<Marker, In = ()>: IntoSystem<In, bool, Marker> 
{

}

impl<Marker, In, F> SystemCondition<Marker, In> for F where
    F: IntoSystem<In, bool, Marker>
{
}

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

pub struct ScheduleConfig<T: Schedulable> {
    pub(crate) node: T,
    pub(crate) conditions: Vec<BoxedCondition>,
}

impl<T : Schedulable> IntoSchedule<T, ()> for ScheduleConfig<T> {
    fn into_schedule(self) -> ScheduleConfig<T> {
        self
    }

    fn run_if<M>(mut self, condition: impl SystemCondition<M>) -> ScheduleConfig<T> {
        self.conditions.push(new_condition(condition));
        self
    }
}

impl<F, Marker> IntoSchedule<ScheduleSystem, Marker> for F
where
    F: IntoSystem<(), (), Marker>,
{
    fn into_schedule(self) -> ScheduleConfig<ScheduleSystem> {
        ScheduleConfig {
            node: Box::new(IntoSystem::into_system(self)),
            conditions: Vec::new()
        }
    }
}


fn new_condition<M>(condition: impl SystemCondition<M>) -> BoxedCondition {
    let condition_system = IntoSystem::into_system(condition);
    Box::new(condition_system)
}


pub fn in_state<S : Resource + PartialEq + Eq>(state: S) -> impl FnMut(&S) -> bool {
    move |current_state: &S| {
        *current_state == state
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