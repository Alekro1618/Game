use crate::ecs::{components::{Component, Mutable, StorageType}, message::{Message, MessageInstance, iterator::MessageIterator}, resource::Resource};


pub struct Messages<M: Message> {
    pub(crate) messages: Vec<MessageInstance<M>>,
}

impl<M : Message> Component for Messages<M> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

impl<M : Message> Resource for Messages<M> {}

impl<M: Message> Default for Messages<M> {
    fn default() -> Self {
        Self {
            messages: Default::default(),
        }
    }
}

impl<M: Message> Messages<M> {
    #[track_caller]
    pub fn write(&mut self, message: M) {
        self.messages.push(MessageInstance { message });
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns true if there are no messages currently stored in the message buffer.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&mut self) -> MessageIterator<'_, M> {
        MessageIterator {
            iter : self.messages.iter()
        }
    }
    
}