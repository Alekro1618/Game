use std::fmt::Display;

use crate::ecs::{bundle::Bundle, components::Component, entities::id::EntityId, event::Event, resource::Resource, systems::commands::command::CommandQueue, worlds::{World, log::print_component_of}};

pub mod command;

pub struct Commands<'w> {
    queue: &'w mut CommandQueue,
}

impl<'w> Commands<'w> {
    pub fn from(queue : &'w mut CommandQueue) -> Self {
        Self {
            queue
        }
    }    

    pub fn spawn<B : Bundle>(&mut self, bundle : B) {
        self.queue.push(move |world| {
            world.spawn(bundle);
        });
    }

    pub fn despawn(&mut self, entity : EntityId) {
         self.queue.push(move |world| {
            world.despawn(entity);
        });
    }

    pub fn insert<R: Resource>(&mut self, resource : R) {
        self.queue.push(move |world | {
            world.insert_resource(resource);
        });
    }

    pub fn add<B : Bundle>(&mut self, entity : EntityId, bundle : B) {
        self.queue.push(move |world | {
            world.insert(entity, bundle);
        });
    }

    pub fn remove<B : Bundle>(&mut self, entity : EntityId) {
        self.queue.push(move |world | {
            world.remove::<B>(entity);
        });
    }

    pub fn trigger<'a, E : Event>(&mut self, mut event: E) {
        self.queue.push(move |world| {
            world.trigger_ref(
                &mut event,
            );
        });
    }

    pub fn queue(&mut self, fun:  impl Fn(&mut World) -> () + Send + Sync + 'static) {
        self.queue.push(move |world | {
            fun(world);
        });
    }

    pub fn log_archetype(&mut self) {
        self.queue.push(move |world | {
            println!("{}", world.archetypes())
        });
    }

     pub fn log<I : Display + Send + Sync + 'static>(&mut self, value : I) {
        self.queue.push(move |world| {
            println!("{}", value)
        });
    }

    pub fn log_component<T : Component + Display>(&mut self, entity : EntityId) {
        self.queue.push(move |world| {
            print_component_of::<T>(world, entity);
        });
    }
}