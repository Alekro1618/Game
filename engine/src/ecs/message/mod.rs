pub mod messages;
pub mod message_reader;
mod iterator;
pub mod message_writer;

pub trait Message: Send + Sync + 'static {}

#[derive(Debug)]
pub(crate) struct MessageInstance<M: Message> {
    pub message: M,
}