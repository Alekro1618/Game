use std::collections::VecDeque;
use std::fmt::Debug;
use engine::ecs::{change_detection::params::Mut, entities::id::EntityId, worlds::World};
use engine_macros::Component;

#[allow(unused_variables)]
pub trait Action : Send + Sync + 'static {
    /// If returns true then action is finished after start
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool;

    fn on_end(&mut self, agent : EntityId, world :  &mut World);

    fn on_stop(&mut self, agent : EntityId, world :  &mut World);

    fn on_add(&mut self, agent : EntityId, world :  &mut World) {}

    fn on_remove(&self, agent : EntityId, world :&mut World) {}

    fn is_finished(&self, agent : EntityId, world : &World) -> bool;

    fn name(&self) -> String {
        String::new()
    }
}


pub trait IntoBoxedAction : Send + Sync + 'static {
    fn into_boxed(&self) -> Box<dyn Action>;
}

impl<A: Action + Clone> IntoBoxedAction for A {
    fn into_boxed(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}