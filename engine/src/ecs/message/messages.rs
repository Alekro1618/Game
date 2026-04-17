use crate::ecs::message::{Message, MessageInstance};

#[derive(Debug)]
pub struct Messages<M: Message> {
    pub(crate) messages_a: Vec<MessageInstance<M>>,
    pub(crate) messages_b: Vec<MessageInstance<M>>,
    pub(crate) message_count: usize,
}

// Derived Default impl would incorrectly require M: Default
impl<M: Message> Default for Messages<M> {
    fn default() -> Self {
        Self {
            messages_a: Default::default(),
            messages_b: Default::default(),
            message_count: Default::default(),
        }
    }
}

impl<M: Message> Messages<M> {
    /// Returns the index of the oldest message stored in the message buffer.
    pub fn oldest_message_count(&self) -> usize {
        self.messages_a.start_message_count
    }

    /// Writes an `message` to the current message buffer.
    /// [`MessageReader`](super::MessageReader)s can then read the message.
    /// This method returns the [ID](`MessageId`) of the written `message`.
    #[track_caller]
    pub fn write(&mut self, message: M) -> MessageId<M> {
        self.write_with_caller(message, MaybeLocation::caller())
    }

    pub(crate) fn write_with_caller(&mut self, message: M, caller: MaybeLocation) -> MessageId<M> {
        let message_id = MessageId {
            id: self.message_count,
            caller,
            _marker: PhantomData,
        };
        #[cfg(feature = "detailed_trace")]
        tracing::trace!("Messages::write() -> id: {}", message_id);

        let message_instance = MessageInstance {
            message_id,
            message,
        };

        self.messages_b.push(message_instance);
        self.message_count += 1;

        message_id
    }

    /// Writes a list of `messages` all at once, which can later be read by [`MessageReader`](super::MessageReader)s.
    /// This is more efficient than writing each message individually.
    /// This method returns the [IDs](`MessageId`) of the written `messages`.
    #[track_caller]
    pub fn write_batch(&mut self, messages: impl IntoIterator<Item = M>) -> WriteBatchIds<M> {
        let last_count = self.message_count;

        self.extend(messages);

        WriteBatchIds {
            last_count,
            message_count: self.message_count,
            _marker: PhantomData,
        }
    }

    /// Writes the default value of the message. Useful when the message is an empty struct.
    /// This method returns the [ID](`MessageId`) of the written `message`.
    #[track_caller]
    pub fn write_default(&mut self) -> MessageId<M>
    where
        M: Default,
    {
        self.write(Default::default())
    }

    /// Gets a new [`MessageCursor`]. This will include all messages already in the message buffers.
    pub fn get_cursor(&self) -> MessageCursor<M> {
        MessageCursor::default()
    }

    /// Gets a new [`MessageCursor`]. This will ignore all messages already in the message buffers.
    /// It will read all future messages.
    pub fn get_cursor_current(&self) -> MessageCursor<M> {
        MessageCursor {
            last_message_count: self.message_count,
            ..Default::default()
        }
    }

    /// Swaps the message buffers and clears the oldest message buffer. In general, this should be
    /// called once per frame/update.
    ///
    /// If you need access to the messages that were removed, consider using [`Messages::update_drain`].
    pub fn update(&mut self) {
        core::mem::swap(&mut self.messages_a, &mut self.messages_b);
        self.messages_b.clear();
        self.messages_b.start_message_count = self.message_count;
        debug_assert_eq!(
            self.messages_a.start_message_count + self.messages_a.len(),
            self.messages_b.start_message_count
        );
    }

    /// Swaps the message buffers and drains the oldest message buffer, returning an iterator
    /// of all messages that were removed. In general, this should be called once per frame/update.
    ///
    /// If you do not need to take ownership of the removed messages, use [`Messages::update`] instead.
    #[must_use = "If you do not need the returned messages, call .update() instead."]
    pub fn update_drain(&mut self) -> impl Iterator<Item = M> + '_ {
        core::mem::swap(&mut self.messages_a, &mut self.messages_b);
        let iter = self.messages_b.messages.drain(..);
        self.messages_b.start_message_count = self.message_count;
        debug_assert_eq!(
            self.messages_a.start_message_count + self.messages_a.len(),
            self.messages_b.start_message_count
        );

        iter.map(|e| e.message)
    }

    #[inline]
    fn reset_start_message_count(&mut self) {
        self.messages_a.start_message_count = self.message_count;
        self.messages_b.start_message_count = self.message_count;
    }

    /// Removes all messages.
    #[inline]
    pub fn clear(&mut self) {
        self.reset_start_message_count();
        self.messages_a.clear();
        self.messages_b.clear();
    }

    /// Returns the number of messages currently stored in the message buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.messages_a.len() + self.messages_b.len()
    }

    /// Returns true if there are no messages currently stored in the message buffer.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a draining iterator that removes all messages.
    pub fn drain(&mut self) -> impl Iterator<Item = M> + '_ {
        self.reset_start_message_count();

        // Drain the oldest messages first, then the newest
        self.messages_a
            .drain(..)
            .chain(self.messages_b.drain(..))
            .map(|i| i.message)
    }

    /// Iterates over messages that happened since the last "update" call.
    /// WARNING: You probably don't want to use this call. In most cases you should use an
    /// [`MessageReader`]. You should only use this if you know you only need to consume messages
    /// between the last `update()` call and your call to `iter_current_update_messages`.
    /// If messages happen outside that window, they will not be handled. For example, any messages that
    /// happen after this call and before the next `update()` call will be dropped.
    ///
    /// [`MessageReader`]: super::MessageReader
    pub fn iter_current_update_messages(&self) -> impl ExactSizeIterator<Item = &M> {
        self.messages_b.iter().map(|i| &i.message)
    }

    /// Get a specific message by id if it still exists in the messages buffer.
    pub fn get_message(&self, id: usize) -> Option<(&M, MessageId<M>)> {
        if id < self.oldest_message_count() {
            return None;
        }

        let sequence = self.sequence(id);
        let index = id.saturating_sub(sequence.start_message_count);

        sequence
            .get(index)
            .map(|instance| (&instance.message, instance.message_id))
    }

    /// Which message buffer is this message id a part of.
    fn sequence(&self, id: usize) -> &MessageSequence<M> {
        if id < self.messages_b.start_message_count {
            &self.messages_a
        } else {
            &self.messages_b
        }
    }
}
