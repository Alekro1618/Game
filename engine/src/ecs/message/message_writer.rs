use crate::ecs::{
    change_detection::params::Mut,
    message::{Message, messages::Messages},
    systems::function_system::SystemParamTuple,
    worlds::World,
};

pub struct MessageWriter<'w, M: Message> {
    messages: Mut<'w, Messages<M>>,
}

impl<'w, M: Message> MessageWriter<'w, M> {
    pub fn write(&mut self, message: M) {
        self.messages.write(message);
    }

    pub fn from(messages: Mut<'w, Messages<M>>) -> Self {
        Self { messages }
    }
}

impl<M: Message> SystemParamTuple for MessageWriter<'_, M> {
    type State = ();

    type Item<'w> = MessageWriter<'w, M>;

    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<Messages<M>>();
    }

    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        MessageWriter::from(world.get_resource_mut::<Messages<M>>().unwrap())
    }
}
