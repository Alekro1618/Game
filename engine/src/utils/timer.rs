use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use engine_macros::Resource;

use crate::{app::{FIXED_UPDATE, main_schedule::ScheduleLabel, plugin::{GenericPlugin, Plugin}}, ecs::{change_detection::params::Mut, components::{Component, Mutable, StorageType}, entities::id::EntityId, event::Event, resource::Resource, systems::{commands::Commands, function_system::{F1, F2, F3}, query::Query}}, utils::resources::GameSpeed};

pub const TIME_EPSILON : f32 = 1e-2;


pub struct OnTimerFinished<T> 
where T : Sync + Send + 'static {
    pub result : Option<T>
}

impl<T>  Event for OnTimerFinished<T> 
where T : Sync + Send + 'static {
    
}

pub struct Timer<T>
where T : Sync + Send + 'static {
    pub result : Option<T>,
    pub value : f32,
    pub begin : f32,
    pub end : f32,
    done : bool,
    fliped : bool,
}

impl<T> Component for Timer<T>
where T : Sync + Send + 'static {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

impl<T> Timer<T>
where T : Sync + Send + 'static {
    pub fn new( begin : f32, end : f32) -> Self {
        if begin < end {
            Self {
                result : None,
                value: begin,
                begin,
                end, 
                done: false,
                fliped: false 
            }
        } else {
            Self { 
                result: None,
                value: end,
                begin, 
                end,
                done: false,
                fliped: true }
        }
    }

    pub fn set_result(&mut self, result : T) {
        self.result = Some(result);
    }

    pub fn reset(&mut self) {
        if !self.fliped {
            self.value = self.begin;
        } else {
            self.value = self.end;
        }
        self.done = false;
    }

    pub fn update(&mut self, interval : f32) {
        if !self.fliped {
            self.value += interval;
            if self.end - self.value < TIME_EPSILON {
                self.value = self.end;
                self.done = true;
            }
        } else {
            if self.value - self.begin < TIME_EPSILON {
                self.value = self.begin;
                self.done = true;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn take_result(&mut self) -> Option<T> {
        self.result.take()
    }
}

fn update_timers<T>(mut command : Commands, query : Query<(EntityId, &mut Timer<T>)>, game_speed : &GameSpeed)
where T : Sync + Send + 'static + Clone + Copy {
    for (entity, mut timer) in query {
        timer.update(game_speed.relative());
        if timer.done {
            command.trigger(OnTimerFinished::<T> { result: timer.take_result() });
            command.despawn(entity);
        }
    }
}

pub struct TimerPlugin<T>(pub PhantomData<T>);

impl<T> Plugin for TimerPlugin<T>
where T : Sync + Send + 'static + Clone + Copy {
    fn build(&self, app:  &mut crate::app::App) {
        app.add_systems::<F3<Commands, Query<(EntityId, &mut Timer<T>)>, &GameSpeed>>(ScheduleLabel::FixedUpdate, update_timers::<T>);
    }
}