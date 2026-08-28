use crate::ecs::{
    change_detection::params::Mut,
    message::{Message, drain::MessageDrain, iterator::MessageIterator, messages::Messages},
    systems::function_system::SystemParamTuple,
    worlds::World,
};

pub struct MessageReader<'w, M: Message> {
    messages: Mut<'w, Messages<M>>,
}

impl<'w, M: Message> MessageReader<'w, M> {
    pub fn read(&mut self) -> MessageIterator<'_, M> {
        self.messages.iter()
    }

    pub fn from(messages: Mut<'w, Messages<M>>) -> Self {
        Self { messages }
    }

    pub fn drain(&mut self) -> MessageDrain<'_, M> {
        self.messages.drain()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

impl<M: Message> SystemParamTuple for MessageReader<'_, M> {
    type State = ();

    type Item<'w> = MessageReader<'w, M>;

    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<Messages<M>>();
    }

    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        MessageReader::from(world.get_resource_mut::<Messages<M>>().unwrap())
    }
}
