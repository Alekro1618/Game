use crate::ecs::{bundle::Bundle, systems::commands::command::CommandQueue};

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
}