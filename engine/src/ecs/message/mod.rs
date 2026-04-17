pub mod message_registry;
pub mod messages;

pub trait Message: Send + Sync + 'static {}

pub(crate) struct MessageInstance<M: Message> {
    pub message_id: MessageId<M>,
    pub message: M,
}