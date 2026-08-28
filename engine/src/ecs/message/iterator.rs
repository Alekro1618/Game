use std::slice::Iter;

use crate::ecs::message::{Message, MessageInstance};

#[derive(Debug)]
pub struct MessageIterator<'a, M: Message> {
    pub(crate) iter: Iter<'a, MessageInstance<M>>,
}

impl<'a, M: Message> Iterator for MessageIterator<'a, M> {
    type Item = &'a M;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|message| &message.message)
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
        self.iter.last().map(|message| &message.message)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|message| &message.message)
    }
}
