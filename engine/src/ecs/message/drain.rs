use std::vec::Drain;

use crate::ecs::message::{Message, MessageInstance};

#[derive(Debug)]
pub struct MessageDrain<'a, M: Message> {
    pub(crate) iter: Drain<'a, MessageInstance<M>>,
}

impl<'a, M: Message> Iterator for MessageDrain<'a, M> {
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|message| message.message)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter.last().map(|message| message.message)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|message| message.message)
    }
}
