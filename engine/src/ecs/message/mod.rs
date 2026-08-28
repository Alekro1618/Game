use std::marker::PhantomData;

use crate::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, message::messages::Messages}};

mod drain;
mod iterator;
pub mod message_reader;
pub mod message_writer;
pub(crate) mod messages;

pub trait Message: Send + Sync + 'static {}

#[derive(Debug)]
pub(crate) struct MessageInstance<M: Message> {
    pub message: M,
}


pub struct MessagePlugin<M: Message>(PhantomData<M>);

impl<M: Message> Default for MessagePlugin<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M: Message> Plugin for MessagePlugin<M> {
    fn build(self, app: &mut crate::app::App) {
        app.add_systems(
            ScheduleLabel::PostUpdate,
            |mut messages : Mut<Messages<M>>| { messages.clear(); }
        );
    }
}